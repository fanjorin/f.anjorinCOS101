fn main() {
	/*qty*/
	let tq:f64= 2.00;
	let mq:f64= 1.00; 
	let hq: f64= 3.00;
	let dq:f64= 3.00;
	let aq:f64= 1.00;

	/*amount*/
	let ta: f64= 450000.00;
	let ma: f64= 1500000.00;
	let ha: f64= 750000.00;
	let da: f64= 2850000.00;
	let aa: f64= 250000.00;

	/*sum*/
	let sum= tq*ta+mq*ma+hq*ha+dq*da+aq*aa;
	println!("the sum is {}",sum );

	/*average*/
	let average= sum/(tq+mq+hq+dq+aq);
	println!("the average is{}",average );
}