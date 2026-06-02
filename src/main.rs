#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::{utils::{read_input, validate_input}};
pub mod utils;
pub mod types;

fn main() {
    let domain_name: String = match read_input("Enter a domain name") {
        Some(name) => name,
        None => return
    };

    match validate_input(&domain_name) {
        Ok(()) => (),
        Err(error) => println!("{error}")
    };
}
