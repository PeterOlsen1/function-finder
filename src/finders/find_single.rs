use crate::utils::types::{Call, Definition};
use crate::utils::parsers::{parse_call, parse_def};
use std::fs;
use std::io::{self, BufRead, Result};


/**
 * @purpose - Calls read_single_function and then displays the results accordingly.
 */
pub fn show_single(filename: &str, name: &str) -> Result<()> {
    let result: (Vec<Definition>, Vec<Call>) = read_single_function(filename, name).unwrap();

    println!("===== RESULTS OF show_single FOR FUNCTION {} IN FILE {} =====", name, filename);
    //only one definition
    if result.0.len() == 0 {
        println!("No function definitions found...");
        println!("This program is still young, so if you defined your function with anything other than the 'function' keyword, that may be why.");
    }
    else if result.0.len() == 1 {
        let function = &result.0[0];
        println!("One function definition found: ");
        println!("  Defined in: '{}' on line {}", function.filename, function.idx);
        println!("  Parameters: ({})", function.params.join(","));
    }
    else {
        //unlikely but possible
        println!("Multiple definitions found!");

        let mut i: i8 = 1;
        for function in result.0 {
            println!("Function: {}", function.name);
            println!("Function number {}", i);
            println!("  Defined in: '{}' on line {}", function.filename, function.idx);
            println!("  Parameters: ({})", function.params.join(","));

            i += 1;
        }   
    }

    //print out all calls to the function
    println!("\n===== FUNCTION CALLS =====");
    if result.1.len() == 0 {
        println!("This function has not been called yet.");
    }
    else {
        for call in result.1 {
            println!("Called: '{}' on line {}", call.filename, call.idx);
            println!("Call: {}\n", call.content);
        }
    }

    Ok(())
}


/**
 * Given a file and function name, search the given file for all occurences of said function.
 * @param filename {&str} the file we want to search
 * @param name {&str} the function we want to search for
 * @return {Option<(Vec<Definition>, Vec<Call>)>} A tuple containing vecs of all definitions and calls in a file.
 *      *the definition vector is included so that if for some reason a function is defined multiple times, it will be caught
 */
pub fn read_single_function(filename : &str, name: &str) -> Option<(Vec<Definition>, Vec<Call>)> {
    //get the filename and open file
    let path = format!("./testfiles/{}", filename);
    let f = fs::File::open(&path).ok()?;
    
    //initialize file reader, increment variable, and line store
    let reader = io::BufReader::new(f);
    let mut i: u16 = 0;
    let mut calls: Vec<Call> = Vec::new();
    let mut defs: Vec<Definition> = Vec::new();

    //loop through the lines of the given file
    for line in reader.lines() {
        //increment line# and create line variable
        i += 1;
        let line = &line.ok()?;

        //parse the line
        if has_function(line, name) {
            match parse_def(&line, i, filename) {
                Some(def) => defs.push(def),
                None => continue
            }
        }
        else {
            match parse_call(&name, &line, i, &filename) {
                Some(call) => calls.push(call),
                None => continue
            }
        }
    }

    //set the components of result and return it
    Some((defs, calls))
}

//function to parse a line, maybe return a line option?
    //if true, send back the entire line
fn has_function(line: &str, name: &str) -> bool {
    if line.contains(&format!("{}(", name)) {
        true
    } 
    else {
        false
    }
}
