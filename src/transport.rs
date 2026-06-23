use std::{io::{Read, Write}, net::{TcpStream, UdpSocket}, time::Duration};
use crate::{builder::create_message, encoder::encode_message, errors::DnsError, types::{Message, Type}};

pub fn connect_udp(domain_name: String, record_type: Type) -> Result<(), DnsError> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    socket.set_read_timeout(Some(std::time::Duration::from_secs(3)))?;

    let domain_name2 = domain_name.clone();
    let message = create_message(domain_name, record_type, true);
    let encoded_message = encode_message(&message);

    socket.send_to(&encoded_message, "8.8.8.8:53")?;

    let mut buf = [0u8; 512];

    let (amt, _) = socket.recv_from(&mut buf)?;
    let message_response = Message::try_from(&buf[..amt])?;

    if message.header().id() == message_response.header().id() {
        if message_response.header().tc_bit() {
            println!("Too big fallback to TCP");
            return connect_tcp(domain_name2, record_type)
        }
        println!("{message_response}");
    } else {
        return Err(DnsError::IdMismatch { expected: message.header().id(), got: message_response.header().id() })
    }

    Ok(())
}

fn read_dns_message(stream: &mut TcpStream) -> Result<Message, DnsError> {
    let mut len_buf = [0u8; 2];

    stream.read_exact(&mut len_buf)?;

    let len = u16::from_be_bytes(len_buf) as usize;

    let mut msg_buf = vec![0u8; len];

    stream.read_exact(&mut msg_buf)?;

    let msg = Message::try_from(msg_buf.as_slice())?;

    Ok(msg)
}

fn connect_tcp(domain_name: String, record_type: Type) -> Result<(), DnsError> {
    let mut socket = TcpStream::connect("8.8.8.8:53")?;

    socket.set_read_timeout(Some(Duration::from_secs(3)))?;

    let message = create_message(domain_name, record_type, true);
    let encoded = encode_message(&message);

    let len = (encoded.len() as u16).to_be_bytes();

    socket.write_all(&len)?;
    socket.write_all(&encoded)?;

    let response = read_dns_message(&mut socket)?;

    if message.header().id() == response.header().id() {
        println!("{response}");
    } else {
        return Err(DnsError::IdMismatch { expected: message.header().id(), got: response.header().id() })
    }

    Ok(())
}
