use crate::utils::types::Definition;
use std::collections::HashMap;
use std::io::{self, Write, Result};


/**
 * This function is created in order to display a hash table that contains information about function definitions
 * There is user interactivity to display different things about the results
 */
pub fn display_hash(data: Option<HashMap<String, Vec<Definition>>>) -> Result<()> {
    //add code here to interactively display information from the hash table. Allow user to interact with it?
    let data = data.unwrap_or_else(HashMap::new);

    let mut input = String::new();

    //gather the total number of definitions and files parsed
    let mut defs = 0;
    let mut files = 0;
    for (_, def_vec) in &data {
        files += 1;
        defs += def_vec.len();
    }
    println!("==== DIRECTORY SEARCH RESULTS ====");
    
    if files == 0 {
        println!("No data was found.");
        return Ok(());
    }

    println!("\x1b[1m{}\x1b[0m functions found across \x1b[1m{}\x1b[0m files", defs, files);
    println!("Type -help for help.");

    //enter input loop
    loop {
        //clear the def_vec of input
        input = String::from("");

        //prompt the user for new input
        print!("Input: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input)
            .expect("Unable to read line");


        //user wants to exit
        if input == "\n" || input.contains("-quit") || input.contains("-q") {
            println!("Quitting...");
            return Ok(());
        }
        //view help menu
        else if input.contains("-help") || input.contains("-h") {
            println!("==== HELP MENU ====");
            println!("Flags         Description");
            println!("-all          Displays information about all functions");
            println!("-help -h      Displays help menu");
            println!("-file -f      Shows information from a single file");
            println!("-func -fn     Displays information about an indiviudal function");
            println!("-a            Displays async functions");
            println!("-e            Displays exported functions");
            println!("-quit -q ''   Quits the input loop");
            println!();
            println!("Only one flag can be used at a time.");
        }

        //view all functions
        else if input.contains("-all") {
            for (key, def_vec) in &data {
                println!();
                println!("==== FILE: {} ====", key);

                if def_vec.len() == 0 {
                    println!("No functions defined in this file.");
                    continue;
                }

                for def in def_vec {
                    println!("Function name: \x1b[1m{}\x1b[0m", def.name);
                    println!("Defined on: line \x1b[1m{}\x1b[0m", def.idx);
                    println!("Parameters: ({})", def.params.join(", "));
                    println!();
                }
            }
        }

        //finding a single function
        else if input.contains("-func") || input.contains("-fn") {
            let mut found = false;
            let parts: Vec<&str> = input
                .split_whitespace()
                .collect();
            let function_name = parts[1];

            for (_, def_vec) in &data {
                if found {
                    break;
                }

                for def in def_vec {
                    if found {
                        break;
                    }

                    //we find the matching function
                    if function_name == def.name {
                        found = true;
                        println!();
                        println!("Function found!");
                        println!("Defined on: line \x1b[1m{}\x1b[0m in file \x1b[1m{}\x1b[0m", def.idx, def.filename);
                        println!("Parameters: ({})", def.params.join(", "));
                    }
                }
            }

            if !found {
                println!();
                println!("No function with the name \x1b[1m{}\x1b[0m found", function_name);
            }
        }

        //functions in a single file
        else if input.contains("-f") || input.contains("-file") {
            let mut found = false;
            let parts: Vec<&str> = input
                .split_whitespace()
                .collect();
            let file_name = parts[1];

            for (key, def_vec) in &data {
                if key.contains(file_name) {
                    found = true;
                    println!("==== RESULTS FOR FILE {} ====", file_name);
                    for def in def_vec {
                        println!("Function name: \x1b[1m{}\x1b[0m", def.name);
                        println!("Defined on: line \x1b[1m{}\x1b[0m", def.idx);
                        println!("Parameters: ({})", def.params.join(", "));
                        println!();
                    }
                }
            }

            //no results
            if !found {
                println!();
                println!("No file with the name {} found", file_name);
            }
        }

        //async functions
        else if input.contains("-a") {
            let mut found = false;
            for (_, def_vec) in &data {
                for def in def_vec {
                    if def.async_flag {
                        found = true;
                        println!();
                        println!("==== Async Function {} ====",  def.name);
                        println!("Defined on: line \x1b[1m{}\x1b[0m in file \x1b[1m{}\x1b[0m", def.idx, def.filename);
                        println!("Parameters: ({})", def.params.join(", "));
                    }
                }
            }

            if !found {
                println!("No async functions found");
            }
        }

        //exported functions
        else if input.contains("-e") {
            let mut found = false;
            for (_, def_vec) in &data {
                for def in def_vec {
                    if def.export_flag {
                        found = true;
                        println!();
                        println!("==== Exported Function {} ====",  def.name);
                        println!("Defined on: line \x1b[1m{}\x1b[0m in file \x1b[1m{}\x1b[0m", def.idx, def.filename);
                        println!("Parameters: ({})", def.params.join(", "));
                    }
                }
            }

            if !found {
                println!("No exported functions found")
            }
        }
    }
}