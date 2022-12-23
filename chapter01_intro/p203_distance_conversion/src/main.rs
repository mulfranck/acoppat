
fn main() {
	const ONE: f32 = 1.0;
	const ONE_KG_IN_FEET: f32 = 3281.;
	const ONE_KG_IN_MILE: f32 = 0.6214;
	const ONE_KG_IN_YARD: f32 = 1093.6;
	const ONE_KG_IN_NAUTICAL_MILE: f32 = 0.5396;
	let str = "-".repeat(56);
    let mut kg: f32 = 0.0;
	
	println!("{}", str);
	println!("Kilometers\tFeet\tYards\tNautical Mile\tMiles");
    println!("{}", str);

    loop {
    	// println!("{}", kg);
    	// > align to the right
    	// ^ align to the center
    	// .n => n decimal place 
    	println!(
    		"{:^10}   {:>6.0}{:>10.2}\t{:^14.2}\t{:.2}",
    		 kg,
			 kg * ONE_KG_IN_FEET,
			 kg * ONE_KG_IN_YARD,
			 kg * ONE_KG_IN_NAUTICAL_MILE,
			 kg * ONE_KG_IN_MILE,
    	 );
    
    	kg += ONE;

    	if kg > 12. {
    		break;
    	}


    	 
    }
}
