mod read;
mod helpers;

use std::io::Result;
use helpers::out;
use read::{read_lines, Line};


fn main() -> Result<()> {
    let filename = "test";
    let result: Vec<Line> = read_lines(filename, "greet")?;

    for line in result {
        
    }



    Ok(())
}
