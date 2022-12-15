use std::io::stdin;

fn main() {
    println!("Enter your Ten cards int separated by \"comma\"!");
    // let list: String = String::from("12, 32, 24, 56,23");
    let mut list: String = String::new();

    stdin().read_line(&mut list)
            .expect("Enter a list, boi!");

    let str_numbers = list.split(',');
    // println!("{:?}", str_numbers);
    let str_numbers: Vec<&str> = str_numbers.collect();
    let mut numbers: Vec<f64> = vec![];
    
    for numb in str_numbers {
        let n  = numb.trim();
        
        let n = n.parse();
        let n: f64 = n.unwrap();
        numbers.push(n);
    }

    let n: usize = numbers.len();
    
    let mut j: i32 = 1;
    let mut i: usize = 1;
    let mut b = numbers[0];

    while i < n  {
        if numbers[i] > b {
            b = numbers[i];
            j = i.try_into().unwrap();
        }
        i += 1;
    }
    println!("You entered {n} Card Int which are {:?}", numbers);
    println!("The largest card int is {b} at position {}.", j + 1);

}
