fn main() {
	let p: Vec<i32> = vec![1, 2, 3];
	let q = increment(&p);
	for x in q.iter() {
		println!("{} ", x);
	}
}

fn increment(v: &Vec<i32>) -> Vec<i32> {
	let mut w = v.clone();
	for i in 0..w.len() {
		w[i] += 1;
	}
	return w;
}