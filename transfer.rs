fn main() {
	transfer_example();
}

fn transfer_example() {
	let a = give(); //transfer_example() becomes the owner of the return value of give()
	let b = 1;
	let c = take_give(b); //ownership of b is transferred to take_give()
						  //and ownership of c is received
} //a and c are dropped, b was transferred so it is not dropped

fn give() -> u32 {
	let x = 123;
	x //ownership of x is transferred to the calling function
}

fn take_give(x: u32) -> u32 { //take_give() gets ownership of x
	x //ownership of x is transferred to the calling function
}
