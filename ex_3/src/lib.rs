use std::thread::{sleep, spawn};
use std::time::Duration;
use pyo3::prelude::*;

fn burger() -> () {
    sleep(Duration::from_millis(1000));
    println!("burger made");
}

fn soda() -> () {
    sleep(Duration::from_millis(100));
    println!("soda pour");
}

#[pyfunction]
async fn order(input_str: String) -> () {
    let input: Vec<&str> = input_str.split(' ').collect();
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

#[pymodule]
fn py202(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(order, m)?)?;
    Ok(())
}