use clap::{builder::Str, Arg, Command};

use super::types::Definition;

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
    pub show_type: Option<String>
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
                    Arg::new("type")
                        .short('t')
                        .long("type")
                        .help("Choose a type to display results for:
    A / ALL: Show function declarations and calls (default)
    F / FUNCTIONS: Show function declarations
    C / CALLS: Show only function calls")
                ),
            filename: None,
            function: None,
            directory: None,
            show_type: None
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
    let line_split: Vec<&str> = line.split("function")
        .into_iter()
        .collect();
    let signature = line_split[1];

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
 * Better than O(n)?
 *      -This would be nice since I want to use this on every line of each file
 */
pub fn parse_line(line: &str, idx: i16, filename: &str) -> Option<()> {
    if line.starts_with('*') || line.starts_with("//") {
        return None;
    }

    let function_split: Vec<&str> = line.split("function")
        .into_iter()
        .collect();

    //function_split[0] will be empty if no text comes before 'function'
    if function_split[0].is_empty() {
        return Some (());
    }
    else if function_split.len() == 1 {
        //check for arrow function here?
        if line.contains("=>") {
            dbg!("Arrow function detected!");
        }
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
    }


    Some (())
}