#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{env::args, net::UdpSocket};
use std::env;
use crate::builder::create_message;
use crate::encoder::encode_message;
use crate::errors::DnsError;
use crate::transport::connect_udp;
use crate::types::{Message, Type};
use crate::{utils::{read_input, validate_input}};
pub mod utils;
pub mod errors;
pub mod types;
pub mod encoder;
pub mod decoder;
pub mod builder;
pub mod transport;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Error Usage: dug <domain-name>");
        return
    }

    let domain_name = match args.get(1) {
        Some(name) => name,
        None => {
            println!("Error getting the name");
            return
        }
    };

    match validate_input(domain_name) {
        Ok(()) => (), 
        Err(error) =>{
            println!("{error}");
            return
        }
    };

    match connect_udp(domain_name.to_string()) {
        Ok(()) => (),
        Err(error) => {
            println!("{error}");
        }
    };
}
