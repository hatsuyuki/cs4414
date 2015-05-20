fn main() {
    let x = (51, true);
    match x {
        (20...26,   true) => { println!("Branch a"); }
        (_,         true) => { println!("Branch b"); }
        (40...49,   _   ) => { println!("Branch c"); }
        _                 => { println!("Branch d"); }
    }
}