use std::io;
use std::cmp::Ordering;
use rand::Rng;
// Rng trait defines random numbers where as io is used for input and output && Ordering and Comparing Library.

fn main() {
	println!("Guess the Number!!");
	
	let secret_number = rand::thread_rng().gen_range(1..=100);
	
//thread_rng gives particular random number.  gen_range gives random no. in provided range which is like ((start..=end)).

	// println!("The secret number is : {secret_number}");
        loop {			//loop will provide repeatation in entering the number.
	println!("Please input your guess..");

	let mut guess = String::new(); //if not for "mut" the variable guess will be immutable

	io::stdin()  //we can also use std::io::stdin in place if hadn't include prelude like std::io
		.read_line(&mut guess) 
		.expect("Failed to read line");

//read_line is used to take input and can only be used if variable passed if mut.
//if the result gets the err than "except" will activate && 
//if "except" will not we called than we will get warning.
// cargo build is used after adding rand in .toml to use rand in secret_number

	let guess: u32 = match guess.trim().parse() {
		Ok(num) => num,
		Err(_) => continue,
	};		

//Enum only has to output Ok or Err therefore. And "-" is "CATCHALL" value in RUST
//Rust allows "Shadow" the previos variable with same name and reinitiate the variable again
//"trim" method will remove Spaces from the string and u32 is numerical meaning in RUST and "parse" is used to convert String to Number 

	println!("You guessed: {guess}");

	match guess.cmp(&secret_number) {  //match is used as expression in which secret_number is passed.
		Ordering::Less => println!("Too Small"),
		Ordering::Greater => println!("Too Big!"),
		Ordering::Equal => { 
			println!("You Win");
			break;
		}
	}
	}
}
//let x = 5;
//let y = 10;
//println!("x = {} and y = {}", x, y);
//Rust differentiate b/w variable type
