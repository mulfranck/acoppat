

fn main() {
	const TOP: f64 = -40.;
	const BT: f64 = 100.;

	let mut in_fah: f64;
	let mut in_deg: f64 = TOP.try_into().unwrap();
	// ndegCent = (9n/5)+32

	println!("deg centigrade\tdeg Fahrenheit");
	
	loop {
		in_fah = ((in_deg * 9.) / 5.) + 32.;
		
		println!("{in_deg:>12.0}\t{in_fah:>12.2}");

		in_deg = in_deg + 1.;
		
		if in_deg == BT {
			break;
		}
	}
}
