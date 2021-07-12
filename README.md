# Reason is for Research

`reason` is an interactive research paper management tool for the commandline.

Invoking `reason` will start a new command prompt. It accepts unix-like commands that work on research papers in your knowledge base.

For instance:
- `ls` filters and prints papers in table format. Default columns are title, nickname, first author, venue, and year.
- `sort` sorts papers by given columns.
- `cd` adds an AND filter to the default set of filters (which is empty upon startup).
- `pwd` shows the current scope set by `cd`.
- `touch` creates a new entry or updates an existing entry in your knowledge base. It will open your editor (defaulting to `vim` but abiding by `$EDITOR`), in which you can edit your notes.
- `rm` removes an entry from your knowledge base.
- `set` sets attributes of papers.
- `head` prints the abstract of papers. It can also print more if configured to do so.
- `stat` prints the metadata and notes of papers.
- `printf` creates an HTML page of your note using pandoc.
- `open` opens the paper with Zathura.
- `top` prints out a summary of your knowledge base.
- `exit` or Ctrl-d quits `reason`.

`ls` in action:
```bash
$ reason
> ls
# all papers
> ls shadowtutor
# papers with 'shadowtutor' in its title
> ls * by Chung at icpp on 2020
# papers with 'Chung' in the name of at least one author, published at icpp on the year 2020
```

## Implementation

- `rustylines`: Used to receive user input and provide completions for paper titles, names, and tags.
- `serde` and `serde-yaml`: Use yaml to store paper metadata.
- `mdbook`: Used to render and open notes.



struct App {
  state,
  config,
}

impl App {
  fn run_command();
}

// Allowed commands and their chaining relationships,
ls -> {rm, printf, sort, set, open}
touch
rm
top
exit
printf
