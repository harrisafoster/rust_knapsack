import time
import py_connect

if __name__ == "__main__":     
    start = time.time()  
    res = py_connect.knapsack_algo()
    duration = time.time() - start
    print(f"Total time with Python call: {duration}")
    print(res, type(res))
    total_price = 0
    total_earned = 0
    for stock in res:
        total_price += stock['price']
        total_earned += stock['earnings']
    print(f"Spent: {total_price}, Earned: {total_earned}")