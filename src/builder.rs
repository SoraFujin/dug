use std::{hash::{BuildHasher, Hasher, RandomState}};

use crate::types::{Header, Message, Question, Type, Class};

pub fn create_message(domain_name: String, qtype: Type, recurrsion_bit: bool) -> Message {
    let header = create_header(recurrsion_bit);
    let question = Question::new(domain_name, qtype, Class::IN);

    let questions = vec![question];

    Message::new(
        header,
        questions,
        vec![], 
        vec![],
        vec![]
        )
}

fn create_header(recurrsion_bit: bool) -> Header {
    let id = generate_id();
    let flags = if recurrsion_bit {
        1 << 8
    } else {
        0
    };
    Header::new(id, flags, 1, 0, 0, 0)
}

fn generate_id() -> u16 {
    let seed = RandomState::new();
    let mut hasher = seed.build_hasher();
    hasher.write_u8(0);

    let raw_hash = hasher.finish(); 
    raw_hash as u16
}
