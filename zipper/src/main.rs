mod cmds;
mod opt;

use std::{error::Error, io::{stdin, stdout, BufRead, Write}};

use clap::ValueEnum;
use tokio::{runtime, sync::mpsc::Receiver, task::LocalSet};

use cmds::{Cmd, Threading};
use opt::Opts;

#[cfg(unix)]
static NEWLINE: &[u8] = &[10];
#[cfg(windows)]
static NEWLINE: &[u8] = &[13, 10];

static PREFIX_OPTION_HELP: &str = r#"
[OPTIONS || ] COMMAND_LINE

Options:
  -s, --skip <N>
          Skip the first <N> lines of output.

  -t, --take <N>
          Use only <N> lines of output.

  -d, --delimiter <REGEX>
          Use <REGEX> as a delimiter (instead of \r?\n).
"#;

/// Controls how zipper behaves when commands terminate after
/// different amounts of output.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ValueEnum)]
pub enum Finished {
    /// Stop when first command terminates.
    #[default]
    Terminate,
    /// Ignore terminated commands.
    Ignore,
    /// Insert blank lines for commands that have terminated.
    Blank,
}

/**
Read from stdin until EOF or a blank line, and return a Vec of
`Cmd` structs to run.
*/
fn get_commands() -> Result<Vec<Cmd>, String> {
    let mut v: Vec<Cmd> = Vec::new();
    for (n, line_res) in stdin().lock().lines().enumerate() {
        let line = line_res.map_err(|e| format!("error reading line {} from stdin: {}", &n, &e))?;

        if line.trim() == "" {
            break;
        } else {
            let cmd = Cmd::from_line(&line)
                .map_err(|e| format!("error parsing command from line {}: {}", &n, &e))?;
            v.push(cmd);
        }
    }

    Ok(v)
}

/**
Repeatedly iterate through the output channels of the commands,
writing lines from each to stdout.
*/
async fn read_loop(mut outputs: Vec<Receiver<Vec<u8>>>, finished: Finished) -> std::io::Result<()> {
    let mut buff: Vec<u8> = Vec::new();
    let mut dones: Vec<bool> = vec![false; outputs.len()];
    let exit_condition: Vec<bool> = vec![true; outputs.len()];

    'writeloop: while &dones != &exit_condition {
        for (n, rx) in outputs.iter_mut().enumerate() {
            match rx.recv().await {
                Some(v) => {
                    buff.write_all(&v)?;
                    buff.write_all(NEWLINE)?;
                }
                None => {
                    dones[n] = true;
                    match finished {
                        Finished::Terminate => break 'writeloop,
                        Finished::Ignore => { /* do nothing */ }
                        Finished::Blank => {
                            buff.write_all(NEWLINE)?;
                        }
                    }
                }
            }
        }
        let mut stdout = stdout().lock();
        stdout.write_all(&buff)?;
        stdout.flush()?;
        buff.clear();
    }

    Ok(())
}

/**
Repeatedly iterate through the output channels of the commands.
The outputs of the individual commands in each iteration will be
separated by `sep`; the total output of each iteration will be
separated by a newline.
*/
async fn paste_mode_read(
    mut outputs: Vec<Receiver<Vec<u8>>>,
    finished: Finished,
    sep: &[u8],
) -> std::io::Result<()> {
    let mut buff: Vec<u8> = Vec::new();
    let mut dones: Vec<bool> = vec![false; outputs.len()];
    let exit_condition: Vec<bool> = vec![true; outputs.len()];
    let n_outputs = outputs.len();

    'writeloop: while &dones != &exit_condition {
        for (n, rx) in outputs.iter_mut().enumerate() {
            match rx.recv().await {
                Some(v) => {
                    buff.extend_from_slice(&v);
                    if n + 1 < n_outputs {
                        buff.extend_from_slice(sep);
                    }
                },
                None => {
                    dones[n] = true;
                    match finished {
                        Finished::Terminate => break 'writeloop,
                        Finished::Ignore => { /* do nothing */ },
                        Finished::Blank => {
                            buff.push(b' ');
                            if n + 1 < n_outputs {
                                buff.extend_from_slice(sep);
                            }
                        }
                    }
                }
            }
        }
        buff.extend_from_slice(NEWLINE);
        let mut stdout = stdout().lock();
        stdout.write_all(&buff)?;
        stdout.flush()?;
        buff.clear();
    }

    Ok(())
}

/// Spawn commands and interleave their output using tokio's default
/// (multh-threaded) task scheduler.
fn run_threaded(cmds: Vec<Cmd>, opts: Opts) -> Result<(), Box<dyn Error>> {
    let rt = runtime::Builder::new_multi_thread()
        .enable_io()
        .build()?;
    
    rt.block_on(async {
        let outputs: Vec<_> = cmds.into_iter()
            .enumerate()
            .filter_map(|(n, cmd)| match cmd.spawn(Threading::Multi) {
                Ok(rx) => Some(rx),
                Err(e) => {
                    eprintln!("error spawning process {} {:?}: {}", &n, &cmd, &e);
                    None
                },
            }).collect();

        if let Some(ref sep) = opts.paste {
            paste_mode_read(outputs, opts.exit, sep.as_bytes()).await
        } else {
            read_loop(outputs, opts.exit).await
        }
    }).map_err(|e| e.into())
}

/// Spawn commands and interleave their output using tokio's single-threaded
/// task scheduler. This does not guarantee strict single-threaded behavior.
/// Even if it did, each command still needs to run in its own process.
fn run_local(cmds: Vec<Cmd>, opts: Opts) -> Result<(), Box<dyn Error>> {
    let rt = runtime::Builder::new_current_thread()
        .enable_io()
        .build()?;
    
    rt.block_on(async {
        let local = LocalSet::new();

        local.run_until(async move {
            let outputs: Vec<_> = cmds.into_iter()
                .enumerate()
                .filter_map(|(n, cmd)| match cmd.spawn(Threading::Local) {
                    Ok(rx) => Some(rx),
                    Err(e) => {
                        eprintln!("error spawning process {} {:?}: {}", &n, &cmd, &e);
                        None
                    },
                }).collect();

            if let Some(ref sep) = opts.paste {
                paste_mode_read(outputs, opts.exit, sep.as_bytes()).await
            } else {
                read_loop(outputs, opts.exit).await
            }
        }).await
    }).map_err(|e| e.into())
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::get()?;
    if opts.commands {
        println!("{}", PREFIX_OPTION_HELP);
        std::process::exit(0);
    }
    let cmds = get_commands().unwrap();

    if opts.threads {
        run_threaded(cmds, opts)
    } else {
        run_local(cmds, opts)
    }
}
