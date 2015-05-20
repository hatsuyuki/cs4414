use std::env;

// Find the minimum number whose collatz sequence takes n steps
// to terminate.
// 17s for n = 500 and vector size = 10^7

fn main() {
    let error = "Error: Please provide a number as an argument.";

    let s = env::args().nth(1).expect(error);
    let i = s.parse::<u32>().ok().expect(error);

    let mut a: Vec<u32> = vec![0; 10000000];
    
    // println!("collatz of {} is {}", i, collatz(i, &mut a));
    println!("{} is the lowest number with {} Collatz steps", find_collatz(i, &mut a), i);
}

fn collatz(mut n: u64, a: &mut Vec<u32>) -> u32 {
    let l = a.len();
    let mut i: usize;
    let mut v: Vec<u64> = vec![];
    // Generate a vector of numbers whose collatz()es are unknown    
    loop {
        i = n as usize;
        if (i < l && a[i] != 0) || n == 1 { break; }
        v.push(n);
        match n % 2 {
            0 => n = n/2,
            _ => n = n*3 + 1
        }
    }
    // Store all the newly computed values
    let base: u32 = a[i];
    let mut count: u32 = 0;
    loop {
        if let Some(m) = v.pop() {
            count += 1;
            i = m as usize;
            if i < l { a[i] = base + count; }
        }
        else { break; }
    }
    // Return answer
    return base + count;
}

fn find_collatz(n: u32, a: &mut Vec<u32>) -> u32 {
    let mut x: u64 = 1;
    loop {
        if n == collatz(x, a) { return x as u32; }
        x += 1;
    }
}