# softies
A collection of easier-to-use versions of classic POSIX tools.

This collection is inspired by
[a Mastodon thread](https://social.jvns.ca/@b0rk/110657425256191872) wherein
[Julia Evans](https://jvns.ca/) was surveying people about classic CLI tools
they thought had poor UI. The central idea here is to simplify and make the
most common use cases obvious (or at least easy to remember), a sort of
80-20 (or 90-10) Pareto cut-off that sacrifices complexity and the long
tail of rarely-used features for simplicity and discoverability. The
anti-archetype for this repository is
[`man find`](https://man7.org/linux/man-pages/man1/find.1.html).
This project is a success if

```
$ tool_name -h
```

and a little experimentation is all you need to figure out what to do.
If you find any of these tools insufficient, well, you know where to
[find `find`](https://www.gnu.org/software/findutils/manual/html_mono/find.html).

It will probably grow over time (and already includes a tool that I
don't think exists, but should).

So far we have:

  * [`fine`](./fine): A `find` replacement
  * [`fresh`](./fresh): A `sed` replacement
  * [`yargs`](./yargs): An `xargs` replacement
  * [`zipper`](./zipper): A tool for interleaving the output of multiple concurrent
    processes; the process-level version of a
    [zipper](https://en.wikipedia.org/wiki/Zipping_(computer_science))-style
    iterator from functional programming.

I'm open to suggestions, both about new tools or improvements on the
current versions; feel free to open an issue.