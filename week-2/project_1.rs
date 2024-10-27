fn main() {
	let p : f64= 520_000000.0;
	let n : f64= 5.0;
	let r : f64 = 10.0;

	// compound interest
	let a = p*(1.00+(r/100.0).powf(n));
	println!("Amount is {}", a);
	let ci = a - p;
	println!("Compound Interest is {}", ci);
}