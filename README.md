# Escaper
## A simple CLI tool to escape and unescape URLs.
### Main Uses
I created this to make command line/terminal web scraping easier, especially since I wanted to use (Google Translate)[https://translate.google.com/translate_tts?ie=UTF-8&client=tw-ob&q=READER&tl=En-us] as a tts reader.
### Usage
```
A program that escapes (or unescapes) special characters for URL sequences
Usage: escaper [OPTIONS] [SEQUENCES]
-h, --help                          Print the help information.
 -                                  Reads SEQUENCES from stdin.
-s, --string                        Reads uses string ($) escape sequences instead of default (%) escape sequences.
-u, --undo                          Unescapes SEQUENCES.
EXCLUSION OPTIONS:
-a, --apostrophes                   Removes apostrophes from escapable character list
-b, --backticks                     Removes backticks from escapable character list
-q, --quotes                        Removes quotation marks from escapable character list
```
### Building
Escaper is a very simple program to build. A simple `cargo build -r` is all.
