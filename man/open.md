Usage:
1) alone: open [filter]
2) pipe:  [paper list] | open

Open papers with a viewer program and outputs
successfully opened papers in the usual table format.
You may configure the viewer to use by setting the
`output.viewer_command` entry in your config file.

When a paper list is given to `open` via pipe, all
command line arguments are ignored. On the other hand,
if nothing is given through pipe, `open` accepts filters
though arguments, and the default filter is also applied.
Thus, `ls | open` is equivalent to just `open`.

The following might come in handy:
```
ls as Reason | open | ed
```
