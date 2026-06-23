use std::net::{Ipv4Addr, Ipv6Addr};
use crate::{errors::DnsError, types::{Class, Header, Message, Question, RData, RR, Type}};

struct Cursor<'a> {
    position: usize,
    buf: &'a [u8],
}

impl<'a> Cursor<'a> {
    fn read_u32(&mut self) -> Result<u32, DnsError> {
        if self.position + 3 >= self.buf.len() {
            Err(DnsError::OutOfBytes {
                needed: 4,
                available: self.buf.len() - self.position
            })
        } else {
            let byte0 = self.buf[self.position];
            let byte1 = self.buf[self.position + 1];
            let byte2 = self.buf[self.position + 2];
            let byte3 = self.buf[self.position + 3];
            let mut data = (byte0 as u32) << 24;
            data += (byte1 as u32) << 16;
            data += (byte2 as u32) << 8;
            data += byte3 as u32;
            self.position += 4;
            Ok(data)
        }
    }

    fn read_u16(&mut self) -> Result<u16, DnsError> {
        if self.position + 1 >= self.buf.len() {
            Err(DnsError::OutOfBytes {
                needed: 2,
                available: self.buf.len() - self.position
            })
        } else { 
            let top_byte = self.buf[self.position];
            let bottom_byte = self.buf[self.position + 1];
            let mut data = (top_byte as u16) << 8;
            data += bottom_byte as u16;
            self.position += 2;
            Ok(data)
        }
    }

    fn read_u8(&mut self) -> Result<u8, DnsError> {
        if self.position >= self.buf.len() {
            Err(DnsError::OutOfBytes {
                needed: 1,
                available: self.buf.len() - self.position
            })
        } else {
            let data = self.buf[self.position];
            self.position += 1;
            Ok(data)
        }
    }

    fn read_name(&mut self) -> Result<String, DnsError> {
        let mut name = String::new();
        let mut jumped = false;
        let mut resume = 0;
        let mut last_jump = self.buf.len();
        loop {
            let first = self.read_u8()?;
            // CASE 1: Pointer
            if first & 0xC0 == 0xC0 {
                let second = self.read_u8()?;
                let offset = (((first & 0x3F) as usize) << 8) | (second as usize);
                if offset < last_jump {
                    if !jumped {
                        resume = self.position;
                        jumped = true;
                    }
                    self.position = offset;
                    last_jump = offset;
                    continue; 
                } else {
                    return Err(DnsError::PointerLoop)
                }
            } 

            // CASE 2: Terminator
            if first == 0 {
                break;
            }

            // CASE 3: normal label:
            if !name.is_empty() {
                name.push('.');
            }
            for _ in 0..first {
                let byte = self.read_u8()?;
                name.push(byte as char);
            }

        } 

        if jumped {
            self.position = resume;
        }

        Ok(name)
    }


    fn read_character_string(&mut self) -> Result<String, DnsError> {
        let len = self.read_u8()?;
        let mut s = String::new();
        for _ in 0..len {
            let byte = self.read_u8()?;
            s.push(byte as char);
        }
        Ok(s)
    }

    fn read_rdata(&mut self, rtype: Type) -> Result<RData, DnsError>{
        match rtype {
            Type::A => {
                let a = self.read_u32()?;
                Ok(RData::A(Ipv4Addr::from(a)))
            },
            Type::AAAA => {
                let a = self.read_u32()?;
                let b = self.read_u32()?;
                let c = self.read_u32()?;
                let d = self.read_u32()?;
                let aaaa = ((a as u128) << 96) | ((b as u128) << 64) | ((c as u128) << 32) | (d as u128); 
                Ok(RData::AAAA(Ipv6Addr::from(aaaa)))
            },
            Type::CNAME => {
                let name = self.read_name()?;
                Ok(RData::CNAME(name))
            },
            Type::HINFO => {
                let cpu = self.read_character_string()?;
                let os = self.read_character_string()?;
                Ok(RData::HINFO { cpu, os })
            },
            Type::MX => {
                let preference = self.read_u16()?;
                let exchange = self.read_name()?;
                Ok(RData::MX { preference, exchange })
            },
            Type::NS => Ok(RData::NS(self.read_name()?)),
            Type::PTR => Ok(RData::PTR(self.read_name()?)),
            Type::SOA => {
                let mname = self.read_name()?;
                let rname = self.read_name()?;
                let serial = self.read_u32()?;
                let refresh = self.read_u32()?;
                let retry = self.read_u32()?;
                let expire = self.read_u32()?;
                let minimum = self.read_u32()?;
                Ok(RData::SOA { mname, rname, serial, refresh, retry, expire, minimum })
            },
            Type::TXT => Ok(RData::TXT(self.read_character_string()?)),
        }
    }
}

impl TryFrom<u16> for Type {
    type Error = DnsError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::A),
            2 => Ok(Self::NS),
            5 => Ok(Self::CNAME),
            6 => Ok(Self::SOA),
            12 => Ok(Self::PTR),
            13 => Ok(Self::HINFO),
            15 => Ok(Self::MX),
            16 => Ok(Self::TXT),
            28 => Ok(Self::AAAA),
            _ => Err(DnsError::UnknownType(value))
        }
    }
}

impl TryFrom<u16> for Class {
    type Error = DnsError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::IN),
            3 => Ok(Self::CH),
            4 => Ok(Self::HS),
            _ => Err(DnsError::UnknownClass(value))
        }
    }

}

impl TryFrom<&[u8]> for Message {
    type Error = DnsError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut cursor =  Cursor { position: 0, buf: value };
        let header_id = cursor.read_u16()?;
        let header_flags = cursor.read_u16()?;
        let header_qd_count = cursor.read_u16()?;
        let header_an_count = cursor.read_u16()?;
        let header_ns_count = cursor.read_u16()?;
        let header_ar_count = cursor.read_u16()?;
        let header = Header::new(header_id, header_flags, header_qd_count, header_an_count, header_ns_count, header_ar_count);

        let mut questions: Vec<Question> = Vec::new();
        for _ in 0..header_qd_count {
            let question_name = cursor.read_name()?;
            let question_type = cursor.read_u16()?;
            let qtype = Type::try_from(question_type)?;
            let question_class = cursor.read_u16()?;
            let qclass = Class::try_from(question_class)?;
            let question = Question::new(question_name, qtype, qclass);
            questions.push(question);
        }

        let mut answers: Vec<RR> = Vec::new();
        for _ in 0..header_an_count {
            let answer = read_rr(&mut cursor)?;
            answers.push(answer);
        }

        let mut authorities: Vec<RR> = Vec::new();
        for _ in 0..header_ns_count {
            let authority = read_rr(&mut cursor)?;
            authorities.push(authority);
        }

        let mut additional: Vec<RR> = Vec::new();
        for _ in 0..header_ar_count {
            let addition = read_rr(&mut cursor)?;
            additional.push(addition);
        }

        Ok(Message::new(header, questions, answers, authorities, additional))
    }

}

fn read_rr(cursor: &mut Cursor) -> Result<RR, DnsError> {
    let rr_name = cursor.read_name()?;
    let rdata_type = cursor.read_u16()?;
    let rr_type = Type::try_from(rdata_type)?;
    let rdata_class = cursor.read_u16()?;
    let class = Class::try_from(rdata_class)?;
    let rr_ttl = cursor.read_u32()?;
    let rr_length = cursor.read_u16()?;
    let start = cursor.position;
    let rdata = cursor.read_rdata(rr_type)?;
    let position = cursor.position;
    if position - start != rr_length as usize {
        return Err(DnsError::InvalidRdataLength { record_type: rr_type, length: rr_length })
    }
    Ok(RR::new(rr_name, rr_type, class, rr_ttl, rdata))
}
