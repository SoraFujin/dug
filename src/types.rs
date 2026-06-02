pub struct Message {
    header: Header,
    question: Vec<Question>,
    answer: Vec<RR>,
    authority: Vec<RR>,
    additional: Vec<RR>
}

struct Header {
    id: u16,
    flags: u16,
    qd_count: u16,
    an_count: u16,
    ns_count: u16,
    ar_count: u16
}

struct Question {
    qname: String,
    qtype: Type,
    qclass: Class,
}

struct RR {
    name: String,
    rr_type: Type,
    class: Class,
    ttl: u32,
    rdata: RData,
}


#[repr(u16)]
enum Type {
    A = 1,
    AAAA = 28,
    NS = 2,
    CName = 5,
    Soa = 6,
    Wks = 11,
    Ptr = 12,
    Hinfo = 13,
    Minfo = 14,
    Mx = 15,
    Txt = 16,
}

#[repr(u16)]
enum Class {
    IN = 1,
    CH = 3,
    HS = 4
}

enum RData {
    A(std::net::Ipv4Addr),
    AAAA(std::net::Ipv6Addr),
    CName(String),
    HInfo {
        cpu: String,
        os: String
    },
    MX { 
        preference: u16,
        exchange: String
    },
    NS(String),
    Ptr(String),
    Soa {
        mname: String,
        rname: String,
        serial: u32,
        refresh: u32,
        retry: u32,
        expire: u32,
        minimum: u32
    },
    Txt(String),
}
