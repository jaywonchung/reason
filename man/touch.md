Usage: touch [paper]

Adds a new paper to the paperbase. For more information
on how papers are specified in [paper], see `man paper`.
Required fields are 'title', 'authors(by)', 'venue(at)',
and 'year(in)'.

When specifying authors(by) and labels(is), use a single
comma-separated list.

For instance:
```
>> touch 'Reason: A Cool New System' by 'Jae-Won
Chung, Chaehyun Jeong' at OSDI in 2022 as Reason
@ reason.pdf
```
