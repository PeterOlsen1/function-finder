use std::fs;
use std::io::Result;



fn main() -> Result<()> {
    let filename = "../testfiles/test.js";
    let contents = fs::read_to_string(filename)?;

    println!("{}", contents);



    Ok(())
}
