Paper metadata.

Reason keeps metadata for each paper in its paperbase.

| field    | keyword | `set` |                   description                  |
|----------|:-------:|:-----:|------------------------------------------------|
| title    |         | yes   | The title of the paper, in full.               |
| nickname | as      | yes   | An arbitrary nickname for the paper.           |
| authors  | by      | yes   | The list of authors, in order.                 |
| venue    | at      | yes   | Where the paper was published, excluding year. |
| year     | in      | yes   | The year when the paper was published.         |
| filepath | @       | yes   | The path to the PDF file of the paper.         |
| labels   | is/not  | yes   | A set of labels assigned to this paper.        |
| state    |         | no    | The management state history. ADDED or READ.   |
| notepath |         | no    | The path to the markdown note file.            |

'filepath' and 'notepath' are specified as relative paths,
each based on `config.storage.file_dir` and
`config.storage.note_dir`. See `man config` for more
information.
