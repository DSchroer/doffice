# DOffice

Set of simple office suite tools that work on plain text files. 

### Usage

```
doffice 0.1.1
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

### Function Support:
- SUM
- COUNT
- AVERAGE
