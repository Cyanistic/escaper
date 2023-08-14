use std::{collections::HashMap, process::exit};
use std::io::{stdout, stdin, Write, IsTerminal, stderr, Read};

fn main() -> Result<(), std::io::Error>{
    let mut sequences: HashMap<char, String> = HashMap::from([
        (' ', "%20".to_string()),
        ('\n',"%0A".to_string()),
        ('#', "%23".to_string()),
        ('$', "%24".to_string()),
        ('%', "%25".to_string()),
        ('&', "%26".to_string()),
        ('@', "%40".to_string()),
        ('`', "%60".to_string()),
        (':', "%3A".to_string()),
        (';', "%3B".to_string()),
        ('<', "%3C".to_string()),
        ('=', "%3D".to_string()),
        ('>', "%3E".to_string()),
        ('?', "%3F".to_string()),
        ('[', "%5B".to_string()),
        ('\\',"%5C".to_string()),
        (']', "%5D".to_string()),
        ('^', "%5E".to_string()),
        ('{', "%7B".to_string()),
        ('|', "%7C".to_string()),
        ('}', "%7D".to_string()),
        ('~', "%7E".to_string()),
        ('"', "%22".to_string()),
        ('\'',"%27".to_string()),
        ('+', "%2B".to_string()),
        (',', "%2C".to_string())
    ]);
    const AVAILABLE_COMMANDS: [&str; 12] = [
        "-q", "--quotes",
        "-b", "--backticks",
        "-a", "--apostrophes",
        "-u", "--undo",
        "-s", "--string",
        "-r", "--regex",
    ];
    let args: Vec<String> = std::env::args().collect();
    let mut undo = false;
    let mut options_end = false;
    for i in args.into_iter().skip(1){
        let mut chars = i.chars();
        if chars.next().is_some_and(|x| x == '-') && !options_end {
            match chars.clone().next() {
                Some('-') => {
                    chars.next();
                    match chars.as_str() {
                        "help"        => {print_help().expect("Couldn't write to stdout");},
                        "version"     => {writeln!(stdout(), "{}", env!("CARGO_PKG_VERSION")).expect("Couldn't write to stdout"); exit(0);},
                        "string"      => {sequences = sequences.into_iter().map(|(key, val)| (key, val.replace('%', "$"))).collect()},
                        "regex"       => {sequences = sequences.into_keys().map(|key| (key, format!("\\{}", key).to_string())).collect()},
                        "quotes"      => {sequences.remove(&'\"');},
                        "backticks"   => {sequences.remove(&'`');},
                        "apostrophes" => {sequences.remove(&'\'');},
                        "new-line"    => {sequences.remove(&'\n');},
                        "undo"        => {undo = true},
                        ""            => {options_end = true;},
                        _ => {
                            eprintln!("Invalid argument: {}", i);
                            exit(1);
                        }
                    }
                },
                None => {
                    if stdin().is_terminal(){
                        writeln!(stderr(), "No input provided.\nExpected usage:")?;
                        writeln!(stderr(), "        echo 'Hello World' | escaper -")?;
                        exit(0);
                    }else {
                        let mut buf = String::new();
                        stdin().read_to_string(&mut buf)?;
                        if undo {
                            undo_escape_sequence(&buf, &sequences).expect("Couldn't write to stdout");
                        }else{
                            escape_sequence(&buf, &sequences).expect("Couldn't write to stdout");
                        }
                    }
                },
                Some(_) => {
                    for char in chars{
                        match char {
                            'q' => {sequences.remove(&'\"');},
                            'b' => {sequences.remove(&'`');},
                            'a' => {sequences.remove(&'\'');},
                            'n' => {sequences.remove(&'\n');},
                            'u' => {undo = true},
                            'h' => {print_help().expect("Couldn't write to stdout");},
                            'V' => {writeln!(stdout(), "{}", env!("CARGO_PKG_VERSION")).expect("Couldn't write to stdout"); exit(0);},
                            's' => {sequences = sequences.into_iter().map(|(key, val)| (key, val.replace('%', "$"))).collect()},
                            'r' => {sequences = sequences.into_keys().map(|key| (key, format!("\\{}", key).to_string())).collect()},
                             _  => {
                                eprintln!("Invalid argument: '{}' in '{}'", char, i);
                                exit(1);
                            }
                        }
                    }
                },
            }
        }else if undo {
            undo_escape_sequence(i.as_str(), &sequences).expect("Couldn't write to stdout");
        }else{
            escape_sequence(i.as_str(), &sequences).expect("Couldn't write to stdout");
        }
    }
    Ok(())
}

fn print_help() -> Result<(), std::io::Error>{
    const BOLD: &str = "\x1b[1m";
    const UND: &str = "\x1b[4m";
    const RES: &str = "\x1b[0m";
    if stdout().is_terminal() {
        writeln!(stdout(), "{}Escapes (or unescapes) special character sequences and prints the result to stdout.\n", BOLD)?;
        writeln!(stdout(), "{}{}Usage:{}{} escaper [OPTIONS] [SEQUENCES...]{}\n", BOLD, UND, RES, BOLD, RES)?;
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
        writeln!(stdout(), "Usage: escaper [OPTIONS] [SEQUENCES...]\n")?;
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

fn escape_sequence(sequence: &str, sequences: &HashMap<char, String>) -> Result<(), std::io::Error>{
    // OLD METHOD
    // let mut result = String::new();
    // for i in sequence.chars() {
    //     result.push_str(sequences.get(&i).unwrap_or(&i.to_string()));
    // }
    //
    let result = sequence.chars().fold(String::new(), |start, add| start + (sequences.get(&add).unwrap_or(&add.to_string())));
    writeln!(stdout(), "{}", result)
}

fn undo_escape_sequence(sequence: &str, sequences: &HashMap<char, String>) -> Result<(), std::io::Error>{
    let sequences: HashMap<&String, &char> = HashMap::from_iter(sequences.iter().map(|(key, val)| (val, key)));
    let mut result = String::new();
    let mut count = 0;
    let start = sequences.keys().next().unwrap().chars().next().unwrap();
    let len = sequences.keys().next().unwrap().chars().count();
    let sequence = sequence.chars().collect::<Vec<char>>();
    for (key, val) in sequence.iter().enumerate(){
        if count > 0{
            count -= 1;
            continue;
        }
        if *val != start{
            result.push(*val);
        }else if let Some(char) = sequences.get(&sequence[key..key+len].iter().collect::<String>()){
            result.push(**char);
            count = 2;
        }else{
            result.push(*val);
        }
    }
    writeln!(stdout(), "{}", result)
}
