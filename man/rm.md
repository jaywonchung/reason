Usage:
1) alone: rm [filter]
2) pipe:  [paper list] | rm

Remove papers from the paperbase.

When a paper list is given to `rm` via pipe, all
command line arguments are ignored. On the other hand,
if nothing is given through pipe, `rm` accepts filters
though arguments, and the default filter is also applied.
Thus, `ls | rm` is equivalent to just `rm`.
