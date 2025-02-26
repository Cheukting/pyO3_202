use pyo3::{prelude::*, wrap_pyfunction};
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

async fn make_order(input_str: String) -> () {
    let input: Vec<&str> = input_str.split(' ').collect();
    let mut actions: Vec<JoinHandle<()>> = Vec::new();
    for item in input {
        match item {
            // save spawned JoinHandles in 'actions' vec
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

#[pyfunction]
// we use string as input because list in Python is kinda tricky
fn order(py: Python, input_str: String) -> PyResult<Bound<PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        make_order(input_str).await;
        Ok(())
    })
}

#[pymodule]
fn py202(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(order, m)?)?;
    Ok(())
}