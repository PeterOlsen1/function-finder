mod finders;
mod utils;

use std::io::Result;
use finders::{
    all_definitions, find_all_definitions, find_all_directory::{find_call_directory, find_call_directory_rec, find_def_directory, find_def_directory_rec
    }, find_single::read_single_function, show_single};
use utils::parsers::{CliParser, parse_line};


fn main() -> Result<()> {
    let mut cli_parser = utils::parsers::CliParser::new();
    let matches = cli_parser.parser.get_matches();


    //parse all arguments to the command line
    //filename
    if let Some(file) = matches.get_one::<String>("file") {
        cli_parser.filename = Some(file.clone());
    } 
    else {
        cli_parser.filename = None;
    }

    //function
    if let Some(function) = matches.get_one::<String>("function_name") {
        cli_parser.function = Some(function.clone());
    } 
    else {
        cli_parser.function = None;
    }

    //directory
    if let Some(directory) = matches.get_one::<String>("directory") {
        cli_parser.directory = Some(directory.clone());
    } 
    else {
        cli_parser.directory = None;
    }

    //recursive flag
    if matches.get_one::<String>("recursive_flag").unwrap() != "" {
        cli_parser.recursive_flag = false;
    } 
    else {
        cli_parser.recursive_flag = true;
    }
    
    //put those arguments to use!

    //the user specifies a directory and wants recursive
    if cli_parser.directory != None && cli_parser.recursive_flag {
        //did they ask for a specific function
        if cli_parser.function != None {
            find_call_directory_rec(&cli_parser.directory.unwrap(), &cli_parser.function.unwrap());
        }
        else {
            find_def_directory_rec(&cli_parser.directory.unwrap());
        }
    }
    //user specified directory but no recursive flag
    else if cli_parser.directory != None {
        //again, ask for a specific function?
        if cli_parser.function != None {
            find_call_directory(&cli_parser.directory.unwrap(), &cli_parser.function.unwrap());
        }
        else {
            find_def_directory(&cli_parser.directory.unwrap());
        }
    }
    //the user asked for a filename
    else if cli_parser.filename != None {
        //did they ask to find a certain function?
        if cli_parser.function != None {
            read_single_function(&cli_parser.filename.unwrap(), &cli_parser.function.unwrap());
        }
        //no function specified, just find definitions
        else {
            all_definitions(&cli_parser.filename.unwrap());
        }
    }
    else {
        println!("Inputs not recognized, unable to perform finding operation.");
    }



    // let result = parse_line("OOK_AT_ME(HELLO, LOOK, HERE, HI) {", 1, "./testfiles/folder/superfolder/woah.js");
    // let result = parse_line("function LOOK_AT_ME(HELLO, LOOK, HERE, HI) {", 1, "./testfiles/folder/superfolder/woah.js");
    // let result = parse_line("async function LOOK_AT_ME(HELLO, LOOK, HERE, HI) {", 1, "./testfiles/folder/superfolder/woah.js");
    // dbg!(result);
    Ok(())
}
