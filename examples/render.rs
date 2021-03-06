#![feature(custom_derive, plugin)]

#![plugin(tojson_macros)]

extern crate env_logger;
extern crate handlebars;
extern crate rustc_serialize;

use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::path::Path;
use std::collections::BTreeMap;

use handlebars::{Handlebars, RenderError, RenderContext, Helper, Context};
use rustc_serialize::json::{Json, ToJson};

#[derive(ToJson)]
struct Team {
    name: String,
    pts: u16
}

fn format_helper (c: &Context, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<String, RenderError> {
    let param = h.params().get(0).unwrap();
    Ok(format!("{} pts", c.navigate(rc.get_path(), param)))
}

fn load_template(name: &str) -> io::Result<String> {
    let path = Path::new(name);

    let mut file = try!(File::open(path));
    let mut s = String::new();
    try!(file.read_to_string(&mut s));
    Ok(s)
}

fn make_data () -> BTreeMap<String, Json> {
    let mut data = BTreeMap::new();

    data.insert("year".to_string(), "2015".to_json());

    let teams = vec![ Team { name: "Jiangsu Sainty".to_string(),
                             pts: 43u16 },
                      Team { name: "Beijing Guoan".to_string(),
                             pts: 27u16 },
                      Team { name: "Guangzhou Evergrand".to_string(),
                             pts: 22u16 },
                      Team { name: "Shandong Luneng".to_string(),
                             pts: 12u16 } ];

    data.insert("teams".to_string(), teams.to_json());
    data
}

fn main() {
    env_logger::init().unwrap();
    let mut handlebars = Handlebars::new();

    let t = load_template("./examples/template.hbs").ok().unwrap();
    handlebars.register_template_string("table", t)
        .ok().expect("template creation failed");

    handlebars.register_helper("format", Box::new(format_helper));
//    handlebars.register_helper("format", Box::new(FORMAT_HELPER));

    let data = make_data();
    println!("{}", handlebars.render("table", &data).ok().unwrap());
}
