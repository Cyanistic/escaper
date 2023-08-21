# Escaper
## A simple CLI tool to escape and unescape special character sequences.
### Main Uses
I created this to make command line/terminal web scraping easier, especially since I wanted to use [Google Translate](https://translate.google.com/translate_tts?ie=UTF-8&client=tw-ob&q=READER&tl=En-us) as a tts reader.
### Usage
```
Escapes (or unescapes) special character sequences and prints the result to stdout.

Usage: escaper [OPTIONS] [SEQUENCES...]

OPTIONS:
  -h, --help                    Print the help information and exit.
  -V, --version                 Print version and exit.
   -                            Reads SEQUENCES from stdin.
  --                            Causes all further arguments to be read as SEQUENCES.
  -s, --string                  Uses string ($) escape sequences instead of default URL (%) escape sequences.
  -r, --regex                   Uses shell/regex (\) escape sequences instead of default URL (%) escape sequences.
  -u, --undo                    Unescapes SEQUENCES.
EXCLUSION OPTIONS:
  -a, --apostrophes             Removes apostrophes from escapable character list.
  -n, --new-line                Removes new lines from escapable character list.
  -b, --backticks               Removes backticks from escapable character list.
  -q, --quotes                  Removes quotation marks from escapable character list.
```
### Building
Escaper is a very simple program to build. 
```
cargo build -r
```
