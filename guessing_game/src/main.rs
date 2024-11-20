use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn return_secret_number () -> u8 {
	rand::thread_rng().gen_range(1..=100)
}

fn check_guessed_number (secret_number:u8, guessed_number:u8) -> i8 {
	match guessed_number.cmp(&secret_number) {
		Ordering::Equal => 0,
		Ordering::Greater => 1,
		Ordering::Less => -1,
	}
}

fn main() {

	println!("Guess the number!");
	
	let secret_number: u8 = return_secret_number();
	//println!("The secret number is: {secret_number}");

	loop {
		println!("Please input your guess.");

		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		let guess: u8 = match guess.trim().parse()	{
			Ok(num) => num,
			Err(_) => continue,
		};

		println!("You guessed: {}",guess);

		let result = check_guessed_number(secret_number, guess);

		match result {
			1 => {
				println!("You win!!");
				break;
			}
			-1 => println!("Too small!"),
			_ => println!("Too big!")
		}

		/* Better to use match in rust
		if result==1 {
			println!("You win");
			break;
		}
		else if result==-1 {
			println!("Too small!");
		}
		else {
			println!("Too big!");
		}*/

		/* Created a function of this for better understanding functions
		match guess.cmp(&secret_number)	{
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}*/
	}
}
