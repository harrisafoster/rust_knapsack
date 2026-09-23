import os
import time
import py_connect


def main() -> None:
    """Run the Rust knapsack implementation through its Python binding."""
    start = time.perf_counter()

    result = py_connect.knapsack_algo()

    duration = time.perf_counter() - start

    print(f"Total time with Python call: {duration:.6f} seconds")
    print(result, type(result))

    total_price = sum(stock["price"] for stock in result)
    total_earned = sum(stock["earnings"] for stock in result)

    print(f"Spent: {total_price}")
    print(f"Earned: {total_earned}")

    if os.name == "nt":
        input("\nPress Enter to close...")


if __name__ == "__main__":
    main()