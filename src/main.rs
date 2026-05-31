#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::utils::validate_input;
pub mod utils;

fn main() {
    match validate_input("hello.com") {
        Ok(()) => (),
        Err(error) => println!("{error}")
    };
}
