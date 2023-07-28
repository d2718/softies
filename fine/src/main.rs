mod opt;
pub mod types;

use std::{
    error::Error,
    io::{stdout, Write},
    path::Path,
};

use bstr::ByteSlice;
use walkdir::{DirEntry, WalkDir};

use opt::Opts;
use types::HasEType;

static NEWLINE: &[u8] = b"\n";

/// Print a matched path relative to the base search path.
fn print_relative(path: &Path) -> Option<()> {
    let bytes = <[u8]>::from_os_str(path.as_os_str())?;
    let mut stdout = stdout();
    stdout.write_all(bytes).unwrap();
    stdout.write_all(NEWLINE).unwrap();
    Some(())
}

/// Print the absolute form of a matched path.
fn print_absolute(path: &Path) -> Option<()> {
    // Unlikely this will error, so we'll just skip it.
    let abs = path.canonicalize().ok()?;
    let bytes = <[u8]>::from_os_str(abs.as_os_str())?;
    let mut stdout = stdout();
    stdout.write_all(bytes).unwrap();
    stdout.write_all(NEWLINE).unwrap();
    Some(())
}

/// Deterime whether a given filename matches the supplied
/// set of patterns.
fn check_match(opts: &Opts, ent: &DirEntry) -> Option<bool> {
    let path = ent.path();
    let path_to_match = if opts.full {
        path.as_os_str()
    } else {
        path.file_name()?
    };
    let bytes = <[u8]>::from_os_str(path_to_match)?;

    Some(opts.patterns.is_match(bytes))
}

/// Walk the directory tree starting from `opts.base`, checking
/// for and printing paths with matching filenames.
fn walk_and_check(opts: &Opts) -> Result<(), Box<dyn Error>> {
    let meta = std::fs::metadata(&opts.base).map_err(|e| format!(
        "unable to read from \"{}\": {}", &opts.base.display(), &e
    ))?;
    if !meta.is_dir() {
        return Err(format!(
            "\"{}\" is not a directory", &opts.base.display()
        ).into());
    }

    let ok_types = if opts.types.is_empty() {
        None
    } else {
        Some(opts.types.as_slice())
    };
    
    for res in WalkDir::new(&opts.base).follow_links(false) {
        let ent = match (res, opts.errors) {
            (Ok(ent), _) => ent,
            (Err(e), true) => {
                eprintln!("{}", &e);
                continue;
            },
            (Err(_), false) => continue,
        };

        if let Some(types) = ok_types {
            if !ent.file_type().is_one(types) {
                continue;
            }
        }

        match (
            check_match(opts, &ent).unwrap_or_default(),
            opts.absolute,
         ) {
            (true, false) => print_relative(ent.path()),
            (true, true)  => print_absolute(ent.path()),
            _ => continue,
        };
    }

    Ok(())
}

fn wrapped_main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::new()?;
    walk_and_check(&opts)
}

fn main() {
    if let Err(e) = wrapped_main() {
        eprintln!("{}", &e);
        std::process::exit(1);
    }
}
