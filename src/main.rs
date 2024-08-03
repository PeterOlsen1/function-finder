mod finders;
mod utils;

use std::io::Result;
use finders::find_single;


fn main() -> Result<()> {
    let filename = "test";
    find_single(filename, "add");

    Ok(())
}
