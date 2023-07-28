# fine

```text
A more forgiving version of find; it works just fine

Usage: fine [OPTIONS] [PATTERN]...

Arguments:
  [PATTERN]...  The pattern(s) to match file paths against

Options:
  -d, --dir <BASE>   Base directory in which to search [default: .]
  -r, --regex        Use regex (instead of glob) matching
  -f, --full         Match any part of the path, not just the filename
  -t, --type <TYPE>  Match only against specified types [default is all]
  -a, --absolute     Print absolute paths. [default: relative to BASE]
  -e, --errors       Show access errors (default is to ignore them)
  -h, --help         Print help
  -V, --version      Print version
```

## Installation

It should just `cargo build --release`.

## Use

By default, `fine` searches in and below the current directory for filenames
that match the supplied set of globs:

```text
dan@lauDANum:~/dev/fine$ fine *.rs
./src/opt.rs
./src/main.rs
```

You can supply multiple patterns:

```text
dan@lauDANum:~/dev/fine$ fine *.rs Cargo.*
./src/opt.rs
./src/main.rs
./Cargo.toml
./Cargo.lock
```

Specify a specific base directory with `-d`:

```text
dan@lauDANum:~/dev/fine$ fine -d /usr/share/fonts terminus*
/usr/share/fonts/opentype/terminus
/usr/share/fonts/opentype/terminus/terminus-bold-oblique.otb
/usr/share/fonts/opentype/terminus/terminus-bold.otb
/usr/share/fonts/opentype/terminus/terminus-oblique.otb
/usr/share/fonts/opentype/terminus/terminus-normal.otb
```

By default, `fine` prints paths relative to the specified base directory.
Force it to print the full path with `-a`:

```text
dan@lauDANum:~/dev/fine$ fine *.rs -a
/home/dan/dev/fine/src/opt.rs
/home/dan/dev/fine/src/main.rs
```

Specify regex patterns instead of globs with `-r`:

```text
dan@lauDANum:~/dev/fine$ fine -d target/debug -r 'clap_.*a[0-2]'
target/debug/deps/clap_lex-a87e359c0a25297d.d
target/debug/deps/clap_builder-8a1806fd13db2c47.d
target/debug/deps/libclap_lex-a87e359c0a25297d.rmeta
target/debug/deps/libclap_builder-8a1806fd13db2c47.rlib
target/debug/deps/libclap_builder-8a1806fd13db2c47.rmeta
target/debug/.fingerprint/clap_lex-a87e359c0a25297d
target/debug/.fingerprint/clap_builder-8a1806fd13db2c47
```

Match only against certain types of directory entries with `-t`:

```text
dan@lauDANum:~/dev/fine$ target/debug/fine -d /dev -t link '*'
/dev/stderr
/dev/stdout
/dev/stdin
/dev/fd
```

You can use `-t` more than once to specify multiple types:

```text
dan@lauDANum:~/dev/fine$ target/debug/fine -d /dev -t dir -t link '*'
/dev
/dev/shm
/dev/pts
/dev/stderr
/dev/stdout
/dev/stdin
/dev/fd
/dev/block
/dev/bsg
/dev/mapper
/dev/bus
/dev/bus/usb
/dev/bus/usb/002
/dev/bus/usb/001
/dev/vfio
/dev/net
/dev/dri
```

Specify an invalid type to get a list of types supported on your platform:

```text
dan@lauDANum:~/dev/fine$ target/debug/fine -t fnord *.*
directory entry type fnord invalid or not supported on this platform
possible values are: file, dir, link, fifo, socket, block, char
```

Match your pattern agains the entire path (instead of just the final
element) with `-p`:

```text
dan@lauDANum:~/dev/fine$ fine -d ~/.config *helix*
/home/dan/.config/helix
dan@lauDANum:~/dev/fine$ fine -d ~/.config -p *helix*
/home/dan/.config/helix
/home/dan/.config/helix/themes
/home/dan/.config/helix/themes/zzd_rose_pine.toml
/home/dan/.config/helix/runtime
/home/dan/.config/helix/languages.toml
/home/dan/.config/helix/config.toml
```

## The Future

  * controlling whether symbolic links should be followed
  * optimization, probably (I've tried to do things in a
    not-obviously-stupid fashion, but otherwise there's none.)
  * more organized error handling

## &c.

This is not meant to be anything like a feature-complete rewrite
of `find(1)`. It's meant to alleviate the need to search out the
Coreutils docs just to use the 80-20 (or maybe 90-10) use case of
finding a file.

```text
dan@lauDANum:~/dev/fine$ info find
bash: info: command not found
```

Great.