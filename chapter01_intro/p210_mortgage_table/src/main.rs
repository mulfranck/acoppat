/* Problem 1.2.10 Mortgage Table
*   
*/ 

fn display_row(m: u32, i: f32, p: f32, t: f32, pay: f32, np: f32 ) -> () {
    println!("{m:>3}\t{p:>8.2}   {i:>8.2}    {t:4.2}   {pay:^7.2} {np:^14.2}")
}

fn main() {
    let r = 0.06; // interest rate for a year
    let t: f32 = 58.33; // tax
    let pay: f32 = 300.00; // payment
    
    // no actual null concept it = 0; not accepted if 0 wouldnt be used
    let mut it: f32; // interest
    let mut np: f32; // new principal
    let mut m: u32 = 1; // month
    let mut p: f32 = 20_000.00; // principal

    let divider = "-".repeat(56);
    // table header
    println!("{divider}");
    println!("Month | Principal | Interest |  Tax  | Payment | New Principal");
    println!("{divider}");
    loop {
        // first get the interest
        it = p * r / 12.0; // interest for a month in a year
        if it < 1.0 {
            println!("The Final Payment of ${:.2}", (p +  t).round()); 
            break; 
        }

        // Calculate the new principal
        np = p - pay + it + t;

        // np = (np * 100.).round() / 100.;
        
        // Draw the table row
        display_row(m, it, p, t, pay, np);
        
        // if m == 6 { break; }

        // update for the next month
        m += 1;
        p = np;

    }
    println!("Hello, world!");
}
