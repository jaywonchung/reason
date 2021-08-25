Paper filters.

Filters are a collection of **regexes** that match on paper
metadata (See `man paper` for more on paper metadata).
Especially, queryable fields and their corresponding
propositional keywords are:
- title (no keyword)
- nickname (`as`)
- authors (`by`)
- first author (`by1`)
- venue (`at`)
- year (`in`)
- label to include (`is`)
- label to exclude (`not`)

Reason allows users to describe paper filters naturally
using propositional keywords.
For instance:
```
>> cd ^shadowtutor at ICPP
>> pwd
title matches '^shadowtutor', venue matches 'ICPP'
>> cd
>> cd 'Deep Learning' by Chung by Jeong
>> pwd
title matches 'Deep Learning', author matches 'Chung' & 'Jeong'
```
