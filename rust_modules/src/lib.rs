use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};
pub mod structs;
pub mod reconstruction;
pub mod tools;
pub mod reader;
use crate::algo::knapsack::run_algo;
pub mod algo;
use pyo3::prelude::*;

#[pyfunction]
fn knapsack_algo() -> HashMap<&'static str, f64> {
    let now: Instant = Instant::now();
    let mut res: HashMap<&str, f64> = run_algo(500, "data.csv");
    let elapsed: Duration = now.elapsed();
    res.insert("speed1", elapsed.as_secs_f64());

    let now_multi: Instant = Instant::now();
    let mut handles: Vec<thread::JoinHandle<HashMap<&str, f64>>> = Vec::new();

    let res1: thread::JoinHandle<HashMap<&str, f64>> = thread::spawn(|| run_algo(500, "data.csv"));
    handles.push(res1);
    let res2: thread::JoinHandle<HashMap<&str, f64>> = thread::spawn(|| run_algo(500, "data.csv"));
    handles.push(res2);
    let res3: thread::JoinHandle<HashMap<&str, f64>> = thread::spawn(|| run_algo(500, "data.csv"));
    handles.push(res3);
    let res4: thread::JoinHandle<HashMap<&str, f64>> = thread::spawn(|| run_algo(500, "data.csv"));
    handles.push(res4);
    let res5: thread::JoinHandle<HashMap<&str, f64>> = thread::spawn(|| run_algo(500, "data.csv"));
    handles.push(res5);
    let res6: thread::JoinHandle<HashMap<&str, f64>> = thread::spawn(|| run_algo(500, "data.csv"));
    handles.push(res6);
    let res7: thread::JoinHandle<HashMap<&str, f64>> = thread::spawn(|| run_algo(500, "data.csv"));
    handles.push(res7);
    let res8: thread::JoinHandle<HashMap<&str, f64>> = thread::spawn(|| run_algo(500, "data.csv"));
    handles.push(res8);
    for handle in handles.drain(..) {
        handle.join().unwrap();
    }
    let elapsed_multi: Duration = now_multi.elapsed();
    res.insert("speed8", elapsed_multi.as_secs_f64());

    res
}

#[pymodule]
fn py_connect(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(knapsack_algo, m)?)?;
    Ok(())
}