# fine

```text
A more forgiving version of the `find` utility; it works just fine.

Usage: fine [OPTIONS] [PATTERN]...

Arguments:
  [PATTERN]...  The pattern(s) to match file paths against

Options:
  -d, --dir <BASE>         Base directory in which to search [default: .]
  -r, --regex              Use regex (instead of glob) matching
  -f, --full               Match any part of the path, not just the filename
  -t, --type <TYPE>        Match only against specified types [default is all]
      --mod-after <START>  Match only files modified more recently than <START>
      --mod-before <END>   Match only files last modified before <END>
  -a, --absolute           Print absolute paths. [default: relative to BASE]
  -e, --errors             Show access errors (default is to ignore them)
  -h, --help               Print help
  -V, --version            Print version
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

You can limit your matches to entries that were last modified in a specific
time frame by using `--mod-after` and `--mod-before`.

```text
dan@lauDANum:~/dev/softies/fine$ /bin/ls -l src
total 20
-rw-r--r-- 1 dan dan 3762 Jul 28 17:42 main.rs
-rw-r--r-- 1 dan dan 4142 Jul 28 17:58 opt.rs
-rw-r--r-- 1 dan dan 2047 Jul 28 17:56 times.rs
-rw-r--r-- 1 dan dan 3671 Jul 27 23:01 types.rs
dan@lauDANum:~/dev/softies/fine$ fine -d src *.rs --mod-after "2023-07-28 17:50"
src/opt.rs
src/times.rs
```

If you omit the time, it'll assume midnight; if you omit the day, it'll assume
the current day. You can also use both to constrain your time interval at
both ends; it'll warn you if you screw it up.

```text
dan@lauDANum:~/dev/softies/fine$ fine -d src *.rs --mod-before 15:00 --mod-after 15:30
--mod-after must be earlier than --mod-before to get any results
```

(Time must be in 24-hour format. Dates can be in either `2021-01-27` ISO
format, or `1/7/2021` North American format, but _not_ the `27/1/2021`
European format; the latter isn't, in general, possible to disambiguate
from the North American format, and you Europeans are smart enough to figure
it out.)

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

See? Great.