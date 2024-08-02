pub fn out(line: &str) -> Option<()> {
    println!("{}", line);
    None
}

pub fn out_string_vec(vec : Vec<&str>) -> Option<()> {
    println!("New Vector!");
    for item in vec {
        println!("{}", item);
    }
    println!("");
    None
}