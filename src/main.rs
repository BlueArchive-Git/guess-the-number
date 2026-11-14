use std::io::{self, Write};
use rand::Rng;
fn main() {
	let secret = rand::thread_rng().gen_range(1..=100);
	println!("我在心中默念了一个 1~100 之间的数字，你来猜猜是什么？");
	loop {
		print!("> ");
		io::stdout().flush().unwrap();
		let mut number=String::new();
		io::stdin().read_line(&mut number).expect("读取失败！");
		let number: i32 = number.trim().parse().expect("请输入数字！");
		if number > secret {
			println!("猜大了");
		} else if number < secret {
			println!("猜小了");
		} else if number == secret {
			println!("猜对了！这个数是 {}", secret);
			break;
		}
	}
}
