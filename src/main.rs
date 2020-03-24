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

use handlebars::template:: {TemplateElement, HelperTemplate, Parameter, DecoratorTemplate};
use handlebars::Path:: {Relative, Local};
//use handlebars::json::path::PathSeg;

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


fn main() {
    env_logger::init();
    let mut handlebars = Handlebars::new();

    handlebars.register_helper("total_amount_due", Box::new(total_amount_due_helper));
    handlebars.register_helper("money_fmt", Box::new(money_fmt_helper));

    let _data = get_data();

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

    let mut fields = Vec::<String>::new();
    get_fields(&template1.elements, &mut fields);
    debug!("ALL FIELDS FOUND: {:?}", fields);

    // let path = Path::new("./templates/template2.html");
    // handlebars.register_template_file("template2", path).expect("render error");

    // let start = Instant::now();

    // // For each contract found in the data use the template and generate the result
    // for contract in &data {
    //     let file_name = format!("./output/contract-{}.html", contract.id);
    //     let mut output_file = File::create(&file_name).expect("file output open error");
    //     handlebars.render_to_write("template1", &contract, &mut output_file).expect("render error");
    // }

    // let duration = start.elapsed();

    // debug!("Time elapsed for rendering template1 is: {:?}", duration);


    // // For each contract found in the data use the template and generated the result
    // for contract in &data {
    //     let file_name = format!("./output/email-{}.html", contract.id);
    //     let mut output_file = File::create(&file_name).expect("file output open error");
    //     handlebars.render_to_write("template2", &contract, &mut output_file).expect("render error");
    // }

    // let duration = start.elapsed();

    // debug!("Time elapsed for rendering template2 is: {:?}", duration);
}

fn get_fields(elements: &Vec<TemplateElement>, mut fields: &mut Vec<String>) {

    for e in elements {
        get_field_in_template_element(e, &mut fields);        
    }

}


// Scan the TemplateElement struct for fields
fn get_field_in_template_element(element: &TemplateElement, mut fields: &mut Vec<String>) {
    match element  {
        TemplateElement::Expression(exp) => {
            debug!("Expression: {:?}", exp);
            get_fields_in_helper(&exp, &mut fields)
        },
        TemplateElement::RawString(_s) => (),
        TemplateElement::HTMLExpression(_html) => (),
        TemplateElement::HelperBlock(hb) => {
            debug!("HelperBlock: {:?}", hb);
            get_fields_in_helper(&hb, &mut fields)
        },
        TemplateElement::DecoratorExpression(d) |
        TemplateElement::DecoratorBlock(d) |
        TemplateElement::PartialExpression(d) |
        TemplateElement::PartialBlock(d) => {
            debug!("DecoratorExpression or DecoratorBlock or PartialBlock or PartialExpression: {:?}", d);
            get_fields_in_decorator(&d, &mut fields)
        },
        TemplateElement::Comment(_c) => (),
    }
}



// Scan the HelperTemplate struct for fields
fn get_fields_in_helper(ht: &HelperTemplate, mut fields: &mut Vec<String>) {

    get_fields_parameter(&ht.name, &mut fields);
    for param in &ht.params {
        get_fields_parameter(&param, &mut fields)
    }
}


// Scan the DecoratorTemplate struct for fields
fn get_fields_in_decorator(ht: &DecoratorTemplate, mut fields: &mut Vec<String>) {

    get_fields_parameter(&ht.name, &mut fields);
    for param in &ht.params {
        get_fields_parameter(&param, &mut fields)
    }
}


fn get_fields_parameter(p: &Parameter, mut fields: &mut Vec<String>) {
    match p {
        Parameter::Name(s) => debug!("Found Name: {}\n", s),
        Parameter::Path(path) => {
            debug!("Found Path: {:?} - adding it to list of fields\n", path);
            match path {
                Relative(tup1) => {
                    let (path_seg, var_name) = tup1;
                    info!("RELATIVE with name={} tuple={:?}", var_name, tup1);
                    fields.push(var_name.clone());
//                    get_fields_in_path_seg(path_seg);
                },
                Local(tup2) => debug!("Found Local: {:?}\n", tup2),                
            }
        },
        Parameter::Literal(j) => debug!("Found Literal: {:?}", j),
        Parameter::Subexpression(u) => {
            debug!("Found Subexpression: {:?}", u);
            get_field_in_template_element(&u.element, &mut fields); 
        },
    }
}


fn get_fields_in_path_seg(ps: Vec<()>) {

}