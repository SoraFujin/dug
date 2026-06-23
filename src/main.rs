use std::env::args;
use crate::transport::connect_udp;
use crate::types::Type;
use crate::utils::validate_input;
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
        println!("Error Usage: dug <domain-name> <Record-Type>");
        return
    }

    let domain_name = match args.get(1) {
        Some(name) => name,
        None => {
            println!("Error getting the name");
            return
        }
    };

    let record_type = match args.get(2) {
        Some(record) => match Type::get_type(record.to_uppercase().as_str()) {
            Ok(record) => record,
            Err(error) => {
                println!("Error converting the record type {error}");
                return
            }
        },
        None => Type::A
    };


    match validate_input(domain_name) {
        Ok(()) => (), 
        Err(error) =>{
            println!("{error}");
            return
        }
    };

    match connect_udp(domain_name.to_string(), record_type) {
        Ok(()) => (),
        Err(error) => {
            println!("{error}");
        }
    };
}
