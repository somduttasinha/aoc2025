use crate::daily::day1::day1;
use crate::daily::day2::day2;
use clap::Parser;

mod daily;

#[derive(Parser, Debug)]
struct Args {
    #[arg[short, long]]
    day: u8,
}

fn main() {
    let args = Args::parse();
    match args.day {
        1 => day1(),
        2 => day2(),
        _ => unimplemented!("Other days have not yet been implemented"),
    }
}
