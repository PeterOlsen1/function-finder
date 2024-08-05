mod finders;
mod utils;

use std::io::Result;
use finders::{find_all_definitions, find_single};
use utils::parsers::CliParser;


fn main() -> Result<()> {
    let mut cli_parser = utils::parsers::CliParser::new();
    let matches = cli_parser.parser.get_matches();


    //parse all arguments to the command line
    //filename
    if let Some(file) = matches.get_one::<String>("file") {
        cli_parser.filename = Some(String::from(file));
    } else {
        cli_parser.filename = None;
    }

    //function
    if let Some(function) = matches.get_one::<String>("function") {
        cli_parser.function = Some(String::from(function));
    } else {
        cli_parser.function = None;
    }

    //directory
    if let Some(directory) = matches.get_one::<String>("directory") {
        cli_parser.directory = Some(String::from(directory));
    } else {
        cli_parser.directory = None;
    }
    
    //


    Ok(())
}
