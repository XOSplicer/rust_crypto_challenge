


#[derive(Debug)]
pub struct Hex {
	data : Vec<u8>
}

pub struct Base64 {
	data : Vec<u8>
}

pub trait Base<T> {
	fn new() -> Self;

	fn from_data(data: &Vec<T>) -> Self;

	fn from_string(string: &str) -> Self;

	fn to_string(&self) -> String;
}

impl Base<u8> for Hex {
	pub fn new() -> Hex {
		Hex { data: Vec::new() }
	}
	
	pub fn from_data(data : &Vec<u8>) -> Hex {
		Hex { data: data.clone() } 
	}

	pub fn from_string(string: &str) -> Hex {
		unimplemented!();
	}

	pub fn to_string(&self) -> String {
		unimplemented!();
	}

}
