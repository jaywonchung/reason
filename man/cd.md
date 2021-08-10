Usage: cd [filter], cd [relative]

## Default filters

`cd [filter]` adds the specified filter to the default
filter. The default filter is automatically applied to
all invocations of `ls`.

You can add more filters to the default filter set by
running `cd [filter]` more than one time. All those
filters are AND'ed together, meaning that a paper needs
to successfully match all the default filters in order
to be displayed.

You may query the current default filter with `pwd`.

See `man filter` for more on filters.

## Traversing filter history

The following is supported:
- `cd`   : Clear the default filter set.
- `cd .` : Retain the default filter set.
- `cd ..`: Change the default filter to its parent.
- `cd -` : Change the default filter to what it was before
           the previous invocation of `cd`.

Perhaps this is better illustrated with an example.
```
>> cd by Jeong
>> pwd
author matches 'Jeong'
>> cd 'Janus|Nimble'
>> pwd
title matches 'Janus|Nimble', author matches 'Jeong'
>> cd ..
>> pwd
author matches 'Jeong'
>> cd -
>> pwd
title matches 'Janus|Nimble', author matches 'Jeong'
>> cd
>> pwd
No filters are active.
```
