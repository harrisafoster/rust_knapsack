# RustKnapsack

## Goal

Prove that Rust increases Python's speed significantly via Rust import

## Local Dev

### Base Requirements

- Python >= 3.12.3
- Rust >= 1.78.0
- C/C++ build tools for your OS
- Terminal access (ideally Bash)

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
##### Build/install rust modules with:
```sh
$ cd rust_knapsack/rust_modules/
$ cargo build --release
$ maturin develop --release
```

### Use
#### To run the pure Python version simply run these commands (with .venv activated):
```sh
$ cd rust_knapsack/
$ python optimization.py
```

#### To run the pure Rust version simply run these commands:
```sh
$ cd rust_knapsack/
$ ./rust_modules/target/release/stock_algo 
```

#### To run the Rust/Python hybrid version simply run these commands (with .venv activated):
```sh
$ cd rust_knapsack/
$ python rust_connect.py
```

### Results

### Metrics on my PC

#### PC specs
CPU: 12 cores @ 3.7ghz - AMD Ryzen 9
RAM: 16gb DDR5
SSD: 1tb

#### Local Results
Local results yield:
```sh
$ cd rust_knapsack/
$ ./rust_modules/target/release/stock_algo 
# Single 1.02s, multi3 1.3s
$ python rust_connect.py 
# Single 1.03s, multi3 1.26s
$ python optimization.py
# Single 43s
```

By these results, it is obvious that Rust adds no lag for its results, and accelerates Python code significantly. In addition to that, multithreading is natively supported and accelerates the code even further.

#### Compatibility

As you'll notice in the file ./rust_modules/src/structs.rs, it is easy to create serializers to convert Rust objects into Python objects. This can also be observed when you run python rust_connect.py where you can see clearly that the result is represented as a list of dictionaries containing various data types within them. This compatibility was easy and fairly intuitive, which is not a negligible improvement in so far as "quality of life" is concerned. 