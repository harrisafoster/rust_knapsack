use std::thread;
use std::time::{Duration, Instant};
pub mod structs;
pub mod reconstruction;
pub mod tools;
pub mod reader;
use crate::algo::knapsack::run_algo;
use crate::structs::structs::Stock;
pub mod algo;
use pyo3::prelude::*;

#[pyfunction]
fn knapsack_algo() -> Vec<Stock> {
    let now: Instant = Instant::now();
    let res: Vec<Stock> = run_algo(500, "data.csv");
    let elapsed_single: Duration = now.elapsed();

    let now_multi: Instant = Instant::now();
    let thr1: thread::JoinHandle<Vec<Stock>> = thread::spawn(|| run_algo(500, "data.csv"));
    let thr2: thread::JoinHandle<Vec<Stock>> = thread::spawn(|| run_algo(500, "data.csv"));
    let thr3: thread::JoinHandle<Vec<Stock>> = thread::spawn(|| run_algo(500, "data.csv"));

    let res1: Vec<Stock> = thr1.join().unwrap();
    let res2: Vec<Stock> = thr2.join().unwrap();
    let res3: Vec<Stock> = thr3.join().unwrap();
    let elapsed_multi: Duration = now_multi.elapsed();
    let mut total_spent: f64 = 0.0;
    let mut total_earned: f64 = 0.0;
    res.iter().for_each(|s: &Stock| {
        total_spent += s.price;
        total_earned += s.earnings;
    });
    assert!(res == res1);
    assert!(res == res2);
    assert!(res == res3);

    println!("Elapsed 1: {:?}", elapsed_single);
    println!("Elapsed 3: {:?}", elapsed_multi);

    res
}

#[pymodule]
fn py_connect(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(knapsack_algo, m)?)?;
    Ok(())
}