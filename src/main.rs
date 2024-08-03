mod finders;
mod utils;

use std::io::Result;
use finders::find_single;
use std::env;


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    //two extra arguments, filename and function to find?
    //or also maybe unsupported features so far
    if args.len() == 3 {
        //check if they want to find in a js file
        match args[1].as_str() {
            "findall" if args[2].ends_with(".js") => println!("Find all occurences of some function in all files!"),
            _ if args[2].ends_with(".js") => find_single(&args[2], &args[1])?,
            _ => println!("End of the pattern"),
        }
    }

    // let filename = "test";
    // find_single(filename, "add");

    Ok(())
}
