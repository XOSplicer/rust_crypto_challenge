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

const HEXCHAR: [char; 16] = ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f'];

const BASE64CHAR: [char; 64] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','0','1','2','3','4','5','6','7','8','9','+','/'];

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
			match c {
				'0' => v.push(0u8),
				'1' => v.push(1u8),
				'2' => v.push(2u8),
				'3' => v.push(3u8),
				'4' => v.push(4u8),
				'5' => v.push(5u8),
				'6' => v.push(6u8),
				'7' => v.push(7u8),
				'8' => v.push(8u8),
				'9' => v.push(9u8),
				'a' => v.push(10u8),
				'b' => v.push(11u8),
				'c' => v.push(12u8),
				'd' => v.push(13u8),
				'e' => v.push(14u8),
				'f' => v.push(15u8),
				_ => return Err(BaseError::BadParsing),
			}
		}
		Ok(Hex::from_data(&v))
	}

	pub fn to_string(&self) -> String {
		self.data.iter().cloned().map(|c| HEXCHAR[c as usize]).into_iter().collect()
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
				byte4bit[i] = vec4bit.pop().unwrap();
			}
			println!("{:?}", (byte4bit[0], byte4bit[1], byte4bit[2], byte4bit[3], byte4bit[4], byte4bit[5]));
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
			match c {
				'A' => v.push(0u8),
				'B' => v.push(1u8),
				'C' => v.push(2u8),
				'D' => v.push(3u8),
				'E' => v.push(4u8),
				'F' => v.push(5u8),
				'G' => v.push(6u8),
				'H' => v.push(7u8),
				'I' => v.push(8u8),
				'J' => v.push(9u8),
				'K' => v.push(10u8),
				'L' => v.push(11u8),
				'M' => v.push(12u8),
				'N' => v.push(13u8),
				'O' => v.push(14u8),
				'P' => v.push(15u8),
				'Q' => v.push(16u8),
				'R' => v.push(17u8),
				'S' => v.push(18u8),
				'T' => v.push(19u8),
				'U' => v.push(20u8),
				'V' => v.push(21u8),
				'W' => v.push(22u8),
				'X' => v.push(23u8),
				'Y' => v.push(24u8),
				'Z' => v.push(25u8),
				'a' => v.push(26u8),
				'b' => v.push(27u8),
				'c' => v.push(28u8),
				'd' => v.push(29u8),
				'e' => v.push(30u8),
				'f' => v.push(31u8),
				'g' => v.push(32u8),
				'h' => v.push(33u8),
				'i' => v.push(34u8),
				'j' => v.push(35u8),
				'k' => v.push(36u8),
				'l' => v.push(37u8),
				'm' => v.push(38u8),
				'n' => v.push(39u8),
				'o' => v.push(40u8),
				'p' => v.push(41u8),
				'q' => v.push(42u8),
				'r' => v.push(43u8),
				's' => v.push(44u8),
				't' => v.push(45u8),
				'u' => v.push(46u8),
				'v' => v.push(47u8),
				'w' => v.push(48u8),
				'x' => v.push(49u8),
				'y' => v.push(50u8),
				'z' => v.push(51u8),
				'0' => v.push(52u8),
				'1' => v.push(53u8),
				'2' => v.push(54u8),
				'3' => v.push(55u8),
				'4' => v.push(56u8),
				'5' => v.push(57u8),
				'6' => v.push(58u8),
				'7' => v.push(59u8),
				'8' => v.push(60u8),
				'9' => v.push(61u8),
				'+' => v.push(62u8),
				'/' => v.push(63u8),
				_ => return Err(BaseError::BadParsing),
			}
		}
		Ok(Base64::from_data(&v))
	}

	pub fn to_string(&self) -> String {
		self.data.iter().cloned().map(|c| BASE64CHAR[c as usize]).into_iter().collect()
	}
}


#[test]
fn test_hex_to_base64() {
	let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	let hex = Hex::from_string(hex_string).unwrap();
	let base64_string = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
	let base64 = Base64::from_string(base64_string).unwrap();

	assert_eq!(hex_string, hex.to_string());
	assert_eq!(base64_string, base64.to_string());
	assert_eq!(base64, hex.to_base64());
}
