use std::{net::TcpStream, io::Write};
use pwn_helper::io::PwnIoRead;
use serde_json::Value;
use ascii::AsciiString;

fn to_u8_vec(v : &Vec<Value>) -> Vec<u8>
{
    v.iter().map(|f| u8::try_from(f.as_i64().unwrap()).unwrap()).collect()
}

pub fn utf8(text: Vec<u8>) -> String {
    String::from_utf8(text).expect("Failed to parse utf8 string")
}

pub fn rot13(text: &str) -> String {
    text.chars().map(|c| {
        match c {
            'A' ..= 'M' | 'a' ..= 'm' => ((c as u8) + 13) as char,
            'N' ..= 'Z' | 'n' ..= 'z' => ((c as u8) - 13) as char,
            _ => c
        }
    }).collect()
}

pub fn hex(text: &str) -> String {
    let bytes = hex::decode(text).expect("Failed to parse hex string");
    ascii::AsciiStr::from_ascii(&bytes).expect("Failed to parse hex ascii string").to_string()
}
pub fn base64(text: &str) -> String {
    let bytes = base64::decode(text).expect("Failed to parse base64 string");
    ascii::AsciiStr::from_ascii(&bytes).expect("Failed to parse b64 ascii string").to_string()
}

pub fn big_int(text: &str) -> String {
    let hex = text.strip_prefix("0x").unwrap();
    let bint = num_bigint::BigUint::parse_bytes(hex.as_bytes(), 16).expect("Failed to parse hex bigint");
    ascii::AsciiStr::from_ascii(&bint.to_bytes_be()).expect("Failed to parse bigint ascii string").to_string()
}

fn main() {
    let mut remote = TcpStream::connect("socket.cryptohack.org:13377").expect("Failed to connect to shell");
    loop {
        let line = remote.receive_line(false).expect("Failed to read line from stream");
        let line = AsciiString::from_ascii(line).expect("Failed to parse ascii string");
        let v: Value = serde_json::from_str(line.as_str()).expect("Failed to parse json");
        println!("Received: {line}");
        let result = match (&v["type"].as_str(), &v["encoded"], v["flag"].as_str()) {
            (_,_, Some(flag)) => 
            { 
                println!("FLAG: {flag}"); 
                break; 
            }
            (Some("utf-8"), Value::Array(a),_) =>  utf8(to_u8_vec(a)),
            (Some("rot13"), Value::String(s),_) =>  rot13(s),
            (Some("hex"), Value::String(s),_) =>  hex(s),
            (Some("base64"), Value::String(s),_) =>  base64(s),
            (Some("bigint"), Value::String(s),_) =>  big_int(s),
            _ => panic!("Not implemented type")
        };
        let result = format!("{{\"decoded\": \"{result}\"}}");
        println!("Sent: {result}");
        remote.write_all(result.as_bytes());
    }
}
