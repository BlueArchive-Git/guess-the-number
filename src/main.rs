use std::io::{self, Write};
use rand::Rng;
fn main() {
	let secret = rand::thread_rng().gen_range(1..=100);
	println!("I silently picked a number between 1 and 100 in my head. Guess what it is?");
	loop {
		print!("> ");
		io::stdout().flush().unwrap();
		let mut number=String::new();
		io::stdin().read_line(&mut number).expect("读取失败！");
		let number: i32 = number.trim().parse().expect("请输入数字！");
		if number > secret {
			println!("Too big");
		} else if number < secret {
			println!("Too small");
		} else if number == secret {
			println!("You guessed right! The number is {}", secret);
			break;
		}
	}
}
