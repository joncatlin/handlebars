#![allow(unused_imports, dead_code)]
extern crate env_logger;
extern crate handlebars;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use serde::Serialize;
use serde_json::value::{self, Map, Value as Json};
use std::path::Path;

use std::error::Error;

//use serde_json::Error;

use std::fs::File;
use std::io::{Read, Write};

use handlebars::{
    to_json, Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError,
};

use std::fs;








// define a custom helper
fn format_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for format helper."))?;
    let rendered = format!("{} pts", param.value().render());
    out.write(rendered.as_ref())?;
    Ok(())
}

// another custom helper
fn rank_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let rank = h
        .param(0)
        .and_then(|ref v| v.value().as_u64())
        .ok_or(RenderError::new(
            "Param 0 with u64 type is required for rank helper.",
        ))? as usize;
    let total = h
        .param(1)
        .as_ref()
        .and_then(|v| v.value().as_array())
        .map(|arr| arr.len())
        .ok_or(RenderError::new(
            "Param 1 with array type is required for rank helper",
        ))?;
    if rank == 0 {
        out.write("champion")?;
    } else if rank >= total - 2 {
        out.write("relegation")?;
    } else if rank <= 2 {
        out.write("acl")?;
    }
    Ok(())
}

static TYPES: &'static str = "serde_json";

// define some data
#[derive(Serialize)]
pub struct Team {
    name: String,
    pts: u16,
}

// produce some data
pub fn make_data() -> Map<String, Json> {
    let mut data = Map::new();

    data.insert("year".to_string(), to_json("2015"));

    let teams = vec![
        Team {
            name: "Jiangsu Suning".to_string(),
            pts: 43u16,
        },
        Team {
            name: "Shanghai SIPG".to_string(),
            pts: 39u16,
        },
        Team {
            name: "Hebei CFFC".to_string(),
            pts: 27u16,
        },
        Team {
            name: "Guangzhou Evergrand".to_string(),
            pts: 22u16,
        },
        Team {
            name: "Shandong Luneng".to_string(),
            pts: 12u16,
        },
        Team {
            name: "Beijing Guoan".to_string(),
            pts: 7u16,
        },
        Team {
            name: "Hangzhou Greentown".to_string(),
            pts: 7u16,
        },
        Team {
            name: "Shanghai Shenhua".to_string(),
            pts: 4u16,
        },
    ];

    data.insert("teams".to_string(), to_json(&teams));
    data.insert("engine".to_string(), to_json(TYPES));
    data
}

// fn main() -> Result<(), Box<dyn Error>> {
//     env_logger::init();
//     let mut handlebars = Handlebars::new();

//     handlebars.register_helper("format", Box::new(format_helper));
//     handlebars.register_helper("ranking_label", Box::new(rank_helper));
//     // handlebars.register_helper("format", Box::new(FORMAT_HELPER));

//     let data = make_data();

// //    let mut source_template = File::open(&"./examples/render_file/template.hbs")?;
//     let mut source_template = File::open(&"./template.hbs")?;
//     let mut output_file = File::create("./table.html")?;
//     handlebars.render_template_source_to_write(&mut source_template, &data, &mut output_file)?;
//     println!("./table.html generated");
//     Ok(())
// }

fn main() {
    env_logger::init();
    let mut handlebars = Handlebars::new();

//     handlebars.register_helper("format", Box::new(format_helper));
//     handlebars.register_helper("ranking_label", Box::new(rank_helper));
//     // handlebars.register_helper("format", Box::new(FORMAT_HELPER));

//     let data = make_data();
    let data = make_data2();

    let path = Path::new("./templates/template1.html");
    handlebars.register_template_file("jon", path).expect("render error");

    for contract in data {
        let file_name = format!("./output/contract-{}.html", contract.id);
        let mut output_file = File::create(&file_name).expect("file output open error");
        handlebars.render_to_write("jon", &contract, &mut output_file).expect("render error");

        break;
    }
}




#[derive(Serialize, Deserialize, Debug)]
struct Contract {
    id: i16,
    first_name: String,
    last_name: String,
    gender: String,
    email: String,
    address1: String,
    address2: String,
    address3: String,
    city: String,
    state: String,
    zip: String,
    days_delinquent: i16,
    amount_due: f32,
    client: String,
    account_number: String,
}


fn make_data2() -> Vec<Contract> {

    let file_contents = fs::read_to_string("./mock_data.json").expect("error on read string from file");

    // let json: serde_json::Value =
    //     serde_json::from_str(file_contents).expect("JSON was not well-formatted");

    let array: Vec<Contract> = serde_json::from_str(&file_contents).expect("");

//    println!("Vec=:\n{:?}", array);

    array
}
