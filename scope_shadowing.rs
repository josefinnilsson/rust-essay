fn main() {
	scope_example();
	shadowing_example();
}

fn scope_example() {
	let x = 5; //x is valid
	{
		let y = 10;
		println!("x = {}", x); //x is valid in the inner scope
		println!("y = {}", y); //y is valid in the inner scope
	} // y goes out of scope
	println!("x = {}", x); //x is valid in the outer scope
} //x goes out of scope

fn shadowing_example() {
	let x = 5;
	{
		let x = "shadowed";
		println!("x = {}", x); //Will print "x = shadowed"
	}
	println!("x = {}", x); //Will print "x = 5"
	let x = 3;
	println!("x = {}", x); //Will print "x = 3"
}