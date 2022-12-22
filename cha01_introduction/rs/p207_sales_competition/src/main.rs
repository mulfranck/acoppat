/*
* Problem 1.2.7 Salesman competition
* Assumptions made to solve this problem
* The punched card has 30 columms
* A column can hold only one digit number
* thus the problem is in receive this input
*    '-> prompt to enter one value at a time? thus user will enter 20 values
*    '-> or a string that is formated as a column sort
*        e.g 123435|23|2022|00123
*
* The punched card structure
*   - The first 9 holds the salesman id
*   - The 10 is empty to separate the salesman id and the month
*   - The 11th and 12th are to hold the value of the month
*   - The 15th to 18th (4 column) is used to hold the year
*   - The 25th to 29th (4 columns) for a months count 4column possible of 9999 sales.
*/

fn main() {
    let mut salesmen_id: Vec<i32> = vec![];
    let mut month: Vec<i32> = vec![];
    let mut year: Vec<i32>  = vec![];
    let mut sales:Vec<i32> = vec![];
    // 
    let sample = 
    "000012345|12|2022|0012
     000054321|11|2022|0034";

    const _MIN_MONTH: i32 = 01;
    const _MAX_MONTH: i32 = 12;
    const _MIN_YEAR: i32 = 2021;

    const _ID_LENGHT: i32 = 9;
    const _SALES_LENGHT: i32 = 4;
    const _YEARS_LENGHT: i32 = 4;
    const _MONTHS_LENGHT: i32 = 2;
    const _CARD_ROW_LENGTH: i32 = 25;    

    // Create a collection for the salesman_id and the other fields
    for line in sample.lines() {
        println!("{line}");
        if line.len() > 0 {
            let sections: Vec<_> = line.split("|").collect();
            salesmen_id.push(sections[0].trim().parse().unwrap());
            month.push(sections[1].trim().parse().unwrap());
            year.push(sections[2].trim().parse().unwrap());
            sales.push(sections[3].trim().parse().unwrap());  
        }
    }

    let mut winner = salesmen_id[0];
    let winner_position: i32 = sales[0];

    for (i, _) in salesmen_id.iter().enumerate() {
        if  sales[i] > winner_position {
            winner = salesmen_id[i];
        }
    }
    println!("{salesmen_id:?} {sales:?}");
    println!("The sales winner is {:?}", winner);
    
}
