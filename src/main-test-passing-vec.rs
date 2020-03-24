#![allow(unused_imports, dead_code)]
extern crate env_logger;
#[macro_use] 
extern crate log;
extern crate handlebars;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use serde::Serialize;
use serde_json::value::{self, Map, Value as Json};
use std::path::Path;

use std::error::Error;

use std::time::{Duration, Instant};

//use serde_json::Error;

use std::fs::File;
use std::io::{Read, Write};

use handlebars::{
    to_json, Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError,
};

use handlebars::template:: {TemplateElement, HelperTemplate, Parameter};
use handlebars::Path:: {Relative, Local};


use std::fs;



fn main() {
    let mut fields = Vec::<&str>::new();
    let data = vec!("hello", "world");

    get_fields(&mut fields, &data);

    println!("fields: {:?}", fields);
}


fn get_fields<'a>(fields: &mut Vec::<&'a str>, data: &Vec::<&'a str>) {

    fields.push("jon");

    for e in data {
        fields.push(e);
    }
}