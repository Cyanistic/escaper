use std::io::{self, stderr, stdin, stdout, IsTerminal, Read, Write};
use std::process::exit;

pub mod consts;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn read_stdin() -> Result<String, io::Error> {
    if stdin().is_terminal() {
        writeln!(stderr(), "No input provided.\nExpected usage:")?;
        writeln!(stderr(), "        echo 'Hello World' | {PKG_NAME} -")?;
        exit(0);
    } else {
        let mut buf = String::new();
        stdin().read_to_string(&mut buf)?;
        Ok(buf)
    }
}

pub fn print_help() -> Result<(), std::io::Error>{
    const BOLD: &str = "\x1b[1m";
    const UND: &str = "\x1b[4m";
    const RES: &str = "\x1b[0m";
    if stdout().is_terminal() {
        writeln!(stdout(), "{}Escapes (or unescapes) special character sequences and prints the result to stdout.\n", BOLD)?;
        writeln!(stdout(), "{}{}Usage:{}{} {PKG_NAME} [OPTIONS] [SEQUENCES...]{}\n", BOLD, UND, RES, BOLD, RES)?;
        writeln!(stdout(), "{}{}OPTIONS:{}", BOLD, UND, RES)?;
        writeln!(stdout(), "  {}-h, --help                    {}Print the help information and exit.", BOLD, RES)?;
        writeln!(stdout(), "  {}-V, --version                 {}Print version and exit.", BOLD, RES)?;
        writeln!(stdout(), "  {} -                            {}Reads SEQUENCES from stdin.", BOLD, RES)?;
        writeln!(stdout(), "  {}--                            {}Causes all further arguments to be read as SEQUENCES.", BOLD, RES)?;
        writeln!(stdout(), "  {}-s, --string                  {}Uses string ($) escape sequences instead of default URL (%) escape sequences.", BOLD, RES)?;
        writeln!(stdout(), "  {}-r, --regex                   {}Uses shell/regex (\\) escape sequences instead of default URL (%) escape sequences.", BOLD, RES)?;
        writeln!(stdout(), "  {}-u, --undo                    {}Unescapes SEQUENCES.", BOLD, RES)?;
        writeln!(stdout(), "{}{}EXCLUSION OPTIONS:{}", BOLD, UND, RES)?;
        writeln!(stdout(), "  {}-a, --apostrophes             {}Removes apostrophes from escapable character list.", BOLD, RES)?;
        writeln!(stdout(), "  {}-n, --new-line                {}Removes new lines from escapable character list.", BOLD, RES)?;
        writeln!(stdout(), "  {}-b, --backticks               {}Removes backticks from escapable character list.", BOLD, RES)?;
        writeln!(stdout(), "  {}-q, --quotes                  {}Removes quotation marks from escapable character list.", BOLD, RES)?;
    }else{
        writeln!(stdout(), "Escapes (or unescapes) special character sequences and prints the result to stdout.\n")?;
        writeln!(stdout(), "Usage: {PKG_NAME} [OPTIONS] [SEQUENCES...]\n")?;
        writeln!(stdout(), "OPTIONS:")?;
        writeln!(stdout(), "  -h, --help                    Print the help information and exit.")?;
        writeln!(stdout(), "  -V, --version                 Print version and exit.")?;
        writeln!(stdout(), "   -                            Reads SEQUENCES from stdin.")?;
        writeln!(stdout(), "  --                            Causes all further arguments to be read as SEQUENCES.")?;
        writeln!(stdout(), "  -s, --string                  Uses string ($) escape sequences instead of default URL (%) escape sequences.")?;
        writeln!(stdout(), "  -r, --regex                   Uses shell/regex (\\) escape sequences instead of default URL (%) escape sequences.")?;
        writeln!(stdout(), "  -u, --undo                    Unescapes SEQUENCES.")?;
        writeln!(stdout(), "EXCLUSION OPTIONS:")?;
        writeln!(stdout(), "  -a, --apostrophes             Removes apostrophes from escapable character list.")?;
        writeln!(stdout(), "  -n, --new-line                Removes new lines from escapable character list.")?;
        writeln!(stdout(), "  -b, --backticks               Removes backticks from escapable character list.")?;
        writeln!(stdout(), "  -q, --quotes                  Removes quotation marks from escapable character list.")?;
    }
    exit(0);
}
