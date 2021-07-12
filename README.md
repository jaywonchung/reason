# Reason is for Research

`reason` is a commandline tool for research paper management.

Invoking `reason` will start a new command prompt. It accepts unix-like commands that instead work on research papers in your paperbase.

For instance:
- `ls` filters and prints papers in table format. Default columns are title, nickname(as), first author(by1), venue(at), and year(in).
- `sort` sorts papers by given columns.
- `cd` adds an AND filter to the default set of filters (which is empty upon startup).
- `pwd` shows the current default filter set by `cd`.
- `touch` creates a new entry in your knowledge base.
- `rm` removes an entry from your knowledge base.
- `set` sets attributes of papers.
- `stat` prints the metadata and notes of papers.
- `printf` creates an HTML page of your note using mdbook.
- `open` opens the paper with Zathura.
- `read` opens the paper with Zathura and also your editor (defaulting to `vim` but abiding by `$EDITOR`), in which you can edit your notes.
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

// Commands
// allowed_positions piped_input -> command(argument_input) -> output
1             ls(filter)         -> table
1             cd(filter)         -> filter
1             pwd()              -> filter
1             touch(filename)    -> table
1             rm(filter)         -> none
1             top()              -> custom
1             exit()             -> !
 2 filter ->  rm()               -> none
1             set(filter, attr)  -> table
 2 filter ->  set(attr)          -> table
1             stat(filter)       -> custom
 2 filter ->  stat()             -> custom
1             printf(filter)     -> exec
 2 filter ->  printf()           -> exec
1             open(filter)       -> exec
 2 filter ->  open()             -> exec
1             read(filter)       -> exec
 2 filter ->  read()             -> exec
