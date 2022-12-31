/* Problem 1.2.11 Nude Plays 
* day 3 
*/
fn main() {
    // 1. Definitions
    let min_price = 1.0_f32;
    let ticket_incre_due_to_a_cent_dec: f32 = 15.0/5.0;

    let mut price = 2.5_f32;
    let mut expense = 0.8_f32;
    let mut tickets = 120.0_f32;

    // initial state
    let mut sales: f32;
    let mut more_tickets:f32;
    let mut cent_diff:f32;
    let mut expenses: f32;
    let mut gain: f32;
    
    let mut gain_price: Vec<(f32, f32)> = vec![];
    let mut better_gain = (0., 0.);
    
    loop {
        // For just a cent decrease
        // 2. calculate the intial value of the sales, expense, and the gain.
        sales = price * tickets;
        expenses = expense * tickets;
        gain = sales - expenses;
        
        // 3. Store the value of gain price and sales in the vector
        gain_price.push((gain, price));
        // println!("{gain:4.2} - {price:4.2} - {sales:4.2} - {expenses:.2}\n");
        
        // 4. decrease the price by a cent
        price -= 0.01 * 5.; // just a cent decrease
        // 5. calculate the diff in cent btwen the price and 2.5
        cent_diff = ((2.5 - price) * 100.).round();

        // 6. Calculate the total number of tickets now
        more_tickets =  ticket_incre_due_to_a_cent_dec * cent_diff;
        tickets = 120. + more_tickets;
        
        // 7. Calculate how much a ticket expense would be now
        expense = 0.8 + (0.02 * (cent_diff / 5. )); // expenses per ticket increase by 2 cent
        
        
        // 8. repeat 2. to 7. till either gain is < 0 or the price >= min_price
        if price <= min_price || gain < 10.0 { break }
    }
    
    println!("{}\nPrice -  Gain\n{}", "-".repeat(14), "-".repeat(14));
    // 9. for items in the vector, search for that with the has gain
    for (gain, price) in gain_price {
        println!("{price:<5.2} - {gain:<6.2}");
        if (gain).round() > (better_gain.0) {
            better_gain = (gain, price);
        }
    }

    println!(
        "{}\n\n\
        It would be benefitial to sell a ticket at {:.2}\n\
        For a better gain of {:.2}\n", 
        "-".repeat(14),
        better_gain.1,
        better_gain.0, 
    );
    
}