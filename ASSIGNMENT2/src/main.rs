use clap::Parser;

mod prog1;
mod prog2;

#[derive(Parser)]
pub struct Pth {
    #[clap(short)]
    pub p: String,
    #[clap(short)]
    pub i: String,
    #[clap(short)]
    pub o: String,
}

pub fn main() {
    let pth: Pth = Pth::parse();
    if pth.p == "prog1" {
        prog1::function(&pth);
    } else if pth.p == "prog2" {
        prog2::function(&pth);
    }
}
