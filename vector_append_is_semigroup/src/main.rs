#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Parity {
	Odd,
	Even,
}
/*
impl Vec<Parity> {
	fn join(x : Vec<Parity>, y : Vec<Parity>) -> Vec<Parity> {
		
	}
}
*/
fn main() {
use crate::Parity::*;
	let mut a = vec![Even, Odd];
	let b = vec![Odd, Even];
	let c = vec![Even, Even];
	/*	
	let a_copy = a.copy();
	let b_copy = b.copy();
	let c_copy = c.copy();
	*/
	let mut a_copy = a.clone();
	let mut b_copy = b.clone();
	let _c_copy = c.clone();
	
	//(a.push(b)).push(c);
	a.extend(&b);
	a.extend(&c);

	//a.extend(c.iter().cloned());
	
	b_copy.extend(&c);
	a_copy.extend(&b);
	let concatenated = [&a[..], &b[..], &c[..]].concat();
	println!("{:?}", concatenated);
	println!("{:?}", a);
	println!("{:?}", a_copy);
	

	
}