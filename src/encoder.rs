use crate::types::Message;

pub fn encode_message(msg: &Message) -> Vec<u8> {
    let mut packet: Vec<u8> = Vec::new();
    let header_bytes = msg.header().to_bytes();
    packet.extend_from_slice(&header_bytes);  

    for question in msg.question() {
        let question_bytes = question.to_bytes();
        packet.extend_from_slice(&question_bytes);  
    }

    for answer in msg.answers() {
        let answer_bytes = answer.to_bytes();
        packet.extend_from_slice(&answer_bytes);
    }

    for auth in msg.authority() {
        let auth_bytes = auth.to_bytes();
        packet.extend_from_slice(&auth_bytes);
    }

    for additional in msg.additional() {
        let additional_bytes = additional.to_bytes();
        packet.extend_from_slice(&additional_bytes);
    }

    packet
}
