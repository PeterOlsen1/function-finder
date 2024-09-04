use clap::{Arg, Command};

use super::types::{Call, Definition};

/**
 * The purpose of this module is to provide single line parsers to extract data 
 */

//======================================================================
//Command line parser

pub struct CliParser {
    pub parser: Command,
    pub filename: Option<String>,
    pub function: Option<String>,
    pub directory: Option<String>,
    pub show_type: Option<String>,
    pub recursive_flag: bool
}

impl CliParser {
    pub fn new() -> Self {
        CliParser {
            parser: Command::new("function-finder")
                .version("1.0")
                .author("Author Name: Peter Olsen")
                .about("Does awesome things")
                .arg(
                    Arg::new("file")
                    .short('f')
                    .long("filename")
                    .help("Specify what JavaScript file to search for. Must have the .js extension.")
                )
                .arg(
                    Arg::new("function_name")
                    .short('n')
                    .long("name")
                    .help("The function to search for. No need to specify params.")
                )
                .arg(
                    Arg::new("directory")
                        .short('d')
                        .long("directory")
                        .help("Choose a directory to search for search type.")
                )
                .arg(
                    Arg::new("recursive_flag")
                        .short('r')
                        .long("recursive")
                        .help("Specify whether or not to recursively search a directory.\nEnabled by default.")
                ),
                // .arg(
    //                 Arg::new("type")
    //                     .short('t')
    //                     .long("type")
    //                     .help("Choose a type to display results for:
    // A / ALL: Show function declarations and calls (default)
    // F / FUNCTIONS: Show function declarations
    // C / CALLS: Show only function calls")
    //             ),
            filename: None,
            function: None,
            directory: None,
            show_type: None,
            recursive_flag: false
        }
    }
}




//======================================================================
//Line parsers

/**
 * @purpose - Helper function to parse the arguments of a function signature
 */
pub fn parse_params(line: &str) -> Vec<String> {
    //extract function signature from the line
    let mut function_split: Vec<&str> = line.split("function")
        .into_iter()
        .collect();

    //if the word 'function' is contained in the function name, join them together
    let joined;
    if function_split.len() > 2 {
        let rest = &function_split[1..];
        joined = rest.join("");
        function_split[1] = &joined;
    }

    let signature = function_split[1];

    //find the parenthesis
    let first_parenthesis = signature
        .find('(')
        .expect("Couldn't Find First Parenthesis");
    let second_parenthesis = signature
        .find(')')
        .expect("Couldn't Find Second Parenthesis");

    //slice the string between parenthesis
    let param_string: &str = &signature[first_parenthesis + 1..second_parenthesis];
    let params: Vec<String> = param_string
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    //return the list of parameters
    params
}

/**
 * @purpose - Function to get the name from a signature string
 */
pub fn parse_name(line: &str) -> String {
    //extract function signature from the line
    let line_split: Vec<&str> = line.split("function")
        .into_iter()
        .collect();
    let signature = line_split[1];

    let split_line: Vec<String> = signature
        .split("(")
        .map(|s| s.trim().to_string())
        .collect();

    //return the name
    split_line[0].clone()
}

/**
 * Function to check if this is actually a function definition
 * AKA make sure that it is not wrapped in quotes / comments
 */
pub fn parse_valid_function(line: &str) -> bool {
    if !line.contains("function") {
        false
    }
    else {
        let function_split: Vec<&str> = line.split("function")
            .into_iter()
            .collect();

        //function_split[0] will be empty if no text comes before 'function'
        if function_split[0].is_empty() {
            true
        }
        else {
            //case where there are things before 'function', we need to check 'em
            let part = function_split[0];

            if part.contains("//") {
                return false;
            }

            //check if there are an even number of quotation marks
            if part.chars().filter(|&c| c == '\'' || c == '"').count() % 2 != 0 {
                return false;
            }
            false
        }
    }
}


/**
 * General fucntion to parse
 * 
 * Best case O(1)
 *      -The line length is less than 8 (can't contain a function)
 *      -Line starts with * or //
 * Worst case O(n)
 *      -We have to split the line and parse it
 */
pub fn parse_def(line: &str, idx: u16, filename: &str) -> Option<Definition> {
    if line.len() < 8 || line.starts_with('*') || line.starts_with("//") {
        return None;
    }

    //split the line by the term 'function'
    let mut function_split: Vec<&str> = line.split("function")
        .into_iter()
        .collect();


    //one entry means no explicit function defined
    if function_split.len() == 1 {
        //check for arrow function here?
        // if line.contains("=>") {
        //     dbg!("Arrow function detected!");
        // }
        return None;
    }
    else {
        //case where there are things before 'function', we need to check 'em
        let part = function_split[0];

        //check if there are an even number of quotation marks
        if part.chars().filter(|&c| c == '\'' || c == '"').count() % 2 != 0 {
            return None;
        }

        //check for async
        let mut async_flag = false;
        if part.contains("async") {
            async_flag = true;
        }

        //check for export
        let mut export_flag = false;
        if part.contains("export") {
            export_flag = true;
        }

        //if the word 'function' is contained in the function name, join them together
        let joined;
        if function_split.len() > 2 {
            let rest = &function_split[1..];
            joined = rest.join("");
            function_split[1] = &joined;
        }

        let content = function_split[1]
            .replace("{", "");
        let content = content
            .trim();

        let signature = match content.find('(') {
            Some(idx) => &content[0..idx],
            None => ""
        };

        return Some (Definition {
            content: String::from(content),
            name: String::from(signature),
            idx,
            params: parse_params(line),
            filename: String::from(filename),
            async_flag,
            export_flag
        });
    } 
}


/**
 * Function to parse a line for a function call, given the name of a function.
 * 
 * It will return a new call object
 */
pub fn parse_call(fn_name: &str, line: &str, idx: u16, filename: &str) -> Option<Call> {
    if line.len() < (fn_name.len()  + 2) || line.starts_with('*') || line.starts_with("//")  || !line.contains(fn_name) {
        return None;
    }
    
    if line.contains(fn_name) {
        let mut await_flag = false;

        if line.contains("await") {
            await_flag = true;
        }

        return Some(Call {
            filename: String::from(filename),
            content: String::from(line),
            idx,
            params: parse_params(line),
            await_flag
        })
    }

    None
}