Usage:
1) alone: ed [filter]
2) pipe:  [paper list] | ed

Open paper notes with a text editor and outputs
the papers of successfully notes in the usual table format.
You may configure the editor to use by setting the
`output.editor_command` entry in your config file.

When a paper list is given to `ed` via pipe, all
command line arguments are ignored. On the other hand,
if nothing is given through pipe, `ed` accepts filters
though arguments, and the default filter is also applied.
Thus, `ls | ed` is equivalent to just `ed`.

The following might come in handy:
```
ls as Reason | open | ed
```
";
