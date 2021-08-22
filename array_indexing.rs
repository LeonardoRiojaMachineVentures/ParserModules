use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

	let mut index = String::new();
	
	loop {
	
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = match a[index] {
		Ok(value) => {value},
		Err(_) => {println!("The index is invalid");},
	}

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );


	}
}
