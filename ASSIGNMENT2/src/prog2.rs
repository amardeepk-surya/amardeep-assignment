use crate::*;

use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;

use protobuf::CodedInputStream;
use protobuf::Message;
include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
use Persons::Person;

pub fn function(pth: &Pth) {
    let mut input_file = OpenOptions::new()
        .read(true)
        .open(&pth.i)
        .expect("Cant open input file");
    let mut output_file = OpenOptions::new()
        .append(true)
        .open(&pth.o)
        .expect("cannot open output file");
    let mut content = String::new();
    input_file
        .read_to_string(&mut content)
        .expect("Cant read content");
    let mut contentp = String::new();
    for line in content.lines() {
        let mut values = line.split(' ');

        let mut input = CodedInputStream::from_bytes(&mut values.next().unwrap_or("").as_bytes());
        let len = input
            .read_raw_varint64()
            .expect("Failed to read varint length");

        let mut out_bytes: Vec<u8> = Vec::new();

        for _i in 1..=len {
            let temp = values.next().expect("No byte");
            out_bytes.push(temp.parse::<u8>().unwrap_or(0u8));
        }
        let in_msg = Person::parse_from_bytes(&out_bytes).unwrap_or(Person::new());
        contentp.push_str(in_msg.last_name.as_str());
        contentp.push_str(",");
        contentp.push_str(in_msg.first_name.as_str());
        contentp.push_str(",");
        contentp.push_str(in_msg.dob.as_str());
        contentp.push_str("\n");
    }
    output_file
        .write(contentp.as_bytes())
        .expect("write failed");
}
