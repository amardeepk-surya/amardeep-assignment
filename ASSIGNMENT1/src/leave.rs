use calamine::{Excel, DataType};
use std::collections::HashMap;
use crate::*;
use chrono::prelude::*;
use chrono::{Duration, NaiveDate};

fn date_from_float(dte:i64)->NaiveDate {
    let start = NaiveDate::from_ymd_opt(1900, 1, 1).expect("DATE");
    let date = start.checked_add_signed(Duration::days(dte-2));
    date.unwrap()
}

pub fn new(pth:&Pth,lvmp:&mut HashMap<i32,i32>)  -> Result<(), std::io::Error> 
{
    let dt = Utc::now();
    let mut excel = Excel::open(&pth.l).unwrap();
    let r = excel.worksheet_range("Sheet1").unwrap();
    for row in r.rows() {
        let empid = match row[0]{
            DataType::Float(v)=>v,
            _ => 0f64,
        };
        let datefrom = match row[2]{
            DataType::Float(v)=>v,
            _ => 0f64,
        };
        let dateto = match row[3]{
            DataType::Float(v)=>v,
            _ => 0f64,
        };
        let empid=empid as i32;
        let datefrom=date_from_float(datefrom as i64);
        let dateto=date_from_float(dateto as i64);
        if datefrom.year()==dt.year() && datefrom.month()==dt.month()
        {
            let lve=dateto.signed_duration_since(datefrom).num_days();
            match lvmp.get(&empid){
                Some(v)=>lvmp.insert(empid as i32,(lve as i32)+*v+1),
                _=>lvmp.insert(empid as i32,lve as i32+1),
            };
        }
    }
    Ok(())
}
