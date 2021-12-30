Usage: curl [source url]

Manually downloading the PDF and entering relevant metadata
with `touch` might be a bit painstaking. `curl` can download
paper PDFs from sources and automatically populate metadata.

You may populate additional metadata fields (e.g. nickname)
later using the `set` command.

Currently, two sources are supported: arXiv and usenix.org.
[source url] must begin with 'http'.

## arXiv

Usage example:
`curl https://arxiv.org/abs/2003.10735`
`curl https://arxiv.org/pdf/2003.10735.pdf`

`reason` will visit the url and fetch the title and author
list. The venue will be set to arXiv, and the year will be
parsed from the arXiv identifier (20[YY]).

The PDF file will be download from the URL
`https://arxiv.org/pdf/[YYNN.NNNNN].pdf` and saved in the
`storage.file_base_dir` directory.

## usenix

Usage example:
`curl https://usenix.org/conference/atc21/presentation/lee`

`reason` will visit the url and fetch the title and author
list.The venue will be parsed from the name of the conference
in the url. For instance, 

The PDF file link will be parsed from HTML and the file will
be downloaded into the `storage.file_base_dir` directory.
At times, USENIX conferences provide multiple versions of the
paper (e.g., preprint and final). If so, `reason` will prompt
the user to choose one.

# Raw PDF

Usage example:
`curl 
