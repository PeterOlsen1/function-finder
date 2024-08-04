/**
 * The purpose of this module is to find all definitions of functions within a file.
 * There is a lof of potential here:
 *      Find all definitions in a directory ?
 *      use multithreading to make the process much faster?
 *  Get the base going first
 */


use crate::utils::types::Definition;
use crate::utils::parsers::{parse_name, parse_params, parse_valid_function};
use std::fs;
use std::io::{self, BufRead, Result};

pub fn find_all_definitions(filename: &str) -> Result<()> {
    let result = all_definitions(filename);
    println!("===== RESULTS OF all_definitions IN {} =====", filename);

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

fn all_definitions(filename: &str) -> Option<Vec<Definition>> {

    //get path and open file
    let path = &format!("./testfiles/{}", filename);
    let f = fs::File::open(path.clone()).ok()?;
    
    //initialize file reader, increment variable, and line store
    let reader = io::BufReader::new(f);
    let mut i: u16 = 0;
    let mut defs: Vec<Definition> = Vec::new();

    for line in reader.lines() {
        i += 1;
        //get the line string and check if it contains a function
        let line = line.ok()?;
        if parse_valid_function(&line) {
            //gather parts of the line
            let parts: Vec<&str> = line
                .split("function")
                .collect();
            let replaced = parts[1]
                .replace("{", "");

            parse_valid_function(&line);
            //push to defs
            defs.push(Definition {
                content:  String::from(&replaced),
                name: parse_name(&line),
                idx: i,
                params: parse_params(&line),
                filename: String::from(path)
            });
        }
    }

    if (defs.len() > 0) {
        return Some(defs);
    }
    None
}