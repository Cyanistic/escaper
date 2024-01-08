use aho_corasick::AhoCorasick;
use escaper::{consts::*, print_help, read_stdin};
use std::io::{self, stdout, Write};
use std::process::exit;

pub fn main() -> Result<(), io::Error> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut undo = false;
    let mut options_end: Option<usize> = None;
    let mut removed: Vec<&str> = Vec::new();

    // Default to URL escape sequences
    let mut escape_chars = &URL_CHARS;

    // Parse the arguments
    for (k, v) in args.iter().enumerate() {
        let mut chars = v.chars();
        if chars.next().is_some_and(|x| x == '-') && options_end.is_none() {
            match chars.clone().next() {
                Some('-') => {
                    chars.next();
                    match chars.as_str() {
                        "help" => {
                            print_help().expect("Couldn't write to stdout");
                        }
                        "version" => {
                            writeln!(stdout(), "{}", env!("CARGO_PKG_VERSION"))
                                .expect("Couldn't write to stdout");
                            exit(0);
                        }
                        "string" => escape_chars = &STRING_CHARS,
                        "regex" => escape_chars = &REGEX_CHARS,
                        "quotes" => {
                            removed.push("\"");
                        }
                        "backticks" => {
                            removed.push("`");
                        }
                        "apostrophes" => {
                            removed.push("'");
                        }
                        "new-line" => {
                            removed.push("\n");
                        }
                        "undo" => undo = true,
                        "" => {
                            options_end = Some(k);
                        }
                        _ => {
                            eprintln!("Invalid argument: {}", v);
                            exit(1);
                        }
                    }
                }
                None => {
                    options_end = Some(k);
                }
                Some(_) => {
                    for char in chars {
                        match char {
                            'q' => {
                                removed.push("\"");
                            }
                            'b' => {
                                removed.push("`");
                            }
                            'a' => {
                                removed.push("'");
                            }
                            'n' => {
                                removed.push("\n");
                            }
                            'u' => undo = true,
                            'h' => {
                                print_help().expect("Couldn't write to stdout");
                            }
                            'V' => {
                                writeln!(stdout(), "{}", env!("CARGO_PKG_VERSION"))
                                    .expect("Couldn't write to stdout");
                                exit(0);
                            }
                            's' => escape_chars = &STRING_CHARS,
                            'r' => escape_chars = &REGEX_CHARS,
                            _ => {
                                eprintln!("Invalid argument: '{}' in '{}'", char, v);
                                exit(1);
                            }
                        }
                    }
                }
            }
        } else {
            options_end = Some(k);
        }
    }

    // Remove any sequences that were excluded
    let (sequences, escape_chars): (Vec<_>, Vec<_>) = SEQUENCES
        .into_iter()
        .zip(escape_chars)
        .filter(|(s, _)| !removed.contains(s))
        .unzip();

    // Create the searcher based on whether we are escaping or unescaping
    let other;
    let searcher = if undo {
        other = &sequences;
        AhoCorasick::new(escape_chars)
    } else {
        other = &escape_chars;
        AhoCorasick::new(sequences)
    }
    .unwrap();

    // Escape or unescape the sequences starting at the index of the first non-option argument
    if let Some(end) = options_end {
        for mut sequence in args.into_iter().skip(end) {
            if sequence == "-" {
                sequence = read_stdin()?;
            }
            writeln!(stdout(), "{}", searcher.replace_all(&sequence, other))?;
        }
    }

    Ok(())
}
