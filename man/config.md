Reason configuration.

Config file location: '~/.config/reason/config.toml'.
If nothing is there, reason will create one populated
with default settings.

## Storage

- paper_metadata: Path to store paper metadata.
   (default: ~/.local/share/reason/metadata.yaml)
- command_history: Path to store command history.
   (default: ~/.local/share/reason/history.txt)
- max_history_size: How many commands to keep in history.
   (default: 1000)
- file_dir: The base directory where paper files are stored.
  If set, you can just specify the file path relative to
  this directory with 'touch @'.
   (default: ~/.local/share/reason/files)
- note_dir: The directory where markdown notes are stored.
   (default: ~/.local/share/reason/notes)

## Filter

- case_insensitive_regex: Whether filter regexes match
  in a case-insensitive manner.
   (default: false)

## Output

- table_columns: Which paper attributes `ls` shows.
  Allowed values are 'title', 'authors', 'first author',
  'venue', 'year', and 'labels'.
   (default: ['title', 'first author', 'venue', 'year'])
- viewer_command: Command to use for the viewer to open
  papers. It is assumed that the viewer program is a
  non-command line program. If you place a set of curly
  braces ('{}') in the list, the path to the file(s) will
  be substituted in that location. Otherwise, the path(s)
  will be placed at the end.
   (default: ['zathura'])
- viewer_batch: Whether to open multiple papers with a
  single invocation of the viewer command. If true, the
  command ran is: `viewer_command file1 file2 ...`.
  Otherwise, the viewer command is invoked once for each
  paper.
   (default: false)
- editor_command: Command to use for the editor to edit
  notes. It is assumed that the editor is a command line
  program. If you place a set of curly braces ('{}') in
  the list, the path to the file(s) will be substituted
  in that location. Otherwise, the path(s) will be placed
  at the end.
   (default: ['vim', '-p'])
- editor_batch: Whether to open multiple notes with a
  single invocation of the editor command. If true, the
  command ran is: `viewer_command file1 file2 ...`.
  Otherwise, the editor command is invoked once for each
  paper.
   (default: true)
- browser_command: Command to use for the web browser to
  open formatted HTML notes. If you place a set of curly
  braces ('{}') in the list, the path to `index.html`
  will be substituted in that location. Otherwise, the
  path to `index.html` will be placed at the end.
   (default: ['google-chrome-stable']
- label_colors: Papers with specific labels can be displayed
  in colors chosen by the user. By default, if you set the
  label 'done' for a paper (`set is done`), it'll be shown
  in Green, and likewise Yellow for label 'active'.
   (default: { done = 'Green', active = 'Yellow' })
- exclusive_label_groups: Some labels are exclusive to each
  other. For instance, if you want to track reading progress
  with labels 'done' and 'active', you don't want a paper to
  have both of those labels. That is, when you `set is done`
  a paper that had the label 'active', you want the command
  to automatically delete 'active' and insert 'done'. 
   (default: [ [ 'done, 'active' ] ])
