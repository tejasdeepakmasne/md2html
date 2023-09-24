use std::fs::*;
use std::io::*;
use std::env;
use regex::Regex;

fn main() {
    let args : Vec<String> = env::args().collect();
    
    if args.len() != 5 || args[1] != "-i" || args[3] != "-o" {
        print_help()
    }
    else if args.len() == 1 || (args.len() == 2 && args[1] == "-h") {
        print_help()
    }

    let input_file_path = check_input_file_extention(&args[2]);
    let output_file_path = check_output_file_extention(&args[4]);

    let markdown_file = match File::open(input_file_path) {
        Ok(file) => file,
        Err(_) => {
            eprint!("Unable to open the file");
            std::process::exit(1);
        }
    };
    let html_file = match File::create(output_file_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Unable to open/create file please check permissions");
            std::process::exit(1);
        }
    };
}

fn check_input_file_extention (filename : &String) -> String {
    let extention = Regex::new(r#"\.md$"#).unwrap();

    if extention.is_match(&filename) {
        return String::from(filename);
    } else {
        eprintln!("Please use a markdown file");
        std::process::exit(1);
    }
    
}

fn check_output_file_extention (filename : &String) -> String {
    let extention = Regex::new(r#"\.html$"#).unwrap();

    if extention.is_match(&filename) {
        return String::from(filename);
    } else {
        eprintln!("Please use a html file");
        std::process::exit(1);
    }
    
}

fn print_help() {

}