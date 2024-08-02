use std::fs;
use std::io::{self, BufRead, Result};
use crate::helpers::{out, out_string_vec};

pub struct Line {
    pub filename: String,
    pub content: String,
    pub idx: u16
}

pub struct Definition {
    pub content: String,
    pub name: String,
    pub idx: u16,
    pub params: Vec<String>
}

pub fn read_lines(filename : &str, name: &str) -> Result<Vec<Line>> {
    //get the filename and open file
    let path = &format!("./testfiles/{}.js", filename);
    let f = fs::File::open(path.clone())?;
    
    //initialize file reader, increment variable, and line store
    let reader = io::BufReader::new(f);
    let mut i: u16 = 0;
    let mut lines: Vec<Line> = Vec::new();
    let mut defs: Vec<Definition> = Vec::new();

    for line in reader.lines() {

        //increment line# and create line variable
        i += 1;
        let line = &line?;

        //check if the line is a function definition
        if line.contains("function") && !line.contains("//") {
            let parts: Vec<&str> = line.split("function").collect();
            out_string_vec(parts);
            // defs.push(Definition {
            //     content:  String::from(parts[1]),
            //     name: String::from(parts[1]),
            //     idx: i,
            //     params: Vec::new(["hi", "hello"])
            // });
        }

        //parse the line
        if parse_line(line, name) {
            //push the line to our store of lines
            lines.push(Line {
                filename : String::from(path),
                content: String::from(line),
                idx: i
            });
        }
    }

    Ok(lines)
}

//function to parse a line, maybe return a line option?
    //if true, send back the entire line
pub fn parse_line(line: &str, name: &str) -> bool {
    if line.contains(&format!("{}(", name)) {
        true
    } 
    else {
        false
    }
}
