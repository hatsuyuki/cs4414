use std::env;

fn main() {
    let error = "Error: Please provide a number as an argument.";

    let s = env::args().nth(1).expect(error);
    let i = s.parse::<u32>().ok().expect(error);

    println!("{} is the lowest number with {} Collatz steps", find_collatz(i), i);
}

fn collatz(n: u64) -> u32 {
    if n == 1 { return 0; }
    match n % 2 {
        0 => { 1 + collatz(n/2) }
        _ => { 1 + collatz(n*3 + 1) }
    }
}

fn find_collatz(n: u32) -> u32 {
	let mut x: u64 = 1;
	loop {
		if n == collatz(x) { return x as u32; }
		x += 1;
	}
}