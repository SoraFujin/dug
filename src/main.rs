#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{env::args, net::UdpSocket};
use std::env;
use crate::builder::create_message;
use crate::encoder::encode_message;
use crate::errors::DnsError;
use crate::types::{Message, Type};
use crate::{utils::{read_input, validate_input}};
pub mod utils;
pub mod errors;
pub mod types;
pub mod encoder;
pub mod decoder;
pub mod builder;

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

    match udp(domain_name.to_string()) {
        Ok(()) => (),
        Err(error) => {
            println!("{error}");
        }
    };

}


fn udp(domain_name: String) -> Result<(), DnsError> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    socket.set_read_timeout(Some(std::time::Duration::from_secs(3)))?;

    let message = create_message(domain_name, Type::A, true);
    let encoded_message = encode_message(&message);

    println!("Sending {} bytes to 8.8.8.8:53...", encoded_message.len());

    socket.send_to(&encoded_message, "8.8.8.8:53")?;

    let mut buf = [0u8; 512];

    let (amt, src) = socket.recv_from(&mut buf)?;
    let message = Message::try_from(&buf[..amt])?;

    println!("Received {} bytes back from {}", amt, src);
    // println!("--------------------------------------------------");
    println!("{:?}", message);
    // println!("{:?}", &buf[..amt]);
    // println!("--------------------------------------------------");

    Ok(())
}
