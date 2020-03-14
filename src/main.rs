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

// define a custom helper
fn total_amount_due_helper (
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let currency = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for format helper."))?;
    let accounts = h
        .param(1)
        .ok_or(RenderError::new("Param 0 is required for format helper."))?;

    let accs: Vec<Account> = serde_json::from_value(accounts.value().clone()).expect("expected json value");
    let mut total: f64 = 0.0;
    for a in accs {
        total += a.amount_due;
    }

    let rendered = format_money(currency.value().render(), total);

    out.write(rendered.as_ref())?;
    Ok(())
}


// define a custom helper
fn money_fmt_helper (
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let currency = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for format helper."))?;
    let amount = h
        .param(1)
        .ok_or(RenderError::new("Param 0 is required for format helper."))?;
    let amount_float = amount.value().as_f64().unwrap();

    let rendered = format_money(currency.value().render(), amount_float);
    out.write(rendered.as_ref())?;
    Ok(())
}


fn format_money(currency: String, amount: f64) -> String {
    format!("{}{1:.2}", currency, amount)
}

fn main() {
    env_logger::init();
    let mut handlebars = Handlebars::new();

    handlebars.register_helper("total_amount_due", Box::new(total_amount_due_helper));
    handlebars.register_helper("money_fmt", Box::new(money_fmt_helper));

    let data = get_data();

    let path = Path::new("./templates/template1.html");
    handlebars.register_template_file("template1", path).expect("render error");

    // Get the template structure to access the fields within it
    let template1 = handlebars.get_template("template1").unwrap();
    debug!("Temaple1 has fields {:?}", template1);

    // // Print out the mapping information held in the template
    // match &template1.mapping {
    //     Some(map) => {
    //         for entry in map {
    //             ("mapping is: {:?}", entry);
    //         };
    //     },
    //     None => debug!("No mappings found"),
    // }

    // Print out the template elements
    // for e in &template1.elements {
    //     debug!("Element: {:?}\n", e);
    // }

    let mut fields = Vec::<&str>::new();
    get_fields(&template1.elements, fields);
    debug!("ALL FIELDS FOUND: {:?}", fields);

    let path = Path::new("./templates/template2.html");
    handlebars.register_template_file("template2", path).expect("render error");

    let start = Instant::now();

    // For each contract found in the data use the template and generate the result
    for contract in &data {
        let file_name = format!("./output/contract-{}.html", contract.id);
        let mut output_file = File::create(&file_name).expect("file output open error");
        handlebars.render_to_write("template1", &contract, &mut output_file).expect("render error");
    }

    let duration = start.elapsed();

    debug!("Time elapsed for rendering template1 is: {:?}", duration);


    // For each contract found in the data use the template and generated the result
    for contract in &data {
        let file_name = format!("./output/email-{}.html", contract.id);
        let mut output_file = File::create(&file_name).expect("file output open error");
        handlebars.render_to_write("template2", &contract, &mut output_file).expect("render error");
    }

    let duration = start.elapsed();

    debug!("Time elapsed for rendering template2 is: {:?}", duration);







}

#[derive(Serialize, Deserialize, Debug)]
struct Account {
    days_delinquent: i16,
    amount_due: f64,
    account_number: String,
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
    client: String,
    accounts: Vec<Account>,
    currency: String,
}


fn get_data() -> Vec<Contract> {

    let file_contents = fs::read_to_string("./mock_data_full.json").expect("error on read string from file");

    let array: Vec<Contract> = serde_json::from_str(&file_contents).expect("");

    array
}


fn get_fields(elements: &Vec<TemplateElement>, mut fields: Vec<&str>) {

    for e in elements {
        match(e) {
            TemplateElement::Expression(exp) => {
                debug!("Expression: {:?}", exp);
                get_fields_helper_template(&exp, fields)
            },
            TemplateElement::RawString(s) => debug!(""),
            TemplateElement::HTMLExpression(html) => debug!(""),
            TemplateElement::HelperBlock(hb) => {
                debug!("HelperBlock: {:?}", hb);
                get_fields_helper_template(&hb, fields)
            },
            TemplateElement::DecoratorExpression(de) => debug!("DecoratorTemplate: {:?}", de),
            TemplateElement::DecoratorBlock(db) => debug!("DecoratorBlock: {:?}", db),
            TemplateElement::PartialExpression(pe) => debug!("DecoratorTemplte: {:?}", pe),
            TemplateElement::PartialBlock(pb) => debug!("PartialBlock: {:?}", pb),
            TemplateElement::Comment(c) => debug!(""),
        }
        debug!("\n");
    }

}


fn get_fields_helper_template(ht: &HelperTemplate, mut fields: Vec<&str>) {

    get_fields_parameter(&ht.name, fields);
    for param in &ht.params {
        get_fields_parameter(&param, fields)
    }
//    hash: HashMap<String, Parameter>
}


fn get_fields_parameter(p: &Parameter, mut fields: Vec<&str>) {
    match (p) {
        Parameter::Name(s) => debug!("Founf Name: {}\n", s),
        Parameter::Path(path) => {
            debug!("Found Path: {:?} - adding it to list of fields\n", path);
            match path {
                Relative(tup1) => {
                    let (_, var_name) = tup1;
                    println!("RELATIVE with name={}", var_name);
                    fields.push(&var_name.to_string());
                },
                Local(tup2) => println!("LOCAL"),                
            }
//            let (_, name) = path.Relative;
            //     Path::Relative(tup) => tup,
            //     Path::Local(l) => (_, "Error in get_fields_parameter when Path"),
            // };
//            fields.push(path.raw());
        },
        Parameter::Literal(j) => debug!("Found Literal: {:?}", j),
        Parameter::Subexpression(u) => debug!("Found Subexpression: {:?}", u),        
    }
}
