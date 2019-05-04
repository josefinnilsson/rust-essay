fn main() {
	let x = 10;
	let y = 3;
	println!("{}", largest_num(&x, &y));
}

fn largest_num<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
	if x > y {
		x
	} else {
		y
	}
}
