mod hex_to_base64;
use hex_to_base64::*;

fn main() {
	println!("Hello, world!");
	let h : Hex = Hex::new();
	println!("{:?}", h);
	let h = Hex::from_data(&vec![1,2,3]);
	println!("{:?}", h);
}
