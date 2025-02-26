use tokio::task::JoinHandle;
use tokio::{
    spawn,
    time::{sleep, Duration},
};

async fn burger() -> () {
    sleep(Duration::from_millis(1000)).await;
    println!("burger made");
}

async fn soda() -> () {
    sleep(Duration::from_millis(100)).await;
    println!("soda pour");
}

async fn order(input_str: String) -> () {
    // we use string as input, it is easier to handle when we use this to PyO3
    let input: Vec<&str> = input_str.split(' ').collect();
    let mut actions: Vec<JoinHandle<()>> = Vec::new();
    for item in input {
        match item {
            "burger" => actions.push(spawn(burger())),
            "soda" => actions.push(spawn(soda())),
            _ => println!("invalid order"),
        }
    }
    for action in actions {
        let _ = action.await;
    }
    println!("order complete");
}

#[tokio::main]
async fn main() {
    order("burger soda burger".to_string()).await;
}
