/**
 * The purpose of this module is to provide single line parsers to extract data or 
 */


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