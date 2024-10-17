# Introduction

This is meant to be a project created to practice with Rust. It's also a simple
`find` command version called `koo` using regular expressions instead of
wildcards syntax.

    Usage: koo [OPTIONS] --pattern <PATTERN> [DIR]

    Arguments:
      [DIR]  Search directory [default: .]

    Options:
      -p, --pattern <PATTERN>  Regex pattern used to filter file names
      -f, --filter <FILTER>    Filter type [default: any] [possible values: any, text, symlink, device, folder]
      -h, --help               Print help
      -V, --version            Print version

