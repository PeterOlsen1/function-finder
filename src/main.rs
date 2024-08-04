mod finders;
mod utils;

use std::io::Result;
use finders::{find_all_definitions, find_single};
use std::env;
use utils::parsers::CliParser;


fn main() -> Result<()> {
    let parser = utils::parsers::CliParser::new().parser;
    let matches = parser.get_matches();
    
    let args: Vec<String> = env::args().collect();


    // let _result = find_all_definitions("test.js");
    // let _result = find_all_definitions("nofunctions.js");
    let _result = find_all_definitions("onefunction.js");


    //two extra arguments, filename and function to find?
    //or also maybe unsupported features so far
    if args.len() == 3 {
        //check if they want to find in a js file
        match args[1].as_str() {
            "findall" if args[2].ends_with(".js") => println!("Find all occurences of some function in all files!"),
            _ if args[2].ends_with(".js") => find_single(&args[2], &args[1])?,
            _ => println!("End of the pattern"),
        }
    }

    Ok(())
}
