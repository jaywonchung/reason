The Reason command line.

## Shell-like experience

Reason provides a real shell-like experience. You can press
- <Up> to load the previous command in history
- <Down> to load the next command in history
- <Ctrl-a> to move the cursor to the beginning of the line
- <Ctrl-e> to move the cursor to the end of the line
- <Ctrl-l> to clear the screen
- <Ctrl-u> to kill the entire line
and more!

## Running commands

Here are the basic rules by which commands are parsed:
- Whitespaces separate arguments.
  Ex) `ls shadowtutor` consists of two arguments.
- Pipes separate commands. Papers are passed to the next command.
  Ex) `ls shadowtutor | rm` will remove all papers whose title
  matches 'shadowtutor'.
- Single-quote your commands to escape from the above rules.
  Ex) `ls 'shadow | tutor'` (still) consists of two arguments.

## Piping commands

As seen above, commands can be chained with pipes. When a
previous command produces a list of papers, for instance `ls`,
the list of papers can be passed to the next command for
further processing.

For instance, `ls shadowtutor | open` will open all papers
that have the word 'shadwotutor' in their titles.

Most commands output a paper list - all those whose outputs
are displayed as tables: `ls`, `open`, `ed`, and `touch`.
