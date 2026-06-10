use std::{fmt::Display, net::{Ipv4Addr, Ipv6Addr}};

#[derive(Debug)]
pub struct Message {
    header: Header,
    question: Vec<Question>,
    answer: Vec<RR>,
    authority: Vec<RR>,
    additional: Vec<RR>
}

impl Message {
    pub fn new(
        header: Header, 
        question: Vec<Question>,
        answer: Vec<RR>, 
        authority: Vec<RR>, 
        additional: Vec<RR>
    ) -> Self {
        Self { header, question, answer, authority, additional }
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn question(&self) -> &[Question] {
        &self.question
    }

    pub fn answers(&self) -> &[RR] {
        &self.answer
    }

    pub fn authority(&self) -> &[RR] {
        &self.authority
    }

    pub fn additional(&self) -> &[RR] {
        &self.additional
    }
}

#[derive(Debug)]
pub struct Header {
    id: u16,
    flags: u16,
    qd_count: u16,
    an_count: u16,
    ns_count: u16,
    ar_count: u16
}

impl Header {
    pub fn new(id: u16, flags: u16, qd_count: u16, an_count: u16, ns_count: u16, ar_count: u16) -> Self {
        Self { id, flags, qd_count, an_count, ns_count, ar_count }
    }

    pub fn to_bytes(&self) -> [u8; 12] {
        let id_bytes = self.id.to_be_bytes();
        let flags_bytes = self.flags.to_be_bytes();
        let qd_count_bytes = self.qd_count.to_be_bytes();
        let an_count_bytes = self.an_count.to_be_bytes();
        let ns_count_bytes = self.ns_count.to_be_bytes();
        let ar_count_bytes = self.ar_count.to_be_bytes();

        [
            id_bytes[0], id_bytes[1],
            flags_bytes[0], flags_bytes[1],
            qd_count_bytes[0], qd_count_bytes[1],
            an_count_bytes[0], an_count_bytes[1],
            ns_count_bytes[0], ns_count_bytes[1],
            ar_count_bytes[0], ar_count_bytes[1],
        ]
    }

    // I only need to access the ID in order to match and check if the response for the same
    // query/question
    pub fn id(&self) -> u16 {
        self.id
    }
}

#[derive(Debug)]
pub struct Question {
    pub qname: String,
    pub qtype: Type,
    pub qclass: Class,
}

impl Question {
    pub fn new(qname: String, qtype: Type, qclass: Class) -> Self {
        // Plain string storage, no extra allocations, completely safe.
        Self { qname, qtype, qclass }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // Encodes directly into a fresh vector at transmission time
        let mut bytes = encode_dns_name(&self.qname);

        let qtype_bytes = self.qtype.to_bytes();
        let qclass_bytes = self.qclass.to_bytes();

        bytes.extend_from_slice(&qtype_bytes);
        bytes.extend_from_slice(&qclass_bytes);

        bytes
    }
}

#[derive(Debug)]
pub struct RR {
    name: String,
    rr_type: Type,
    class: Class,
    ttl: u32,
    // rdata length no need to be added here 
    rdata: RData,
}

impl RR {
    pub fn new(name: String, rr_type: Type, class: Class, ttl: u32, rdata: RData) -> Self {
        Self { name, rr_type, class, ttl, rdata }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = encode_dns_name(&self.name);

        let rr_type_bytes = self.rr_type.to_bytes();
        bytes.extend_from_slice(&rr_type_bytes);

        let class_bytes = self.class.to_bytes();
        bytes.extend_from_slice(&class_bytes);

        let ttl_bytes = self.ttl.to_be_bytes();
        bytes.extend_from_slice(&ttl_bytes);

        let rdata_bytes = self.rdata.to_bytes();
        let rd_length = rdata_bytes.len() as u16;

        bytes.extend_from_slice(&rd_length.to_be_bytes());
        bytes.extend_from_slice(&rdata_bytes);

        bytes
    }

    pub fn rdata(&self) -> &RData {
        &self.rdata
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
#[allow(clippy::upper_case_acronyms)]
pub enum Type {
    A = 1,
    AAAA = 28,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    PTR = 12,
    HINFO = 13,
    MX = 15,
    TXT = 16,
}

impl Type {
    pub fn to_bytes(&self) -> [u8; 2] {
        let value = *self as u16;
        value.to_be_bytes()
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::A => write!(f, "A"),
            Type::AAAA => write!(f, "AAAA"),
            Type::NS => write!(f, "NS"),
            Type::CNAME => write!(f, "CNAME"),
            Type::SOA => write!(f, "SOA"),
            Type::PTR => write!(f, "PTR"),
            Type::HINFO => write!(f, "HINFO"),
            Type::MX => write!(f, "MX"),
            Type::TXT => write!(f, "TXT")
        }
    }
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Class {
    IN = 1,
    CH = 3,
    HS = 4
}

impl Class {
    pub fn to_bytes(&self) -> [u8; 2] {
        let value = *self as u16;
        value.to_be_bytes()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum RData {
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
    CNAME(String),
    HINFO {
        cpu: String,
        os: String
    },
    MX { 
        preference: u16,
        exchange: String
    },
    NS(String),
    PTR(String),
    SOA {
        mname: String,
        rname: String,
        serial: u32,
        refresh: u32,
        retry: u32,
        expire: u32,
        minimum: u32
    },
    TXT(String),
}

impl RData {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        match self {
            RData::A(ip) => bytes.extend_from_slice(&ip.octets()),
            RData::AAAA(ip) => bytes.extend_from_slice(&ip.octets()),
            RData::CNAME(s) | RData::NS(s) | RData::PTR(s) => {
                bytes.extend(encode_dns_name(s));
            },
            RData::TXT(s) => {
                bytes.push(s.len() as u8);
                bytes.extend_from_slice(s.as_bytes());
            }
            RData::HINFO { cpu, os } => {
                bytes.push(cpu.len() as u8);
                bytes.extend_from_slice(cpu.as_bytes());
                bytes.push(os.len() as u8);
                bytes.extend_from_slice(os.as_bytes());
            }
            RData::MX { preference, exchange } => {
                bytes.extend_from_slice(&preference.to_be_bytes());
                bytes.extend(encode_dns_name(exchange));
            }
            RData::SOA { mname, rname, serial, refresh, retry, expire, minimum } => {
                bytes.extend(encode_dns_name(mname));
                bytes.extend(encode_dns_name(rname));

                bytes.extend_from_slice(&serial.to_be_bytes());
                bytes.extend_from_slice(&refresh.to_be_bytes());
                bytes.extend_from_slice(&retry.to_be_bytes());
                bytes.extend_from_slice(&expire.to_be_bytes());
                bytes.extend_from_slice(&minimum.to_be_bytes());
            }
        }

        bytes
    }
}

fn encode_dns_name(name: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for label in name.split('.') {
        bytes.push(label.len() as u8);
        bytes.extend_from_slice(label.as_bytes());
    }
    bytes.push(0x00);
    bytes
}
