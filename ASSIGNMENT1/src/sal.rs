use crate::*;
use calamine::{DataType, Excel};
use chrono::prelude::*;
use std::collections::HashMap;

fn getmon(s: &str) -> u32 {
    if s == "Jan" {
        1
    } else if s == "Feb" {
        2
    } else if s == "Mar" {
        3
    } else if s == "Apr" {
        4
    } else if s == "May" {
        5
    } else if s == "Jun" {
        6
    } else if s == "Jul" {
        7
    } else if s == "Aug" {
        8
    } else if s == "Sep" {
        9
    } else if s == "Oct" {
        10
    } else if s == "Nov" {
        11
    } else if s == "Dec" {
        12
    } else {
        0
    }
}

pub fn new(pth: &Pth, salmp: &mut HashMap<i32, String>) -> Result<(), std::io::Error> {
    let dt = Utc::now();
    let mut i = 0;
    let mut excel = Excel::open(&pth.s).unwrap();
    let r = excel.worksheet_range("Sheet1").unwrap();
    for row in r.rows() {
        if i == 0 {
            i = 1;
            continue;
        }
        let empid = match row[0] {
            DataType::Float(v) => v,
            _ => 0f64,
        };
        let date = match &row[2] {
            DataType::String(v) => v,
            _ => "",
        };
        let status = match &row[4] {
            DataType::String(v) => v,
            _ => "",
        };
        let mut values = date.split(' ');
        let month = values.next().expect("No month");
        let year = values.next().expect("No year");
        if (year.parse::<i32>().unwrap()) == dt.year() && (getmon(month) == dt.month()) {
            salmp.insert(empid as i32, status.to_string());
        }
    }
    Ok(())
}
