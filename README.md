# dug

A minimal DNS client written in Rust from scratch. No external crates — raw UDP socket, binary packet construction and parsing per [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035).

> Work in progress.

---

## Features

- Query A, AAAA, MX, TXT, and CNAME records
- Speaks raw DNS wire format over UDP port 53
- Resolves against `8.8.8.8` by default
- Zero dependencies

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

```sh
dug google.com          # defaults to A record
dug google.com AAAA
dug google.com MX
dug google.com TXT
dug google.com CNAME
```

Example output:

```
google.com.     A       142.250.185.78    TTL 300
google.com.     A       142.250.185.46    TTL 300
```

## How it works

`dug` constructs a DNS query message in binary, sends it over a raw UDP socket to `8.8.8.8:53`, reads the response, and parses it byte by byte according to the DNS wire format spec. No libc wrappers, no resolver libraries — just the socket and the spec.
