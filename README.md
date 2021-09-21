<div align="center">
<h1>Reason: A Shell for Research Papers</h1>
</div>

- Did I ever read this paper?
- Which OSDI 2021 papers did I read?
- Which ones have the word 'Distributed' in their titles?
- How many papers in 2020 were co-authored by Professor Mosharaf Chowdhury?

Well, ask `reason`.

## How it works

### Filter and list papers

```
$ reason
>> # Show all papers.
>> ls
+----------------------------------------------------------+----------------+---------+------+
|                           title                          |  first author  |  venue  | year |
+============================================================================================+
| Shadowtutor: Distributed Partial Distillation for Mobile | Jae-Won Chung  | ICPP    | 2020 |
| Video DNN Inference                                      |                |         |      |
|----------------------------------------------------------+----------------+---------+------|
| Efficient Memory Disaggregation with Infiniswap          | Juncheng Gu    | NSDI    | 2017 |
|----------------------------------------------------------+----------------+---------+------|
| Refurbish Your Training Data: Reusing Partially          | Gyewon Lee     | ATC     | 2021 |
| Augmented Samples for Faster Deep Neural Network         |                |         |      |
| Training                                                 |                |         |      |
|----------------------------------------------------------+----------------+---------+------|
| Finding Consensus Bugs in Etherium via Multi-transaction | Youngseok Yang | OSDI    | 2021 |
| Differential Fuzzing                                     |                |         |      |
|----------------------------------------------------------+----------------+---------+------|
| Tiresias: A GPU Cluster Manager for Distributed Deep     | Juncheng Gu    | NSDI    | 2019 |
| Learning                                                 |                |         |      |
|----------------------------------------------------------+----------------+---------+------|
| Nimble: Lightweight and Parallel GPU Task Scheduling for | Woosuk Kwon    | NeurIPS | 2020 |
| Deep Learning                                            |                |         |      |
+----------------------------------------------------------+----------------+---------+------+
>> # Filter by 'title'. All these are regexes!
>> ls 'Deep Learning$'
+------------------------------------------------------------+--------------+---------+------+
|                            title                           | first author |  venue  | year |
+============================================================================================+
| Tiresias: A GPU Cluster Manager for Distributed Deep       | Juncheng Gu  | NSDI    | 2019 |
| Learning                                                   |              |         |      |
|------------------------------------------------------------+--------------+---------+------|
| Nimble: Lightweight and Parallel GPU Task Scheduling for   | Woosuk Kwon  | NeurIPS | 2020 |
| Deep Learning                                              |              |         |      |
+------------------------------------------------------------+--------------+---------+------+
>> # You may set default filters with `cd`.
>> # BTW, `cd .`, `cd ..`, `cd -`, and `cd` are supported, too.
>> cd 'Deep Learning$'
>> pwd
title matches 'Deep Learning$'
>> # Default filter are automatically applied.
>> # Infiniswap (NSDI'17) is not shown, because its title doesn't match 'Deep Learning$'.
>> ls at NSDI
+------------------------------------------------------------+--------------+---------+------+
|                            title                           | first author |  venue  | year |
+============================================================================================+
| Tiresias: A GPU Cluster Manager for Distributed Deep       | Juncheng Gu  | NSDI    | 2019 |
| Learning                                                   |              |         |      |
+------------------------------------------------------------+--------------+---------+------+
>> # Delete Tiresias.
>> ls at NSDI | rm
Removed 1 paper.
```

### Import new papers

```
>> # Import directly from arXiv and USENIX. This will also download paper PDFs.
>> curl https://arxiv.org/abs/2105.11367
+--------------------------------------------------------+--------------+-------+------+
|                          title                         | first author | venue | year |
+======================================================================================+
| FedScale: Benchmarking Model and System Performance of | Fan Lai      | arXiv | 2021 |
| Federated Learning                                     |              |       |      |
+--------------------------------------------------------+--------------+-------+------+
>> curl https://www.usenix.org/conference/nsdi21/presentation/you
+------------------------------------------+--------------+-------+------+
|                   title                  | first author | venue | year |
+========================================================================+
| Ship Compute or Ship Data? Why Not Both? | Jie You      | NSDI  | 2021 |
+------------------------------------------+--------------+-------+------+
>> # Modify paper metadata.
>> ls ship | set as Kayak
>> # Or, import manually.
>> touch 'Batch Normalization: Accelerating Deep Network Training by Reducing Internal
Covariate Shift' by 'Sergey Ioffe, Christian Szegedy' at ICML in 2015 as BN @ BatchNorm.pdf
+--------------------------------------------------------------+--------------+-------+------+
|                             title                            | first author | venue | year |
+============================================================================================+
| Batch Normalization: Accelerating Deep Network Training by   | Sergey Ioffe | ICML  | 2015 |
| Reducing Internal Covariate Shift                            |              |       |      |
+--------------------------------------------------------------+--------------+-------+------+
```

### Read, take notes, and create books!

```
>> # Open with a PDF viewer (`open`) and edit markdown notes with your editor (`ed`).
>> ls 'Why Not Both' | open | ed
+------------------------------------------+--------------+-------+------+
|                   title                  | first author | venue | year |
+========================================================================+
| Ship Compute or Ship Data? Why Not Both? | Jie You      | NSDI  | 2021 |
+------------------------------------------+--------------+-------+------+
>> # Format your markdown notes into an HTML book, and open it in your browser.
>> ls 'Deep Learning' | printf
+------------------------------------------------------------+--------------+---------+------+
|                            title                           | first author |  venue  | year |
+============================================================================================+
| Tiresias: A GPU Cluster Manager for Distributed Deep       | Juncheng Gu  | NSDI    | 2019 |
| Learning                                                   |              |         |      |
|------------------------------------------------------------+--------------+---------+------|
| Nimble: Lightweight and Parallel GPU Task Scheduling for   | Woosuk Kwon  | NeurIPS | 2020 |
| Deep Learning                                              |              |         |      |
+------------------------------------------------------------+--------------+---------+------+
```

## `printf`

`printf` will create an HTML book from your notes.

![printf](/examples/printf.png)

## Commands

Invoking `reason` will start a new command prompt. It accepts unix-like commands that instead work on research papers in your paperbase.

Works now:
- `ls` filters and prints papers in table format. Default columns are title, first author(by1), venue(at), and year(in).
- `cd` adds an AND filter to the default set of filters (which is empty upon startup).
- `pwd` shows the current default filter set by `cd`.
- `touch` creates a new entry in your paperbase.
- `curl` imports papers from the web, e.g. arXiv or usenix.org. It also downloads paper PDFs if available.
- `rm` removes entries from your paperbase.
- `set` sets attributes of papers.
- `printf` creates an HTML page of your notes using `mdbook`.
- `open` opens the paper with your PDF viewer (configurable, defaults to zathura).
- `ed` opens your editor (configurable, defaults to vim), in which you can edit your notes.
- `man` plus a command will print documentation for that command.
- `exit` or Ctrl-d quits `reason`.

Not yet, but hopefully soon:
- `grep` returns a list of papers whose notes contain the query string that you specify.
- `sort` sorts papers by given columns.
- `stat` prints the metadata and notes of papers.
- `top` prints out a summary of your paperbase.
- `sync` stores the paper metadata state to disk. (Changes are kept in memory and are committed to disk on exit.)

## Installation

You can grab binaries from Releases, or you can run `cargo install reason-shell`.


## Cross-Platform Support

`reason` currently supports Linux and MacOS. Windows is excluded because the owner does not currently own a Windows machine.

In order to share data between multiple platforms, users are encouraged to locate `reason` metadata, PDF files, and markdown notes in a location synced by cloud storage services such as Google Drive. I use the official Google Drive app on MacOS and Insync on Linux. This offers an extra benefit - you can read PDFs with your iPad, also synced with the cloud storage.

## Documentation

If you already have reason, run `man man` to view the top-level documentation.

If you're just exploring whether to use reason, take a look at the [`man`](/man) directory.

## Configuration

The configuration file is kept at `~/.config/reason/config.toml`. If not present, `reason` will generate one populated with default settings.

For more information, open `reason` and run `man config`, or read [`man/config.md`](/man/config.md).
