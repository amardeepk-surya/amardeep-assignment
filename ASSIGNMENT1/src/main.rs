use clap::Parser;
use std::collections::HashMap;
use std::io::Read;

#[derive(Parser)]
pub struct Pth {
    #[clap(short)]
    pub e:String,
    #[clap(short)]
    pub d:String,
    #[clap(short)]
    s:String,
    #[clap(short)]
    l:String,
    #[clap(short)]
    o:String,
}

mod emp;
mod dept;
mod sal;
mod leave;
fn main() {
    let pth:Pth=Pth::parse();
    let mut deptmp:HashMap<i32,String>=HashMap::new();
    dept::new(&pth,&mut deptmp).expect("Initialisation of dept db failed");
    let mut salmp:HashMap<i32,String>=HashMap::new();
    sal::new(&pth,&mut salmp).expect("Initialisation of sal db failed");
    let mut lvmp:HashMap<i32,i32>=HashMap::new();
    leave::new(&pth,&mut lvmp).expect("Initialisation of leave db failed");
    let outst = emp::OutSt::new(&pth,&deptmp,&salmp,&lvmp).expect("Initialisation of emp db failed");
    match outst.save(&pth) {
        Ok(_) => println!("Completed"),
        Err(why) => println!("An error occurred: {}", why),
    }
}