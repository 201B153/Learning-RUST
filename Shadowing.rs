fn main() {

	let x = 5;
	let x = x + 1;
 
	{
		let x = x*2;
		println!("The Value inside the Bracket is: {x}");
	}

	println!("The value of x in Outer Circle is: {x}");
}
