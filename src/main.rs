mod core;
use core::*;

fn main() {
	let hex = Hex::from_string("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
	//println!("{:?}", hex);
	println!("{:?}", hex.to_string());
	let b64ref = Base64::from_string("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t").unwrap();
	//println!("{:?}", b64ref);
	println!("{:?}", b64ref.to_string());
	let b64 = hex.to_base64();
	//println!("{:?}", b64);
	println!("{:?}", b64.to_string());
}
