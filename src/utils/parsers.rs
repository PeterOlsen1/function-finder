/**
 * @purpose - Helper function to parse the arguments of a function signature
 */
pub fn parse_params(signature: &str) -> Vec<String> {
    //find the parenthesis
    let first_parenthesis = signature
        .find('(')
        .expect("Couldn't Find First Parenthesis");
    let second_parenthesis = signature
        .find(')')
        .expect("Couldn't Find Second Parenthesis");

    let param_string: &str = &signature[first_parenthesis + 1..second_parenthesis];
    let params: Vec<String> = param_string
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    println!("{:?}", params);

    params
}

/**
 * @purpose - Function to get the name from a signature string
 */
pub fn parse_name(signature: &str) -> String {
    let split_line: Vec<String> = signature
        .split("(")
        .map(|s| s.trim().to_string())
        .collect();

    split_line[0].clone()
}