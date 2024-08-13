/**
 * The purpose of this module is to find all definitions of functions within a file.
 * There is a lof of potential here:
 *      Find all definitions in a directory ?
 *      use multithreading to make the process much faster?
 *  Get the base going first
 */


use crate::utils::types::Definition;
use crate::utils::parsers::{parse_name, parse_params, parse_valid_function, parse_line};
use std::fs;
use std::io::{self, BufRead, Result};

pub fn find_all_definitions(filename: &str) -> Result<()> {
    let result = all_definitions(filename);
    println!("===== RESULTS OF find_all_definitions IN {} =====", filename);

    //check the result of the option type
    match result {
        None => println!("Unable to find any definitions in the given file."),
        Some (defs) => {
            //change output prompt based on the number of definitions
            if defs.len() == 1 {
                println!("One definition found!\n");
                println!("Function: {}", defs[0].name);
                println!("  Defined in: '{}' on line {}", defs[0].filename, defs[0].idx);
                println!("  Parameters: ({})\n", defs[0].params.join(", "));
            }
            else {
                //print all function definitions
                println!("{} definitions found, listing all!\n", defs.len());
                let mut i: i16 = 1;
                for function in defs {
                    println!("Function: {}", function.name);
                    println!("Function number {}", i);
                    println!("  Defined in: '{}' on line {}", function.filename, function.idx);
                    println!("  Parameters: ({})\n", function.params.join(", "));
                    i += 1;
                }
            }
        }
    }

    Ok (())
}

pub fn all_definitions(filename: &str) -> Option<Vec<Definition>> {
    //get path and open file
    let path = format!("{}", filename);
    let f = fs::File::open(&path).ok()?;
    
    //initialize file reader, increment variable, and line store
    let reader = io::BufReader::new(f);
    let mut i: u16 = 0;
    let mut defs: Vec<Definition> = Vec::new();

    for line in reader.lines() {
        i += 1;
        //get the line string and check if it contains a function
        let line = line.ok()?;

        //parse the line
        match parse_line(&line, i, &path) {
            Some(def) => defs.push(def),
            None => continue
        }
    }

    //return option depending on how many functions are present
    if defs.len() > 0 {
        return Some(defs);
    }
    None
}