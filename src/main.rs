use rand::{thread_rng, Rng};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 2 {
        print_usage_and_exit();
    }

    let low_result = str::parse::<i64>(&args[0]);
    let high_result = str::parse::<i64>(&args[1]);
    if low_result.is_err() || high_result.is_err() {
        print_usage_and_exit();
    }

    let low = low_result.unwrap();
    let high = high_result.unwrap();

    let mut rng = thread_rng();
    let answer = rng.gen_range(low..=high);

    println!("{}", answer);
}

fn print_usage_and_exit() {
    println!("Usage: randint LOW HIGH");
    println!("");
    println!("generates a random integer between LOW and HIGH (both inclusive).");
    println!("LOW and HIGH must fit in a signed 64 bit integer, and are represented in base-10.");
    process::exit(1);
}
