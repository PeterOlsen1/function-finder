use crate::utils::types::{Call, Definition};
use crate::utils::parsers::{parse_name, parse_params};
use std::fs;
use std::io::{self, BufRead, Result};

/**
 * @purpose - Calls read_single_function and then displays the results accordingly.
 */
pub fn find_single(filename: &str, name: &str) -> Result<()> {
    let result: (Vec<Definition>, Vec<Call>) = read_single_function(filename, name)?;

    println!("===== RESULTS OF find_single FOR FUNCTION {} IN FILE {} =====", name, filename);
    //only one definition
    if result.0.len() == 0 {
        println!("No function definitions found...");
        println!("This program is still young, so if you defined your function with anything other than the 'function' keyword, that may be why.");
    }
    else if result.0.len() == 1 {
        let function = &result.0[0];
        println!("One function definition found: ");
        println!("  Defined in: {} on line {}", function.filename, function.idx);
        println!("  Parameters: {}", function.params.join(","));
    }
    else {
        println!("Multiple definitions found!");

        let mut i: i8 = 1;
        for function in result.0 {
            println!("Function: {}", function.name);
            println!("Function number {}", i);
            println!("  Defined in: {} on line {}", function.filename, function.idx);
            println!("  Parameters: {}", function.params.join(","));

            i += 1;
        }   
    }

    println!("\n===== FUNCTION CALLS =====");
    if result.1.len() == 0 {
        println!("This function has not been called yet");
    }
    else {
        for call in result.1 {
            println!("Called: {} on line {}", call.filename, call.idx);
            println!("Call: {}\n", call.content);
        }
    }

    Ok(())
}


fn read_single_function(filename : &str, name: &str) -> Result<(Vec<Definition>, Vec<Call>)> {
    //get the filename and open file
    let path = &format!("./testfiles/{}.js", filename);
    let f = fs::File::open(path.clone())?;
    
    //initialize file reader, increment variable, and line store
    let reader = io::BufReader::new(f);
    let mut i: u16 = 0;
    let mut lines: Vec<Call> = Vec::new();
    let mut defs: Vec<Definition> = Vec::new();

    //loop through the lines of the given file
    for line in reader.lines() {
        //increment line# and create line variable
        i += 1;
        let line = &line?;

        //parse the line
        if has_function(line, name) {
            //check if the line is a function definition
            if line.contains("function") && !line.contains("//") {
                //gather parts of the line
                let parts: Vec<&str> = line
                    .split("function")
                    .collect();
                let replaced = parts[1]
                    .replace("{", "");
                let signature = replaced.trim();

                defs.push(Definition {
                    content:  String::from(&replaced),
                    name: parse_name(signature),
                    idx: i,
                    params: parse_params(signature),
                    filename: String::from(path)
                });
            }
            else {
                //push the line to our store of lines
                lines.push(Call {
                    filename : String::from(path),
                    content: String::from(line),
                    idx: i
                });
            }
        }
    }

    //set the components of result and return it
    Ok((defs, lines))
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
