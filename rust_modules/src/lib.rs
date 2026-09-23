use std::thread;
use std::time::{ Duration, Instant };
use pyo3::{ prelude::*, types::PyDict };

pub mod algo;
pub mod reader;
pub mod reconstruction;
pub mod structs;
pub mod tools;
pub mod embedded_data;

use crate::algo::knapsack::run_algo;
use crate::structs::structs::Stock;
use crate::embedded_data::STOCK_DATA;

#[pyfunction]
fn knapsack_algo(py: Python<'_>) -> PyResult<Vec<Py<PyDict>>> {
    let now: Instant = Instant::now();

    let res: Vec<Stock> = run_algo(500, STOCK_DATA);

    let elapsed_single: Duration = now.elapsed();

    let now_multi: Instant = Instant::now();

    let thr1: thread::JoinHandle<Vec<Stock>> = thread::spawn(|| run_algo(500, STOCK_DATA));

    let thr2: thread::JoinHandle<Vec<Stock>> = thread::spawn(|| run_algo(500, STOCK_DATA));

    let thr3: thread::JoinHandle<Vec<Stock>> = thread::spawn(|| run_algo(500, STOCK_DATA));

    let res1: Vec<Stock> = thr1.join().expect("Thread 1 panicked");
    let res2: Vec<Stock> = thr2.join().expect("Thread 2 panicked");
    let res3: Vec<Stock> = thr3.join().expect("Thread 3 panicked");

    let elapsed_multi: Duration = now_multi.elapsed();

    let total_spent: f64 = res
        .iter()
        .map(|stock| stock.price)
        .sum();
    let total_earned: f64 = res
        .iter()
        .map(|stock| stock.earnings)
        .sum();

    assert_eq!(res, res1);
    assert_eq!(res, res2);
    assert_eq!(res, res3);

    println!("Elapsed 1: {:?}", elapsed_single);
    println!("Elapsed 3: {:?}", elapsed_multi);
    println!("Total spent: {total_spent}");
    println!("Total earned: {total_earned}");

    res.into_iter()
        .map(|stock| stock.into_python(py))
        .collect()
}

#[pymodule]
fn py_connect(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(knapsack_algo, m)?)?;

    Ok(())
}
