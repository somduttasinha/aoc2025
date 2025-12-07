use crate::common::Part;
use crate::daily::day1::day1;
use crate::daily::day2::day2;
use crate::daily::day3::day3;
use crate::daily::day4::day4;
use clap::Parser;

mod common;
mod daily;

#[derive(Parser, Debug)]
struct Args {
    #[arg[short, long]]
    day: u8,
    #[arg[short, long, value_enum]]
    part: Part,
}

fn main() {
    let args = Args::parse();
    match (args.day, args.part) {
        (1, _) => day1(),
        (2, p) => day2(p),
        (3, p) => day3(p),
        (4, p) => day4(p),
        _ => unimplemented!("Other days have not yet been implemented"),
    }
}
