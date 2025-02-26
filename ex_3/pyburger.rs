use std::thread::{sleep, spawn};
use std::time::Duration;
fn burger() -> () {
    sleep(Duration::from_millis(1000));
    println!("burger made");
}

fn soda() -> () {
    sleep(Duration::from_millis(100));
    println!("soda pour");
}

fn order(input_str: String) -> () {
    // we use string as input, it is easier to handle when we use this to PyO3
    let input: Vec<&str> = input_str.split(' ').collect();
    // clone each strings as they need to live through the whole program lifetime
    let input_order: Vec<String> = input.iter().map(|x| x.to_string()).collect();
    let mut actions = vec![];
    for item in input_order {
        // now item is String instead of &str
        match item.as_str() {
            "burger" => actions.push(spawn(burger)),
            "soda" => actions.push(spawn(soda)),
            _ => println!("invalid order"),
        }
    };
    for handle in actions {
        handle.join().unwrap();
    }
    println!("order complete");
}

fn main() {
    order("burger soda burger".to_string());
}
