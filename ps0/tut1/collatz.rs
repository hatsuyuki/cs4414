use std::env;

fn main() {
    let error = "Error: Please provide a number as an argument.";

    let s = env::args().nth(1).expect(error);
    let i = s.parse::<i32>().ok().expect(error);

    println!("{} has {} Collatz steps", i, collatz(i));
}

fn collatz(n: i32) -> i32 {
    if n == 1 { return 0; }
    match n % 2 {
        0 => { 1 + collatz(n/2) }
        _ => { 1 + collatz(n*3 + 1) }
    }
}