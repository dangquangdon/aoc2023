mod days;

use clap::Parser;
use std::collections::HashMap;

use days::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long, required(true))]
    day: i32,
}
fn main() {
    let args = Args::parse();

    type Func = fn();
    let mut solution: HashMap<i32, Func> = HashMap::new();
    solution.insert(1, day1::day1);
    solution.insert(2, day2::day2);

    let maybe_func = solution.get(&args.day);
    match maybe_func {
        Some(func) => func(),
        None => println!("\n--day {} is an invalid choice", args.day),
    };
}
