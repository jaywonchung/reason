<div align="center">
<h1>Reason: A Shell for Research Papers</h1>
</div>

- Did I ever read this paper?
- Which OSDI 2021 papers did I read?
- Which ones have the word 'Distributed' in their title?
- How many papers in 2020 were co-authored by Professor Byung-Gon Chun?

Well, ask `reason`.

## How it works

```bash
$ reason
>> ls
+----------------------------------------------------------+----------------+---------+------+
|                           title                          |  first author  |  venue  | year |
+============================================================================================+
| Shadowtutor: Distributed Partial Distillation for Mobile | Jae-Won Chung  | ICPP    | 2020 |
| Video DNN Inference                                      |                |         |      |
|----------------------------------------------------------+----------------+---------+------|
| CloneCloud: Elastic Execution Between Mobile Device and  | Byung-Gon Chun | EuroSys | 2011 |
| Cloud                                                    |                |         |      |
|----------------------------------------------------------+----------------+---------+------|
| Efficient Memory Disaggregation with Infiniswap          | Juncheng Gu    | NSDI    | 2017 |
|----------------------------------------------------------+----------------+---------+------|
| WindTunnel: Towards Differentiable ML Pipelines Beyond a | Gyeong-In Yu   | VLDB    | 2022 |
| Single Model                                             |                |         |      |
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
>> ls 'Deep Learning'
+------------------------------------------------------------+--------------+---------+------+
|                            title                           | first author |  venue  | year |
+============================================================================================+
| Tiresias: A GPU Cluster Manager for Distributed Deep       | Juncheng Gu  | NSDI    | 2019 |
| Learning                                                   |              |         |      |
|------------------------------------------------------------+--------------+---------+------|
| Nimble: Lightweight and Parallel GPU Task Scheduling for   | Woosuk Kwon  | NeurIPS | 2020 |
| Deep Learning                                              |              |         |      |
+------------------------------------------------------------+--------------+---------+------+
>> cd 'Deep Learning'
>> pwd
title matches 'Deep Learning'
>> ls
+------------------------------------------------------------+--------------+---------+------+
|                            title                           | first author |  venue  | year |
+============================================================================================+
| Tiresias: A GPU Cluster Manager for Distributed Deep       | Juncheng Gu  | NSDI    | 2019 |
| Learning                                                   |              |         |      |
|------------------------------------------------------------+--------------+---------+------|
| Nimble: Lightweight and Parallel GPU Task Scheduling for   | Woosuk Kwon  | NeurIPS | 2020 |
| Deep Learning                                              |              |         |      |
+------------------------------------------------------------+--------------+---------+------+
>> # Delete 'Tiresias'.
>> ls at NSDI | rm
>> ls ..
+----------------------------------------------------------+----------------+---------+------+
|                           title                          |  first author  |  venue  | year |
+============================================================================================+
| Shadowtutor: Distributed Partial Distillation for Mobile | Jae-Won Chung  | ICPP    | 2020 |
| Video DNN Inference                                      |                |         |      |
|----------------------------------------------------------+----------------+---------+------|
| CloneCloud: Elastic Execution Between Mobile Device and  | Byung-Gon Chun | EuroSys | 2011 |
| Cloud                                                    |                |         |      |
|----------------------------------------------------------+----------------+---------+------|
| Efficient Memory Disaggregation with Infiniswap          | Juncheng Gu    | NSDI    | 2017 |
|----------------------------------------------------------+----------------+---------+------|
| WindTunnel: Towards Differentiable ML Pipelines Beyond a | Gyeong-In Yu   | VLDB    | 2022 |
| Single Model                                             |                |         |      |
|----------------------------------------------------------+----------------+---------+------|
| Refurbish Your Training Data: Reusing Partially          | Gyewon Lee     | ATC     | 2021 |
| Augmented Samples for Faster Deep Neural Network         |                |         |      |
| Training                                                 |                |         |      |
|----------------------------------------------------------+----------------+---------+------|
| Finding Consensus Bugs in Etherium via Multi-transaction | Youngseok Yang | OSDI    | 2021 |
| Differential Fuzzing                                     |                |         |      |
|----------------------------------------------------------+----------------+---------+------|
| Nimble: Lightweight and Parallel GPU Task Scheduling for | Woosuk Kwon    | NeurIPS | 2020 |
| Deep Learning                                            |                |         |      |
+----------------------------------------------------------+----------------+---------+------+
>> touch 'Hippo: Taming Hyper-parameter Optimization of Deep Learning with Stage Trees'
by 'Ahnjae Shin, Do Yoon Kim, Joo Seong Jeong, Byung-Gon Chun' at arXiv in 2020 as Hippo
+--------------------------------------------------------------+--------------+-------+------+
|                             title                            | first author | venue | year |
+============================================================================================+
| Hippo: Taming Hyper-parameter Optimization of Deep Learning  | Ahnjae Shin  | arXiv | 2020 |
| with Stage Trees                                             |              |       |      |
+--------------------------------------------------------------+--------------+-------+------+
```

Invoking `reason` will start a new command prompt. It accepts unix-like commands that instead work on research papers in your paperbase.

Many commands will become available over time:
- `ls` filters and prints papers in table format. Default columns are title, first author(by1), venue(at), and year(in).
- `cd` adds an AND filter to the default set of filters (which is empty upon startup).
- `pwd` shows the current default filter set by `cd`.
- `touch` creates a new entry in your paperbase.
- `rm` removes entries from your paperbase.
- `sort` sorts papers by given columns.
- `set` sets attributes of papers.
- `stat` prints the metadata and notes of papers.
- `printf` creates an HTML page of your notes using `mdbook`.
- `open` opens the paper with Zathura.
- `read` opens the paper with Zathura and also your editor (defaulting to `vim` but abiding by `$EDITOR`), in which you can edit your notes.
- `top` prints out a summary of your paperbase.
- `sync` stores the paper metadata state to disk.
- `man` plus a command will print documentation for that command.
- `exit` or Ctrl-d quits `reason`.

## Configuration

The configuration file is kept at `~/.config/reason/config.toml`. If not present, `reason` will generate one populated with default settings.

For more information, open `reason` and run `man config`.

## Todo

Shell-like experience
- [x] Run commands.
- [x] Support pipes between commands. A command passes a list of papers to the next command.
- [x] GNU Readline features (up arrow, down arrow, Ctrl-A, Ctrl-E, Ctrl-L, etc).

Configuration
- [x] Allowing configuration.
- [ ] Tweaking table appearance.
- [ ] Regex-related (?)

Paper metadata
- [ ] Support tags or labels (with keyword 'is'?)

Commands
- [x] `ls`
- [x] `cd`
- [x] `pwd`
- [x] `touch`
- [x] `exit`
- [x] `rm`
- [ ] `sort`
- [ ] `set`
- [ ] `stat`
- [ ] `printf`
- [x] `open`
- [ ] `read`
- [ ] `top`
- [ ] `sync`
- [x] `man`
- [x] `exit`
