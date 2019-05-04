fn main() {
	ownership_loss();
}

fn ownership_loss() {
	let v = vec![1, 2, 3];
	let first = first_element(&v); //ownership of v borrowed
	let second = second_element(&v); //ownership of v borrowed
	println!("{}", first);
	println!("{}", second);
}

fn first_element(v: &Vec<i32>) -> i32 { v[0] }

fn second_element(v: &Vec<i32>) -> i32 { v[1] }