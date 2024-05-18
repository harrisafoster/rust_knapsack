use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};
pub mod reader;
pub mod reconstruction;
pub mod structs;
pub mod tools;
use crate::algo::knapsack::run_algo;
pub mod algo;

fn main() {
    let now_single: Instant = Instant::now();
    let res: HashMap<&str, f64> = run_algo(500, "data.csv");
    let elapsed_single: Duration = now_single.elapsed();
    println!("Elapsed single: {:.2?}", elapsed_single);
    println!("{:?}", res);

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
    println!("Elapsed multi8: {:.2?}", elapsed_multi);
    println!("{:?}", res);
}
