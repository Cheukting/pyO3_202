use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;
fn burger(counter: Arc<Mutex<usize>>) -> () {
    println!("making burger");
    let mut num = counter.lock().unwrap();
    sleep(Duration::from_millis(1000));
    *num += 1;
    println!("burger made, # of burger: {}", *num);
}

fn soda(counter: Arc<Mutex<usize>>) -> () {
    println!("pouring soda");
    let mut num = counter.lock().unwrap();
    sleep(Duration::from_millis(100));
    *num += 1;
    println!("soda poured, # of soda: {}", *num);
}

fn order(input_str: String, burger_pool: &Arc<Mutex<usize>>, soda_pool: &Arc<Mutex<usize>>) -> () {
    let input: Vec<&str> = input_str.split(' ').collect();
    let input_order: Vec<String> = input.iter().map(|x| x.to_string()).collect();

    //count the number of each order
    let burger_count: usize = input_order.iter().filter(|&item| item == "burger").count();
    let soda_count: usize = input_order.iter().filter(|&item| item == "soda").count();

    let mut actions = vec![];
    for item in input_order {
        // now item is String instead of &str
        match item.as_str() {
            "burger" => {
                let burger_pool = Arc::clone(&burger_pool);
                actions.push(spawn(move || {
                    burger(Arc::clone(&burger_pool))
                }));
            },
            "soda" => {
                let soda_pool = Arc::clone(&soda_pool);
                actions.push(spawn(move || {
                    soda(Arc::clone(&soda_pool))
                }));
            },
            _ => println!("invalid order"),
        }
    };

    fn try_consume(pool: &Arc<Mutex<usize>>, count: usize) -> bool {
        let mut resource = pool.lock().unwrap();
        if *resource >= count {
            *resource -= count;
            true
        } else {
            false
        }
    }

    let mut is_burger_done = false;
    let mut is_soda_done = false;

    loop {
        sleep(Duration::from_millis(10));
        is_burger_done = is_burger_done || try_consume(&burger_pool, burger_count);
        is_soda_done = is_soda_done || try_consume(&soda_pool, soda_count);

        if is_burger_done && is_soda_done {
            println!("order [{}] complete",input_str);
            break;
        }
    }
}

fn main() {
    let burger_pool = Arc::new(Mutex::new(0));
    let soda_pool = Arc::new(Mutex::new(0));

    let order1 = {
        let burger_pool = Arc::clone(&burger_pool);
        let soda_pool = Arc::clone(&soda_pool);
        spawn(move || {
            order("burger soda burger soda soda".to_string(), &burger_pool, &soda_pool);
        })
    };

    let order2 = {
        let burger_pool = Arc::clone(&burger_pool);
        let soda_pool = Arc::clone(&soda_pool);
        spawn(move || {
            order("burger soda burger".to_string(), &burger_pool, &soda_pool);
        })
    };

    order1.join().unwrap();
    order2.join().unwrap();

}
