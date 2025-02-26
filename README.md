# PyO3 202 - Support Python Async and Multithreading in PyO3

Part of the reason why we would want to write Rust code for a Python library is for speed, another is to unlock the power of multithreading. In this workshop, we will look into advanced topics in PyO3 regarding support async in Python and how to do multithreading with PyO3.

---

## Prerequisite

Please note that in this workshop, beside assuming that you have experience programming [async in Python](https://docs.python.org/3/library/asyncio.html), you are also required to have basic knowledge of Rust. You are highly advised to go over [the Rust Book](https://doc.rust-lang.org/book/) to make sure you understand coding in Rust. You are also advised to be already familiar with [PyO3](https://pyo3.rs/). If they are new to PyO3, first doing the [PyO3 101 workshop](https://github.com/Cheukting/py03_101) would be ideal.

---

## Preflight checklist

- [Install/ Update Rust](https://www.rust-lang.org/tools/install)
- Make sure having Python 3.13, both the "normal" version (with GIL) and [free-threaded Python](https://docs.python.org/3/howto/free-threading-python.html#installation))
- Make sure using virtual environment (recommend using uv)

## Windows checklist

In this workshop we recommend using Unix OS (Mac or Linux). *If you use Windows, you may encounter problems with Rust and Maturin.* To minimise issues that you may encounter, please go through the extra checklist below:

- Install the [c++ build tools](https://visualstudio.microsoft.com/downloads/)
- [Check the `dll` files are linked correctly](https://pyo3.rs/v0.21.2/faq#im-trying-to-call-python-from-rust-but-i-get-status_dll_not_found-or-status_entrypoint_not_found)

## Setting up

1. Create a new working directory

```
mkdir pyo3_202
cd pyo3_202
```

2. Install both normal and thread-free Python

```
uv python install 3.13.2 3.13.2+freethreaded
```

3. Set up virtual environment and install **maturin**

```
uv venv -p 3.13.2 .venv
source .venv/bin/activate
uv pip install maturin
python -m ensurepip --default-pip
```
*Note: the last command is needed as `maturin develop` cannot find pip otherwise*

4. Start a project

If you want to start from scratch and write your own module, you can start a new project by

```
maturin init
```

or you can clone this repo with the code in this workshop included in the `ex_*` folders as check points at the end of each session/ exercises.

---

## Concurrency in model programming

Concurrency is used a lot in modern programming. We do not want to waste any computational time in our fast phased world, especially everyone now have a personal computer/ laptop or mobile device which is packed with computational power.

The are many design and way to achieve concurrency, to keep things simple when we are talking about concurrency in this workshop, we will be mainly focusing on:

- Asynchronous programming (with async/await pattern)
- Multithreaded programming

tha main difference between the two is, "who" is managing the synchronous tasks. In async/await, usually tasks are as coroutines which can be pause the resume, the synchronization is perform is a cooperative manner, if one of the tasks does not pause and give up control, then the program would not process until that tasks to finished. In multithreading, the OS will be managing all these different threads and decide who can proceed. The only way you can force there's only one thread at a time in multithreading is to deploy a lock (the GIL in Python is also a lock).

### Asyncio in Python

Asyncio is mainly used in slow I/O bound processes, e.g. API calls in a server. it involves an event loop that let tasks sits there idle while other tasks can be taken care of. Instead of running your Python code in sequences, with asyncio you can let a task run in the background and not blocking other code's execution.

Asyncio has been used in many Python applications, from frameworks like [FastAPI](https://fastapi.tiangolo.com/) to [Discord.py](https://discordpy.readthedocs.io/en/stable/), you may have used asyncio in Python before and wonder how you can use `async` and `await` with PyO3. We will cover how to do that in this workshop.

### Multi-threading in Python

Multi-threading, or threading, speed up I/O Bound and CPU bound processes, however, the CPU bound processes benefits maybe limited by the [Global Interpreter Lock (GIL)](https://docs.python.org/3/glossary.html#term-global-interpreter-lock). That may be the reason why threading in Python is not very popular. Threading allow tasks be handle in parallel, however, it is more complicated as there are complication that can arise with threading, such as race conditions and, deadlocks and memory leaks.

In the past, Python's solution to handle complexity caused by multi-threading is to introduce a [Global Interpreter Lock (GIL)](https://docs.python.org/3/glossary.html#term-global-interpreter-lock), which limit [bytecode](https://docs.python.org/3/glossary.html#term-bytecode) to be execute by only one thread at a time. However, due to high demand from the community, from Python 3.13 the GIL can be optionally disabled, this is sometimes also refered to as [free-threaded Python](https://docs.python.org/3/howto/free-threading-python.html#installation).


## async/.await in Rust

The most popular crate that provide async/.await programming patten is [tokio](https://docs.rs/tokio/latest/tokio/). It provides a lot of async io APIs which is similar to asyncio in Python. There are other crate that support async/.await such as [async-std](https://docs.rs/async-std/latest/async_std/), however, we will be using tokio in this workshop. 

In general Async concurrency is much more efficient than concurrency with threads in Rust, however memory cache may not be able to be shared between different tasks.

### Multi-threading in Rust

Since there is no GIL n Rust, we will have to pay attention to the issues that multi-threading may give us. But fear not, Rust is a language famous for its memory safety and robust ownership rules which can help avoid such problems. With tight ownership and lifetime rules, smart pointers and atomic referencing in Rust, we are equipt with tools that can help us to achieve freeless concurrency.

In this workshop, we will look at how to do threading in Rust and experiment with using PyO3 to perform multi-threading in Python code. We will also look at what adjustment we need to make in PyO3 to accommodate for [free-threaded Python](https://docs.python.org/3/howto/free-threading-python.html#installation).

---

## Exercise 1 - Taking orders with async/ await in Python

In this exercise, we will first do some warm up by writing some async/ await code in Python. Let's think about a fastfood restaurant, we will call it PyBurger. So let's look at `pyburger.py` (you create a new Python file an d copy and paste the following code):

```python
import asyncio


async def burger():
    await asyncio.sleep(5)  # time it takes to make a burger
    print("burger made")


async def soda():
    await asyncio.sleep(1)  # time it takes to pour a soda
    print("soda pour")


async def order(order=[]):
    actions = []
    for item in order:
        match item:
            case "burger":
                actions.append(burger())
            case "soda":
                actions.append(soda())
            case _:
                print("invalid order")
    await asyncio.gather(*actions) # await all the item tasks to be finished asynchronously.


asyncio.run(order(["burger", "soda", "burger"]))
```
Now try to add another order item "fries" and try to prepare various orders. Note that the orders are getting prepared asynchronously, you can try varing the time it takes for each item and see the order of competitions are being different.

So, how can we implement it in Rust and use it in Python with PyO3? For Rust, there are 2 most popular crate for doing async/.away, they are [`async-std`](https://async.rs/) and [`tokio`](https://tokio.rs/). In this example, we will use `tokio`. In a pure rust code, the similar implementation can be somthing like:

```rust
use tokio::task::JoinHandle;
use tokio::{
    spawn,
    time::{sleep, Duration},
};

async fn burger() -> () {
    sleep(Duration::from_millis(500)).await;
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
            // save spawned JoinHandles in 'actions' vec
            "burger" => actions.push(spawn(burger())),
            "soda" => actions.push(spawn(soda())),
            _ => println!("invalid order"),
        }
    }
    for action in actions {
        // make sure all items in the order has completed (all JoinHandles awaited, results discarded)
        let _ = action.await;
    }
    println!("order complete");
}

#[tokio::main]
async fn main() {
    order("burger soda burger".to_string()).await;
}
```

If you want, you can try running the code above in a new Rust binary project.

Note that when you use `spawn` to a future in Rust, the future will get scheduled and you will get a `JoinHandle` which you can await later to get the result back (if there's any). In our example above all the item tasks (`burger` and `soda`) deos not return anything meaningfully so we just discard them. However, by awaiting the JoinHandles later (not at the time it was spawn) the item tasks can be performed asynchronously.

To provide Python bindings for an async Rust library, a new dependency [`pyo3-async-runtimes`](https://github.com/PyO3/pyo3-async-runtimes) will be needed. It is a brand new part of the broader PyO3 ecosystem so things may change rapidly. Please check the official documentation and the project repo if in doubt.

in `Cargo.toml`, add these depedncencies:

```toml
pyo3 = "0.23"
pyo3-async-runtimes = { version = "0.23", features = ["attributes", "tokio-runtime"] }
tokio = { version = "1", features = ["full"] }
```

Note that [`pyo3-async-runtimes`](https://github.com/PyO3/pyo3-async-runtimes) support both [`async-std`](https://async.rs/) and [`tokio`](https://tokio.rs/) and the setting in `Cargo.toml` and the procedural marcos in the code may be different if you are using [`async-std`](https://async.rs/).

Now, let's transform the `order` future above to something we can use in Python:

```rust
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
```

In the code above, we renamed `order` as `make_order` and now `order` is just a normal function returning a `PyResult` with a bounded `PyAny` however, inside the function, we use `pyo3_async_runtimes::tokio::future_into_py` to turn the future that we have writen into a bounded Python object. `future_into_py` take two arguments, the first one is the Python token, the second is an async block (which will be evaluates to a future). 

PyO3 will then wrap and turn this future into a Python future, so when we call it from our Python module, we can `await` it. Let's look at the Python script which we can use it. Let's try this:

```python
import asyncio
from py202 import order

asyncio.run(order("burger soda burger"))

```

When you run it, you will see a `RuntimeError: no running event loop`. Why is it? The reason being that, now we have a future in Python created by PyO3 that have no reference to the Python event loop. So in Python's perspective it cannot be called. Unfortunately the current work around is to wrap `order` in a proper Python async function:

```python
import asyncio
from py202 import order


async def main():
    await order("burger soda burger")


asyncio.run(main())
```

Now you should see similar behaviour as your starting Python code. Take some time to add implementation to other order items such as "fries". If there are extra time, you can **challenge** yourself to implement a better input type than using String. Maybe a custom enum for items that someone can order.

---

## Exercise 2 - Testing Rust Async in Python

As seen in the previous exercise, Python has it's own event loop while we also have the event loop in Rust doing all the async/.await. The contribute to some [weird behaviours](https://docs.rs/pyo3-async-runtimes/latest/pyo3_async_runtimes/index.html) when using PyO3. To make things work, [the current solution](https://github.com/PyO3/pyo3-async-runtimes?tab=readme-ov-file#managing-event-loops) is to let Python control the main thread and block the main thread in Rust. This present extra challenges when trying to use corroutines provided by Python in Rust. This also require some special care to be taken during testing.

Lucky for us, [`pyo3-async-runtimes`](https://github.com/PyO3/pyo3-async-runtimes) provides [testing utilities](https://docs.rs/pyo3-async-runtimes/latest/pyo3_async_runtimes/testing/index.html) that we can use. These utilities let us perform integration tests and doc tests. Note that lib tests are not available due to the complexity mentioned.

First thing to note is that, the tests that we will be writing in this exercise will be put inside `pytests` (the name is customizable) instead of `tests`. As `tests` are standardized by `cargo` and we cannot use it this way.

Next, we will add the testing information in `Cargo.toml`:

```toml
[[test]]
name = "test_py202"
path = "pytests/test.rs"
harness = false
```

We also want to make sure `testing` and `attributes` are included in `features` of `pyo3-async-runtimes`:

```toml
pyo3-async-runtimes = { version = "0.23", features = ["attributes", "tokio-runtime", "testing"] }
```

Now in `pytests/test.rs` we can put a simple test to test if the `order` can be awaited with no issues:

```rust
mod tests {
    use pyo3::prelude::*;
    use py202::make_order;

    #[pyo3_async_runtimes::tokio::test]
    async fn test_async_sleep() -> PyResult<()> {
        make_order("burger soda burger".to_string()).await;
        Ok(())
    }
}

#[pyo3_async_runtimes::tokio::main]
async fn main() -> pyo3::PyResult<()> {
    pyo3_async_runtimes::testing::main().await
}
```

Now try running the test by `cargo test`, you will see that we run into an issue. You may get an unresolved import error "use of undeclared crate or module `py202`". This is because, [the compiler cannot find the crate with `crate-type` as "cdylib"](https://pyo3.rs/v0.23.4/faq.html#i-cant-run-cargo-test-my-crate-cannot-be-found-for-tests-in-tests-directory). Simply add "rlib" to `crate-type` in `Cargo.toml`:

```toml
[lib]
name = "py202"
crate-type = ["cdylib", "rlib"]
```

Now run `cargo test` again, you may see the "function `make_order` is private" error if you have not make the `make_order` in `src/lib.rs` public:

```rust
pub async fn make_order(input_str: String) -> () {
    let input: Vec<&str> = input_str.split(' ').collect();
    ...
}
```
If you run into other issues, have a look at the [FAQ page at PyO3 user guide](https://pyo3.rs/v0.23.4/faq.html) and see if there is a solution/ workaround for it.

Now, spend some time to expand the module. Add more functionalities in `src/lib.rs` and don't forget to test it in `pytests/test.rs`. For example, you can create another public future of `value_meal` which is a meal with a burger, a soda and a fries. It can be an item in the order. So an order can be "value_meal burger soda" which in total will have 2 burgers, 2 soda and 1 fries.

---

## Exercise 3 - Fearless threading in Rust

As we briefly mentioned before, threading in Python may not help with speeding up due to limitation by the GIL. Although there are now options to remove the GIL, which we will cover later. Let's assume the premise that we have the GIL in Python.

The GIL in Python is used as an assurance of threadsafety. In Rust, threadsafety is safegarded by the compiler with ownership and type checking.

To start, we will look at our previous Rust code that use async/.await with tokio:

```Rust
use tokio::task::JoinHandle;
use tokio::{
    spawn,
    time::{sleep, Duration},
};

async fn burger() -> () {
    sleep(Duration::from_millis(500)).await;
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
            // save spawned JoinHandles in 'actions' vec
            "burger" => actions.push(spawn(burger())),
            "soda" => actions.push(spawn(soda())),
            _ => println!("invalid order"),
        }
    }
    for action in actions {
        // make sure all items in the order has completed (all JoinHandles awaited, results discarded)
        let _ = action.await;
    }
    println!("order complete");
}

#[tokio::main]
async fn main() {
    order("burger soda burger".to_string()).await;
}
```

Now, instead of using async, we will make order in mutiple threads like this:

```rust
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
```

Note that there are no more `async` or `.away`, instead, `JoinHandle` are created when calling `thread::spawn`. They will then be `join()` at the end, which is equivalent to awaiting all of them to be finished.

Now, let's transfer this with PyO3 into something we could use in Python:

```rust
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
```
Now try `maturin develop` you will see an error:

`async functions are only supported with the `experimental-async` feature`

We forgot the `Cargo.toml` file, let's look at it. Now we can remove `pyo3-async-runtimes` and `tokio` in our dependency because we don't need them (`pyo3-async-runtimes` is still used in the test which we will not use for now). However, we are using the async feature in PyO3 (which is still in development). So we need to add this feature to PyO3:

```toml
pyo3 = { version = "0.23", features = ["experimental-async"]}
```
Now try `maturin develop` again and try out the function in Python. Notice that this time you can directly `asyncio.run` the coroutine created in PyO3:

```python
import asyncio
from py202 import order

asyncio.run(order("burger soda burger"))
```

As the coroutine crated does not depend on an event loop, it is running on multithread instead. Of cause it can also be awaited like before:

```python
import asyncio
from py202 import order

async def main():
    await order("burger soda burger")

asyncio.run(main())

```

Before we move on, have a revision on the [Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html) chapter of the Rust book. Since we would like to focus on using PyO3, we will no go over details about message passing and shared memory.

---

## Exercise 4 - Shared Python object

Now, since we already understand how tha basics works, let's imagin we are putting it in a more realistic system. Imagine we have a tiny but busy fastfood stand. Our design before won't work because:

1. We can spawn as many as the same type of items simultaneously as we want, realistically there are limitation. For example, if we only have 1 soda machine, we can only pour 1 soda at a time.
2. Now for each item in the order, once the order is created, the specific items required will be spawn and the order will not be completed untill those items specifically created for this order is completed. In a realistic fastfood restaurant, the items created will be put in a pool for the staff finishing off the item to grab. We would like to recreated this pool system.
3. So far we have only create 1 order at a time, realistically there will be many orders getting prepared simytaneously.

To accommodate this change in designs:

1. We will lock the spawning of the same type of items to be 1 at a time.
2. We will created a pool count for each type of items and these counts will be shared througout the program
3. We will use concurrency (multi-threading in Rust and async in Python) to make multiple orders

In Rust, we use Mutex to share data with multiple access point. The Mutex locking system can ensure that whatever it is holding can only be access by one point at a time. For reference counting, we will need a `Arc<T>` smart pointer for the Mutex since we are going to share the Mutex across multiple threads. For details see the [Shared-State Concurrency chapter in the Rust book](https://doc.rust-lang.org/book/ch16-03-shared-state.html)

```rust
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
...
fn order(input_str: String, burger_pool: &Arc<Mutex<usize>>, soda_pool: &Arc<Mutex<usize>>) -> () {
    ...
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

    fn main() {
        let burger_pool = Arc::new(Mutex::new(0));
        let soda_pool = Arc::new(Mutex::new(0));
        order("burger soda burger".to_string(), &burger_pool, &soda_pool);
    }
```

Now, to check on if the order is completed, we will check if there are enough items in the pools to complete the order, if there are, the items will be consumed and the order is marked as completed.

```rust
fn order(input_str: String, burger_pool: &Arc<Mutex<usize>>, soda_pool: &Arc<Mutex<usize>>) -> () {
    ...
    //count the number of each order
    let burger_count: usize = input_order.iter().filter(|&item| item == "burger").count();
    let soda_count: usize = input_order.iter().filter(|&item| item == "soda").count();
    
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
            is_burger_done = is_burger_done || try_consume(&burger_pool, burger_count);
            is_soda_done = is_soda_done || try_consume(&soda_pool, soda_count);
    
            if is_burger_done && is_soda_done {
                println!("order [{}] complete",input_str);
                break;
            }
        }
    };
    ...
```

Last thing, we will spawn multiple orders in different threads in main.

```rust
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
```

The completed code would probably looks like [`ex_4/pyburger.rs`](ex_4/pyburger.rs). Feel free to try it in a `main.rs` in a binary rust project. It seems working fine but there is a runtime issue in this code, can you spot it? If not, don't worry, it will be more apprant when we move things to the Python side and we will explain then.

Now we have a code in pure Rust that can handle multiple orders asynchronously, what about we create orders in python?

In PyO3 where we use the Python GIL, we do not have to use Mutex to lock the PyObjects as it can only be access by acquiring the GIL and therefoer ensuring the PyObject can only be access at one point at a time. So you can also say that the GIL act as a Mutex.

```rust
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
```

Instead of using a number to hold the number of burgers and soda available, we will use a list to "store" the items. This design change is made because of the flexibility of list and its methods in Python. It would have be more complicated if we use integers in Python instead. Another way to do is to create a custom Python object type. You can try implementing it as an extra challenge in this exercise.

If `call_method1` and `call_method0` looks unfamilar to you, please check [the documentation here](https://docs.rs/pyo3/latest/pyo3/struct.Py.html#method.call_method1) and [here](https://docs.rs/pyo3/latest/pyo3/struct.Py.html#method.call_method0), same as `Python::with_gil`, you can [see details here](https://docs.rs/pyo3/0.23.4/pyo3/marker/struct.Python.html#method.with_gil). 

Also note that the `counter` passing in here is a `Py<T>` pointer pointing to a `PyList` - it is different from the `Bound<'py, PyList>` which we will encounter later. With `Py<T>` pointer the references is not tied to the GIL and therefore can be passed into multiple threads in Rust. If you check the [documentation of `Py`](https://docs.rs/pyo3/latest/pyo3/struct.Py.html#), you will see that it has the `Send` and `Sync` trait implemented. This is not the same as the `Bound` which has [both of them NOT implemented](https://docs.rs/pyo3/latest/pyo3/struct.Bound.html#synthetic-implementations).

```rust
#[pyfunction]
fn order<'py>(
    py: Python,
    input_str: String,
    burger_pool: &Bound<'py, PyList>,
    soda_pool: &Bound<'py, PyList>,
) -> () {
    ...

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
    ...
```

For the `order` and `try_consume` function, we will replace `&Arc<Mutex<usize>>` with `&Bound<'py, PyList>` due to the explanation above. You may notice that `py` is not required at `call_method0` for `Bound` objects, it is because `Bound` objects are bounded to the Python GIL so it does not need to be passed in explicitly. We have also used `extract::<usize>` to convert a `PyInt` to `usize`.

If you put everything together, it should look somthing like [ex4/lib-0.rs](ex4/lib-0.rs). After `maturin develop` you may try it with a simple Python script:

```python
from py202 import order

burger_pool = []
soda_pool = []

order("burger soda burger soda soda", burger_pool, soda_pool)
order("burger soda burger", burger_pool, soda_pool)
```

Here we have not put the `order` into multi-threads yet, we will do it later. For now we just want to make sure it works.

You may notice that it ended up having an endless loop and the program is deadlocked. This is the result of our bad design.

You see that we have an infinite loop that checks checks if the required items has been prepared:

```rust
loop {
    is_burger_done = is_burger_done || try_consume(&burger_pool, burger_count);
    is_soda_done = is_soda_done || try_consume(&soda_pool, soda_count);

    if is_burger_done && is_soda_done {
        println!("order [{}] complete", input_str);
        break;
    }
}
```

This is bad Rust code as the loop is looping very fast will burn a lot of CPU time while the other thread `burger` and `soda` is trying to create items. One way to make it less bad is to introduce a small break time:

```rust
loop {
    sleep(Duration::from_millis(10));
    is_burger_done = is_burger_done || try_consume(&burger_pool, burger_count);
    is_soda_done = is_soda_done || try_consume(&soda_pool, soda_count);

    if is_burger_done && is_soda_done {
        println!("order [{}] complete",input_str);
        break;
    }
}
```

This works fine in pure Rust, however with PyO3 this is not enough (you can try it yourself). The deadlock is still there. This is because, while the loop is looping, even with the break, it is still holding on to the Python GIL. With the Python GIL locked there, there is no chance for the code in `burger`

```rust
Python::with_gil(|py| {
    let _ = counter.call_method1(py, "append", ("burger",)).unwrap();
    println!("burger made, # of burger: {}", counter.call_method0(py,"__len__").unwrap());
})
```

to acquire the GIL and update the `burger_pool`. This is the same for `soda`. To make sure the item pool got a change to update, we have to release the GIL while the checking loop is taking a break:

```rust
loop {
    py.allow_threads( || {
        sleep(Duration::from_millis(10));
    });
    is_burger_done = is_burger_done || try_consume(&burger_pool, burger_count);
    is_soda_done = is_soda_done || try_consume(&soda_pool, soda_count);

    if is_burger_done && is_soda_done {
        println!("order [{}] complete", input_str);
        break;
    }
}
```

Here `py.allow_threads` [temporary releasing the Python GIL](https://docs.rs/pyo3/0.23.4/pyo3/marker/struct.Python.html#method.allow_threads) to be acquired by other Python threads and will get it back once the `sleep` is finished.

Now if you try to `maturin develop` again and run the Python try script above, you will see that now the orders can be completed. However, they are only fulfilling the order one after another. Now, if we put the orders in different Python threads:

```python
from py202 import order
import threading

burger_pool = []
soda_pool = []

if __name__ == "__main__":
    t1 = threading.Thread(
        target=order, args=("burger soda burger soda soda", burger_pool, soda_pool)
    )
    t2 = threading.Thread(
        target=order, args=("burger soda burger", burger_pool, soda_pool)
    )

    t1.start()
    t2.start()

    t1.join()
    t2.join()

    print("Done!")
```

Then you will see similar behaviors as the pure Rust code that we have before.


---

## Exercise 5 - Thread-free Python

As you see having the Python GIL plays an important part as our code before, you may wonder, what if we are using thread-free Python which has no GIL? The answer is not simple.

For the full explanation, you may want to checkout the [relevant part in the PyO3 user guide](https://pyo3.rs/main/free-threading.html). For the short version, we can think of it like this. Instead of thinking function like `Python::with_gil` is acquiring the GIL, the relevant Rust thread is attached to the Python thread. And in a thread-free Python, instead of having only one Python thread, we will have multiple. To see that effect, we will do an experiment.

Remember we will run into a deadlock when not using  `py.allow_threads` around the break in the infinite loop? If we only have one Python thread allowed and it got held on by that loop, the item pool Python object can never be access thus creating a deadlock. In thread-free Python,  `py.allow_threads` would not be needed as there will be multiple Python thread running so it should be fine just like the code in [`ex_4/pyburger.rs`](ex_4/pyburger.rs). So let's do that.

First, remove the `py.allow_threads` in the loop:

```rust
loop {
    sleep(Duration::from_millis(10));
    is_burger_done = is_burger_done || try_consume(&burger_pool, burger_count);
    is_soda_done = is_soda_done || try_consume(&soda_pool, soda_count);

    if is_burger_done && is_soda_done {
        println!("order [{}] complete",input_str);
        break;
    }
}
```

Since in the current verion of PyO3 (version 0.23), having the GIL is still the default. To enable compiling module that is compatable and using the thread-free property of the thread-free Python, we will have to pass `gil_used = false` as a parameter to the `pymodule` procedural macro like this:

```rust
#[pymodule(gil_used = false)]
fn py202(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(order, m)?)?;
    Ok(())
}
```

Now let's set up our thread-free enviroment:

```
uv venv -p 3.13.2t .venv-free
source .venv-free/bin/activate
uv pip install maturin
python -m ensurepip --default-pip
```

Then run `maturin develop` again. Try running the same [`ex_4/try.py`](ex_4/try.py) and you will see that our theory is correct. Now it works as the same as pure Rust.

---

## Reference

This is the end of the workshop, there are much more in the usage of PyO3, however, we only have enough time to scratch the surface. Also, to make a usable Python package with PyO3, knowledge in Rust is needed. Here are links to resources that you can keep learning Rust and PyO3:

- [The Rust Book](https://doc.rust-lang.org/book/title-page.html)
- [asynchronous programming in Rust](https://rust-lang.github.io/async-book/intro.html)
- [Python documentation - Coroutines and Tasks](https://docs.python.org/3/library/asyncio-task.html)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Teach-rs (GitHub repo)](https://github.com/tweedegolf/teach-rs)
- [The PyO3 user guide](https://pyo3.rs/)
- [pyo3_asyncio documentation](https://docs.rs/pyo3-asyncio/0.20.0/pyo3_asyncio/index.html)

---

## Support this workshop

This workshop is created by Cheuk and is open source for everyone to use (under MIT license). Please consider sponsoring Cheuk's work via [GitHub Sponsor](https://github.com/sponsors/Cheukting).







