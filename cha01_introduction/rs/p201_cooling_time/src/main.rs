use std::io::stdin;

fn main() {
	// Variables denys to be separated by a comma
	// To declare a variable without initializing it
	// and making use of it is not allowed.
	let mut first_reading: String = String::new();
	let mut second_reading: String = String::new();
	let mut third_reading: String = String::new();
	// _ infront of a variable makes it pass the compiler without warning
	let mut _fourth_reading: String = String::new(); 
	let sum: i32;
	
	/*======== COLLECT STRING DATA INPUT ==========*/
	
	println!("Enter the First cooling time: ");
	stdin().read_line(&mut first_reading)
			.expect("Failed to obtain the first cooling time");

	println!("Enter the second cooling time: ");
	stdin().read_line(&mut second_reading)
			.expect("Failed to obtain the second cooling time");

	println!("Enter the second cooling time: ");
	stdin().read_line(&mut third_reading)
			.expect("Failed to obtain the second cooling time");



	/*======== CONVERT STRING TO NOBA ==========*/

	// tirm() to remv whitespace and parse() to try converting string to noba
	// to the type :i32 thus it must be annotated
	let expect_enter_value_error_msg = "Please enter a valid number!";
	
	let first_reading: i32 = first_reading.trim().parse()
						.expect(&expect_enter_value_error_msg);

	let second_reading: i32 = second_reading.trim().parse()
						.expect(&expect_enter_value_error_msg);

	let	third_reading: i32 = third_reading.trim().parse()
						.expect(&expect_enter_value_error_msg);

	sum = first_reading + second_reading + third_reading;
	
    println!(
    	"THE THREE TIMES ARE: {} {} {} ",
    	first_reading,
    	second_reading,
    	third_reading
    );
   println!(
   		"THE TOTAL COOLING TIME IS   {} SECONDS!",
   		sum
   	);
}
