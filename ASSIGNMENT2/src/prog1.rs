use crate::*;

use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;

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
        .open(&pth.o.clone())
        .expect("cannot open output file");

    let mut content = String::new();
    input_file
        .read_to_string(&mut content)
        .expect("Cant read content");
    for line in content.lines() {
        let mut values = line.split(',');
        let lastname = values.next().expect("No lastname");
        let firstname = values.next().expect("No firstname");
        let dob = values.next().expect("No dob");

        let mut out_msg = Person::new();
        out_msg.last_name = lastname.to_string();
        out_msg.first_name = firstname.to_string();
        out_msg.dob = dob.to_string();

        let out_bytes: Vec<u8> = out_msg.write_to_bytes().unwrap_or(Vec::new());
        if out_bytes.len() == 0 {
            continue;
        }
        let data_length = out_bytes.len() as u64;

        let mut content = String::new();
        let mut record = format!("");

        let mut buffer = Vec::new();
        protobuf::CodedOutputStream::vec(&mut buffer)
            .write_raw_varint64(data_length)
            .expect("Failed to write varint-encoded u64");
        output_file.write(&buffer).expect("write failed");

        for byte in &out_bytes {
            record.push_str(&format!(" {}", byte));
        }
        record.push_str(&format!("\n"));
        content.push_str(&record);
        output_file.write(content.as_bytes()).expect("write failed");
    }
}
