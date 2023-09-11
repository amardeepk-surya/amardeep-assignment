use crate::*;
use calamine::{DataType, Excel};
use std::collections::HashMap;

pub fn new(pth: &Pth, deptmp: &mut HashMap<i32, String>) -> Result<(), std::io::Error> {
    let mut i = 0;
    let mut excel = Excel::open(&pth.d).unwrap();
    let r = excel.worksheet_range("Sheet1").unwrap();
    for row in r.rows() {
        if i == 0 {
            i = 1;
            continue;
        }
        let deptid = match row[0] {
            DataType::Float(v) => v,
            _ => 0f64,
        };
        let depttitle = match &row[1] {
            DataType::String(v) => v,
            _ => "",
        };
        let deptval = depttitle.to_string();
        deptmp.insert(deptid as i32, deptval);
    }
    Ok(())
}
