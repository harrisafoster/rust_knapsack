# RustKnapsack

## Goal

Goal:

Prove that Rust increases Python's speed significantly via Rust import

## Local Dev

### Base Requirements

- Python >= 3.12.3
- Rust >= 1.78.0
- Ubuntu 23 (Mantic Minotaur)

### Installation
##### Clone the source directly from Github:
```sh
$ git clone https://github.com/harrisafoster/rust_knapsack.git
$ cd rust_knapsack
```
##### Create & activate your Python virtual environment:
```sh
$ python3 -m venv .venv
$ source ./.venv/bin/activate
```
##### Install required packages with:
```sh
$ pip install -r requirements.txt
```
##### Build/install rust module with:
```sh
$ cd rust_knapsack/rust_modules/
$ cargo build --release
$ maturin build --release
```

### Use
#### To run the Python version simply use these commands (with .venv activated):
```sh
$ cd rust_knapsack/
$ python optimization.py
```

#### To build the Rust executable and run it:
```sh
$ cd rust_knapsack/rust_modules/
$ cargo build --release
$ cd ..
$ ./rust_modules/target/release/stock_algo 
```

#### To build the Rust Python package and run it:
```sh
$ cd rust_knapsack/rust_modules/
$ maturin develop --release
$ cd ..
$ python rust_connect.py
```

### Results

As you can see, Rust can accelerate Python code with NO added lag from import. It is the best choice for modules associated with Python with the goal of performance acceleration.

### Metrics on my PC

#### PC specs
CPU: 12 cores @ 3.7ghz
RAM: 16gb @ 16gb
SSD: 1tb

#### Local Results
Local results yield:
```sh
$ cd rust_knapsack/
$ ./rust_modules/target/release/stock_algo 
# Single 1.02s, multi8 2.81s
$ python rust_connect.py 
# Single 1.03s, multi8 2.75s
$ python optimization.py
# Single 43s
```

By these results, it is obvious that Rust adds no lag for its results, and accelrates Python code significantly.