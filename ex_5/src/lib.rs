use pyo3::prelude::*;
use pyo3::types::PyList;
use std::thread::{sleep, spawn};
use std::time::Duration;
fn burger(counter: Py<PyList>) -> () {
    println!("making burger");
    sleep(Duration::from_millis(1000));
    Python::with_gil(|py| {
        let _ = counter.call_method1(py, "append", ("burger",)).unwrap();
        println!("burger made, # of burger: {}", counter.call_method0(py,"__len__").unwrap());
    })
}

fn soda(counter: Py<PyList>) -> () {
    println!("pouring soda");
    sleep(Duration::from_millis(100));
    Python::with_gil(|py| {
        let _ = counter.call_method1(py, "append", ("soda",)).unwrap();
        println!("soda poured, # of soda: {}", counter.call_method0(py, "__len__").unwrap());
    })
}

#[pyfunction]
fn order<'py>(
    py: Python,
    input_str: String,
    burger_pool: &Bound<'py, PyList>,
    soda_pool: &Bound<'py, PyList>,
) -> () {
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
                let burger_pool = burger_pool.clone().unbind();
                let second = Py::clone_ref(&burger_pool, py);
                actions.push(spawn(move || burger(second)));
            }
            "soda" => {
                let soda_pool = soda_pool.clone().unbind();
                let second = Py::clone_ref(&soda_pool, py);
                actions.push(spawn(move || soda(second)));
            }
            _ => println!("invalid order"),
        }
    }
    fn try_consume<'py>(pool: &Bound<'py, PyList>, count: usize) -> bool {
        let resource = pool.call_method0("__len__").unwrap().extract::<usize>().unwrap();
        if resource >= count {
            for _ in 0..count {
                let _ = pool.call_method0("pop").unwrap();
            }
            true
        } else {
            false
        }
    }

    let mut is_burger_done = false;
    let mut is_soda_done = false;

    loop {
        // py.allow_threads( || {
            sleep(Duration::from_millis(10));
        // });
        is_burger_done = is_burger_done || try_consume(&burger_pool, burger_count);
        is_soda_done = is_soda_done || try_consume(&soda_pool, soda_count);

        if is_burger_done && is_soda_done {
            println!("order [{}] complete", input_str);
            break;
        }
    }
}

#[pymodule(gil_used = false)]
fn py202(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(order, m)?)?;
    Ok(())
}
