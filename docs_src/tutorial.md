# Getting Started

## Installation

Currently the easiest way to install doffice is via [Cargo](https://doc.rust-lang.org/cargo/).

```sh
cargo install --git https://github.com/DSchroer/doffice.gits
```

After that you should have `doffice` available in your command line.

DOffice has three main commands:
- [show](#show): slideshow generation
- [docs](#docs): document generation
- [calc](#calc): spreadsheet engine


## Show

`Show` is the doffice presentation tool. It allows you to make re-usable slideshows from a markdown file. 

The key to making a presentation is to add the magic comment `<!-- slide -->` where the slide gaps should be located.

```md

# This is the first slide

<!-- slide -->

# This is the second slide

```

### Advanced

The slide rendering engine that is used by Show is [Reveal.JS](https://revealjs.com/). Most tricks from reveal will work within Show.

## Docs

`Docs` is a document renderer. It supports the majority of 

## Calc