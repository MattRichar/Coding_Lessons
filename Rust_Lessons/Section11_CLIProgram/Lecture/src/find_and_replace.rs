use text_colorizer::*;//* mean add everything from that crate */
use std::env;
use std::fs;
use regex::Regex;
//This crate provides routines for searching strings for matches of a regular expression (aka “regex”).

#[derive(Debug)]
//Supress warnings to the terminal
#[allow(dead_code)]
struct Arguments{
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

fn print_help(){
    eprintln!("{} - replace a string with a new string", "Find and Replace".green());//Print to error messages
    eprintln!("Usage: <target string> <replacement string> <INPUT FILE> <OUTPUT FILE>");
}

fn replace(target: &str, rep: &str, data: &str) -> Result<String, regex::Error>{
    //replace takes in a target, rep, and data as strings.
    //It returns a Result 
    // enum Result<T,E>
    // {
    //     Ok(T),
    //     Err(E),
    // }
    let regex = Regex::new(target)?;
    //If this succefully runs through the error propogation of ?
    Ok(regex.replace_all(data,rep).to_string())
}

fn read_and_write(args: &Arguments){
    let data = match fs::read_to_string(&args.input_file){
        //read_to_string(<INPUT>) takes the <INPUT> and reads the entire contents of a file into a string
        Ok(f) => f,
        Err(e) => {
            eprintln!("{} Failed to read from file {}; {:?}", "Error".red().bold(), args.input_file,e);{}
            std::process::exit(-2);
        }
    };
    let replace_data = match replace(&args.pattern, &args.replace, &data){
        Ok(d) => d,
        Err(e) => {
            eprintln!("{} Failed to replace text; {:?}", "Error".red().bold(), e);
            std::process::exit(-4);
        }
    };

    match fs::write(&args.output_file, &replace_data){
        //Writes a slice as the entire contents of a file.
        //This function will create a file if it does not exist, and will entirely replace its contents if it does.
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} Failed to write to file {}; {:?}", "Error".red().bold(), args.input_file,e);
            std::process::exit(-3);
        }
    }
}

fn parse_args() -> Arguments{
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4{
        print_help();
        eprintln!("{} Wrong number of arguments given, expected 4, got {}", "Error".red().bold(), args.len());
        std::process::exit(-1);
    }

    Arguments{
        pattern: args[0].clone(),
        replace: args[1].clone(),
        input_file: args[2].clone(),
        output_file: args[3].clone(),
    }
}

pub fn run(){
    let args = parse_args();
    println!("{:?}", args);

    read_and_write(&args);
}

