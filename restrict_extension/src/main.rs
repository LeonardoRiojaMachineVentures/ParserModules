fn validate_filename(filename : Vec<u8>) -> Result<String, ()> {
	//: NOT(Eq (filename.first() Char::Space))
	let mut words : Vec<u8> = Vec::with_capacity(1);
	let mut word : Vec<u8> = Vec::with_capacity(6);
	for i in filename.iter() {
		if i.is_ascii_alphanumeric() || *i == b'_' {
			word.push(*i);
		}
		else {
			if i == b'.' {
				words.push(word);
				word.clear();
			}
			return Err(());
		}
	}
	match words.last() {
		"rs" | "tech" => {
			if words.len() == 2 {
				return Ok(filename.as_string());
			}
			else {return Err(());}
		},
		_ => {
			if words.len() == 1 {
				return Ok(filename.as_string());
			}
			else {return Err(());}
		},
	}
}

fn main() {
	let file1 = b"my_file.rs";
	let file2 = b"another_one";
	let file3 = b"&&*valid?.no";
	let file4 = b"something.tech";
	let file5 = b"some.tech.rs";

	let file1 = validate_filename(file1.to_vec());
	let file2 = validate_filename(file2.to_vec());
	let file3 = validate_filename(file3.to_vec());
	let file4 = validate_filename(file1.to_vec());
	let file5 = validate_filename(file5.to_vec());

	println!("{:?}", file1);
	println!("{:?}", file2);
	println!("{:?}", file3);
	println!("{:?}", file4);
	println!("{:?}", file5);
	
}