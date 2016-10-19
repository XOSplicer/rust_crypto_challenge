#[derive(Debug, PartialEq, Clone)]
pub struct Hex {
	data : Vec<u8>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Base64 {
	data : Vec<u8>
}

#[derive(Debug)]
pub enum BaseError {
	BadParsing
}

static HEXCHARS: &'static str = "0123456789abcdef";
static BASE64CHARS: &'static str =
 	"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

impl Hex {
	pub fn new() -> Hex {
		Hex { data: Vec::new() }
	}

	pub fn from_data(data : &Vec<u8>) -> Hex {
		Hex { data: data.clone() }
	}

	pub fn from_string(string: &str) -> Result<Hex, BaseError> {
		let s = string.to_lowercase();
		let mut v = Vec::new();
		for c in s.chars() {
			match HEXCHARS.find(c) {
				Some(x) => v.push(x as u8),
				None 	=> return Err(BaseError::BadParsing),
			}
		}
		Ok(Hex::from_data(&v))
	}

	pub fn to_string(&self) -> String {
		self.data.iter().cloned().map(|c| HEXCHARS.chars()
			.nth(c as usize).expect("Invalid Hex value!")).into_iter().collect()
	}

	pub fn to_base64(&self) -> Base64 {
		println!("Converting to Base64");
		let mut vec4bit = self.data.clone();
		let mut vec6bit = Vec::new();
		//make sure that the bytes are complete
		if vec4bit.len()%2 == 1 {
			vec4bit.push(0u8);
		}
		//make sure there is a multiple of 3 bytes
		while (vec4bit.len()/2)%3 != 0 {
			vec4bit.push(0u8);
			vec4bit.push(0u8);
		}
		println!("{:?}", Hex::from_data(&vec4bit).to_string());
		vec4bit.reverse();
		while !vec4bit.is_empty() {
			//will always have 3 elements to pop, since len is a multiple of 3 as assured above
			let mut byte4bit : [u8; 6] = [0;6];
			for i in 0..6 {
				byte4bit[i] = vec4bit.pop()
									.expect("4-Bit-Vector length not a multiple of 6 (this should not happen)!");
			}
			println!("{:?}", (byte4bit[0],
								byte4bit[1],
								byte4bit[2],
								byte4bit[3],
								byte4bit[4],
								byte4bit[5]));
			let mut int24bit : u32 = 0;
			for i in 0..6 {
				int24bit += (byte4bit[i] as u32) << (4*i);
			}
			println!("{:x}", int24bit);
			/*for i in 0..4 {
				//TODO
				vec6bit.push(((int24bit & (0b1111u32 << 6*i)) >> 6*i) as u8);
			}*/

		}
		println!("{:?}", vec6bit);
		Base64::from_data(&vec6bit)
	}
}

impl Base64 {
	pub fn new() -> Base64 {
		Base64 { data: Vec::new() }
	}

	pub fn from_data(data : &Vec<u8>) -> Base64 {
		Base64 { data: data.clone() }
	}

	pub fn from_string(string: &str) -> Result<Base64, BaseError> {
		let s = string;
		let mut v = Vec::new();
		for c in s.chars() {
			match BASE64CHARS.find(c) {
				Some(x) => v.push(x as u8),
				None 	=> return Err(BaseError::BadParsing),
			}
		}
		Ok(Base64::from_data(&v))
	}

	pub fn to_string(&self) -> String {
		self.data.iter().cloned().map(|c| BASE64CHARS.chars()
			.nth(c as usize).expect("Invalid Base64 value!")).into_iter().collect()
	}
}


#[test]
fn test_hex_to_base64() {
	let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	let hex = Hex::from_string(hex_string).expect("Hex String could not be parsed!");
	let base64_string = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
	let base64 = Base64::from_string(base64_string).expect("Base64 String could not be parsed!");

	assert_eq!(hex_string, hex.to_string());
	assert_eq!(base64_string, base64.to_string());
	assert_eq!(base64, hex.to_base64());
}
