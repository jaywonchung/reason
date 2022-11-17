Usage:
1) alone: wc [filter]
2) pipe:  [paper list] | wc

Count the number of papers.

When a paper list is given to `wc` via pipe, all
command line arguments are ignored. On the other hand,
if nothing is given through pipe, `wc` accepts filters
though arguments, and the default filter is also applied.
Thus, `ls | wc` is equivalent to just `wc`.
