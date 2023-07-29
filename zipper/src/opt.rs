/*!
Parsing the command-line options.
*/
use clap::Parser;

use super::Finished;

static DEFAULT_PASTE_SEP: &str = "\t";

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct OptArgs {
    /// Specify behavior on command termination.
    #[arg(short, long, value_enum, default_value_t = Finished::Terminate)]
    exit: Finished,
    /// Use more threads.
    #[arg(short, long, default_value_t = false)]
    threads: bool,
    /// Print the command-line prefix options
    #[arg(short = 'H', long = "command-help", default_value_t = false)]
    commands: bool,
    /// Print each tuple on the same line, separated by
    /// <SEP> [default is \t], like paste(1)
    #[arg(short, long, name = "SEP")]
    paste: Option<Option<String>>,
}

#[derive(Debug, Default)]
pub struct Opts {
    pub exit: Finished,
    pub threads: bool,
    pub commands: bool,
    pub paste: Option<String>,
}

impl Opts {
    pub fn get() -> Result<Opts, String> {
        let oa = OptArgs::parse();
        let mut opts = Opts {
            exit: oa.exit,
            threads: oa.threads,
            commands: oa.commands,
            ..Default::default()
        };

        let paste = match oa.paste {
            None => None,
            Some(None) => Some(String::from(DEFAULT_PASTE_SEP)),
            Some(Some(s)) => Some(s),
        };

        opts.paste = paste;
        Ok(opts)
    }
}