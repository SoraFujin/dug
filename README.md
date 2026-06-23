# dug

A DNS client written in Rust from scratch. No external crates — raw UDP/TCP sockets, binary packet construction and parsing per [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035).

---

## Features

- Queries A, AAAA, NS, CNAME, SOA, PTR, HINFO, MX, and TXT records
- Builds and parses the DNS wire format by hand, byte by byte
- Sends over UDP and **falls back to TCP** automatically when a response is truncated (TC bit set)
- Follows DNS name compression pointers, with a loop guard against malformed packets
- Validates responses: query/response ID match and per-record RDATA length checks
- `dig`-style output
- Resolves against `8.8.8.8` by default
- Zero dependencies — standard library only

## Installation

```sh
git clone https://github.com/SoraFujin/dug
cd dug
cargo build --release
```

## Usage

```sh
dug <domain> [record type]
```

The record type is optional and case-insensitive; it defaults to `A`.

```sh
dug google.com          # defaults to A
dug google.com AAAA
dug google.com MX
dug google.com NS
dug google.com SOA
dug google.com TXT
```

Example output:

```
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 64567
;; flags: qr rd ra; QUERY: 1, ANSWER: 1, AUTHORITY: 0, ADDITIONAL: 0

;; QUESTION SECTION:
;google.com		IN	MX

;; ANSWER SECTION:
google.com	66	IN	MX	10 smtp.google.com
```

## How it works

`dug` constructs a DNS query message in binary, sends it over a UDP socket to `8.8.8.8:53`, then reads and parses the response byte by byte against the wire format. No resolver libraries, no `getaddrinfo` — just the socket and the spec.

A response under 512 bytes arrives over UDP. If the server sets the truncation (TC) bit, `dug` retransmits the same query over TCP — which prefixes each message with a 2-byte length — and parses the larger response with the same decoder.

The parser is built around a `Cursor` over the raw byte buffer, with bounds-checked readers for the integer fields, name decoding that follows compression pointers (rejecting forward/cyclic jumps), and a length-validation pass that confirms each record's RDATA consumed exactly the bytes its header declared.

## Scope

Built as a learning project. Intentionally not implemented: EDNS0/OPT, DNSSEC, custom resolver selection, and IDN/punycode handling. Record text is decoded as ASCII.
