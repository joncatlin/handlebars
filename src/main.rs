use std::fs;
use std::collections::HashMap;
use std::io::{self, BufRead};
extern crate regex;

use regex::Regex;
use std::fmt;

fn main() -> std::io::Result<()> {


//    let replace_file = File::open("./fields.data")?;
    let file_contents = fs::read_to_string("./fields.data")?;

    let re = Regex::new(r"^\s*(?<key>[A-Za-z0-9]+):\s*(?<value>.*)$").unwrap();

//    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();

    match re.captures(&file_contents) {
        Ok(caps) => println!("capture groups = {:?}", caps),
        Err(e) => println("error: {}", e),
    };


    // fn extract_login(input: &str) -> Option<&str> {
    //     lazy_static! {
    //         static ref RE: Regex = Regex::new(r"(?x)
    //             ^(?P<login>[^@\s]+)@
    //             ([[:word:]]+\.)*
    //             [[:word:]]+$
    //             ").unwrap();
    //     }
    //     RE.captures(input).and_then(|cap| {
    //         cap.name("login").map(|login| login.as_str())
    //     })
    // }







    // let mut empty = HashMap::new();
    // let account_details = make_hashmap (&mut empty);
    // println!("Hashmap={:?}", account_details);

    // let replace_file = File::open("./data/Google.html")?;
    // let mut buffer = String::new();
    // let result = replace(replace_file,String::from("{{"), String::from("}}"), account_details, &mut buffer);

    Ok(())
}

