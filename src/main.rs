use std::{collections::HashMap, process::exit, borrow::{BorrowMut}};


fn main() {
    let mut sequences: HashMap<char, String> = HashMap::from_iter([
        (' ', "%20"),
        ('\n',"%0A"),
        ('#', "%23"),
        ('$', "%24"),
        ('%', "%25"),
        ('&', "%26"),
        ('@', "%40"),
        ('`', "%60"),
        (':', "%3A"),
        (';', "%3B"),
        ('<', "%3C"),
        ('=', "%3D"),
        ('>', "%3E"),
        ('?', "%3F"),
        ('[', "%5B"),
        ('\\',"%5C"),
        (']', "%5D"),
        ('^', "%5E"),
        ('{', "%7B"),
        ('|', "%7C"),
        ('}', "%7D"),
        ('~', "%7E"),
        ('"', "%22"),
        ('\'',"%27"),
        ('+', "%2B"),
        (',', "%2C")
    ].into_iter().map(|(key, val)| (key, val.to_string())));
    const AVAILABLE_COMMANDS: [&str; 12] = [
        "-n", "--no-newline",
        "-q", "--quotes",
        "-b", "--backticks",
        "-a", "--apostrophes",
        "-u", "--undo",
        "-s", "--string",
    ];
    let args: Vec<String> = std::env::args().collect();
    let args: Vec<&str> = args.iter().map(|x| x.as_str()).collect();
    let mut undo = false;
    for i in args.into_iter().skip(1){
        let mut chars = i.chars();
        if chars.next() == Some('-') {
            if chars.clone().next().is_some_and(|x| x != '-'){
                for char in chars{
                    match char {
                        'q' => {sequences.remove(&'\"');},
                        'b' => {sequences.remove(&'`');},
                        'a' => {sequences.remove(&'\'');},
                        'u' => {undo = true},
                        'h' => {print_help();},
                        's' => {sequences = sequences.into_iter().map(|(key, val)| (key, val.replace('%', "$"))).collect()},
                         _  => {
                            eprintln!("Invalid argument: {}", i);
                            exit(1);
                        }
                    }
                }
            }else if chars.next().is_none(){
                let lines = std::io::stdin().lines();
                for line in lines {
                    if undo {
                        undo_escape_sequence(&line.unwrap(), &sequences);
                    }else{
                        escape_sequence(&line.unwrap(), &sequences);
                    }
                }
            }else {
                match i {
                    "--help" => {print_help();},
                    "--string" => {sequences = sequences.into_iter().map(|(key, val)| (key, val.replace('%', "$"))).collect()},
                    "--quotes" => {sequences.remove(&'\"');},
                    "--backticks" => {sequences.remove(&'`');},
                    "--apostrophes" => {sequences.remove(&'\'');},
                    "--undo" => {undo = true},
                    _ => {
                        eprintln!("Invalid argument: {}", i);
                        exit(1);
                    }
                }
            }
        }else{
            if undo {
                undo_escape_sequence(i, &sequences);
            }else{
                escape_sequence(i, &sequences);
            }
        }
    }
}

fn print_help(){
    const BOLD: &str = "\x1b[1m";
    const UND: &str = "\x1b[4m";
    const RES: &str = "\x1b[0m";
    println!("{}A program that escapes (or unescapes) special characters for URL sequences", BOLD);
    println!("{}Usage: escaper [OPTIONS] [SEQUENCES]{}", BOLD, RES);
    println!("{}-h, --help                          {}Print the help information.", BOLD, RES);
    println!("{} -                                  {}Reads SEQUENCES from stdin.", BOLD, RES);
    println!("{}-s, --string                        {}Reads uses string ($) escape sequences instead of default (%) escape sequences.", BOLD, RES);
    println!("{}-u, --undo                          {}Unescapes SEQUENCES.", BOLD, RES);
    println!("{}EXCLUSION OPTIONS:", BOLD);
    println!("{}-a, --apostrophes                   {}Removes apostrophes from escapable character list", BOLD, RES);
    println!("{}-b, --backticks                     {}Removes backticks from escapable character list", BOLD, RES);
    println!("{}-q, --quotes                        {}Removes quotation marks from escapable character list", BOLD, RES);
}

fn escape_sequence(sequence: &str, sequences: &HashMap<char, String>){
    let mut result = String::new();
    for i in sequence.chars() {
        result.push_str(sequences.get(&i).unwrap_or(&&i.to_string()));
    }
    println!("{}", result)
}

fn undo_escape_sequence(sequence: &str, sequences: &HashMap<char, String>){
    let sequences: HashMap<&String, char> = HashMap::from_iter(sequences.into_iter().map(|(key, val)| (val, *key)));
    let mut result = String::new();
    let mut count = 0;
    for (key, val) in sequence.chars().enumerate(){
        if count > 0{
            count -= 1;
            continue;
        }
        if val != '%' && val != '$' {
            result.push(val);
        }else{
            if let Some(char) = sequences.get(&sequence.chars().skip(key).take(3).collect::<String>()){
                result.push(*char);
                count = 2;
            }else{
                result.push(val);
            }
        }
    }
    println!("{}", result)
}
