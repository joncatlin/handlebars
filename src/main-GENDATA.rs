use std::fs;
use std::collections::HashMap;
use std::io::{self, BufRead};
extern crate regex;
use std::path::Path;
use std::fs::File;
use regex::Regex;
use std::fmt;
use std::io::prelude::*;

use std::io::{BufWriter, Write};


fn main() -> std::io::Result<()> {


    let file_contents = fs::read_to_string("./fields.data")?;

    let re = Regex::new(r"(?m)^\s*(?P<key>[A-Za-z0-9]+):(?P<value>.*)$").unwrap();

    // let mut key_val_list = Vec::<(String, String)>::new();
    
    
    let path = Path::new("./fields.json");

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create file err={}", why),
        Ok(file) => file,
    };
    let mut writer = BufWriter::new(&file);

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
//    file.write_all("Hello jon23".as_bytes())?;
    // {
    //     Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
    //     Ok(_) => println!("successfully wrote to {}", display),
    // }



//     let mut json_str = String::with_capacity(50000);

    write!(writer, "{{\n");

    for caps in re.captures_iter(&file_contents) {
        write!(writer, "\t\"{}\":\"{}\",\n", &caps["key"], &caps["value"].trim());
    }
    write!(writer, "}}\n");

    Ok(())
}

