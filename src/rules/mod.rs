use std::fs::File;
use std::io::Read;

use regex::{Regex, Captures};

use crate::ApplicationError;

struct Context {
    
}

fn extract_type_names<'a>() -> Result<Option<Captures<'a>>, ApplicationError> {
    let file = File::open("types.py");
    let re = Regex::new(r"class (.+)\(Entity\)").unwrap();

    

}

pub fn entrypoint() -> Result<(), ApplicationError> {
    todo!()
}