use crate::types::Message;

//  The parser needs to extract information from the response bytes and savvees them inside the
//  Message (already built not create a new one), it should match the id from the response to the
//  one it sent
pub fn parser(buffer: Vec<u8>, msg: &Message) {
    let id = msg.header().id();
    println!("{id}");

    for byte in buffer {
    }
}

