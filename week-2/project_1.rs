fn main() {
	let p:f64 = 520000.0;
	let r:f64 = 10.0;
	let n:f64 = 5.0;

	// compund interest
	let a = p * (1.0 + (r / 100.0)).powf(n);
	println!("Amount is {}", a);
	let ci = a - p;
	println!("Compound Interest is {}", ci); 
} 