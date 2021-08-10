Usage: touch [paper]

Adds a new paper to the paperbase. Papers can be
specified like filters. Differences are:
- The path to the paper file can be specified with the
   keyword '@'.
- Authors should be specified with a single-quoted
   comma-separated string.

Required fields are 'title', 'authors(by)', 'venue(at)',
and 'year(in)'. Just like filters, they don't have to
be in order.

For instance:
```
>> touch 'Reason: A Cool New System' by 'Jae-Won
Chung, Chaehyun Jeong' at OSDI at 2022 as Reason
@ reason.pdf
```
