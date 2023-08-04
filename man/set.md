Usage: [paper list] | set [paper]

Assign and modify paper metadata. See `man paper` on which
fields can be specified how.

`set` can especially be used to add and remove labels for
papers:
```
# Add the label 'done' to papers whose title match 'zeus'.
# The default configuration will color these papers in Green.
# See `man config` > 'label_colors'.
$ ls zeus | set is done

# Unset the 'active' label for all papers that had 'active'.
$ ls is active | set not active
```
