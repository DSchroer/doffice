# DOffice

Set of simple office suite tools that work on plain text files. 

### Usage

```
doffice 0.1.2
Dominick Schroer <dominick@schroer.ca>
Plain text office suite

USAGE:
    doffice <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    calc    Process CSV file
    doc     Process markdown document
    help    Print this message or the help of the given subcommand(s)
    show    Create slides from markdown
```

## Doc

Convert Markdown files to HTML.

### Usage

```
doffice-doc 
Process markdown document

USAGE:
    doffice doc <FILE>

ARGS:
    <FILE>    

OPTIONS:
    -h, --help    Print help information
```

## Show

Create presentation from markdown file. Split up your slides using `<!-- slide -->`.

### Usage

```
doffice-show 
Create slides from markdown

USAGE:
    doffice show [OPTIONS] <FILE>

ARGS:
    <FILE>    

OPTIONS:
    -h, --help             Print help information
    -t, --theme <THEME>    Theme to use for the presentation [white, black] [default: white]
```

### Features

- use the `csv` code type to have Calc replace formulas in your presentation

## Calc

Process CSV files with formulas. Following Excel style formulas it will read a CSV file and output a new CSV with all the values computed.

### Usage

```
doffice-calc 
Process CSV file

USAGE:
    doffice calc <FILE>

ARGS:
    <FILE>    

OPTIONS:
    -h, --help    Print help information
```

### Function Support
- SUM
- COUNT
- AVERAGE
