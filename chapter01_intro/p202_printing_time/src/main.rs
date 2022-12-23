use std::io::stdin;


fn main() {
	const HOURS: i32 = 3600;
	const MINS: i32 = 60;


	let mut hour: i32 = 0;
	let mut min: i32 = 0;
	let length: i32;
	let sec: i32;
	let mut time_in_sec: String = String::new();

	println!("Enter time in seconds to convert");
	stdin().read_line(&mut time_in_sec)
			.expect("Failed to read input");

	length = time_in_sec.trim().len().try_into().unwrap();

	if length > 5 {
		panic!("Input value is greater than 5 digits");
	}
	
	let mut time_in_sec = time_in_sec.trim().parse()
								.expect("Enter a valid number!");
	// int has the copy trait thus the value of the time_in_sec is copied 
	let original_noba = time_in_sec;

	loop {
		
		if time_in_sec >= HOURS {
			hour = time_in_sec / HOURS;
			time_in_sec %= HOURS;
		} else if time_in_sec >= MINS {
				min	= time_in_sec / MINS;
				time_in_sec %= MINS;
		}
		  else if time_in_sec < 60 {
			sec = time_in_sec;
			break;
		}	
	}


	match hour {
		1 => println!("{} Seconds = {} hour {} minutes {} seconds", original_noba, hour, min, sec),
		_ => println!("{} Seconds = {} hours {} minutes {} seconds", original_noba, hour, min, sec),
	}

}
