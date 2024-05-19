use std::thread;
use std::time::{Duration, Instant};
pub mod reader;
pub mod reconstruction;
pub mod structs;
pub mod tools;
use crate::algo::knapsack::run_algo;
use crate::structs::structs::Stock;
pub mod algo;

fn main() {
    let now_single: Instant = Instant::now();
    let res: Vec<Stock> = run_algo(500, "data.csv");
    let elapsed_single: Duration = now_single.elapsed();
    println!("Elapsed single: {:.2?}", elapsed_single);

    let now_multi: Instant = Instant::now();

    let thr1: thread::JoinHandle<Vec<Stock>> = thread::spawn(|| run_algo(500, "data.csv"));
    let thr2: thread::JoinHandle<Vec<Stock>> = thread::spawn(|| run_algo(500, "data.csv"));
    let thr3: thread::JoinHandle<Vec<Stock>> = thread::spawn(|| run_algo(500, "data.csv"));

    let res1: Vec<Stock> = thr1.join().unwrap();
    let res2: Vec<Stock> = thr2.join().unwrap();
    let res3: Vec<Stock> = thr3.join().unwrap();
    let elapsed_multi: Duration = now_multi.elapsed();
    println!("Elapsed multi3: {:.2?}", elapsed_multi);
    let mut total_spent: f64 = 0.0;
    let mut total_earned: f64 = 0.0;
    res.iter().for_each(|s: &Stock| {
        total_spent += s.price;
        total_earned += s.earnings;
    });
    println!("Spent: {:?}", total_spent);
    println!("Earned: {:?}", total_earned);
    println!("{:?}", res);
    assert!(res == res1);
    assert!(res == res2);
    assert!(res == res3);
}
