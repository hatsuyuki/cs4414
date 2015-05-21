use std::env;
use std::cmp;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let error = "Error: Please provide two valid file names";

    let n1 = env::args().nth(1).expect(error);
    let n2 = env::args().nth(2).expect(error);

    let mut reader1 = BufReader::new(File::open(n1).unwrap());
    let mut reader2 = BufReader::new(File::open(n2).unwrap());

    let mut msg1 = String::new();
    let mut msg2 = String::new();
    let mut msg = String::new();

    unsafe {
        reader1.read_to_end(msg1.as_mut_vec()).unwrap();
        reader2.read_to_end(msg2.as_mut_vec()).unwrap();
        xor(msg1.as_bytes(), msg2.as_bytes(), msg.as_mut_vec());
    }
    
    println!("{}", msg as String);
}

fn xor(a: &[u8], b: &[u8], c: &mut Vec<u8>) {
    c.clear();
    for i in 0..cmp::min(a.len(), b.len()) {
        c.push(a[i] ^ b[i]);
    }
}