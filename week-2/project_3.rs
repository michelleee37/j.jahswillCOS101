fn main() {
	let p:f64 = 210_000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	// Depreciation
	let fv = p * (1.0 - (r / 100.0)).powf(n);
	// dv = depreciated value
	// fv = future value
	let dv = p - fv;
    println! ("Dear Esteemed User,");
    println! ("The value of the Tv after three years is {} with a depreciated value of {}.", fv, dv);
} 