import functools
import timeit

from .time_fibonacci import time_fibonacci
from alliander_rust_functions import rust_fibonacci


def time_fibonacci_rust():
    print(f"Rust: {timeit.timeit(functools.partial(rust_fibonacci, 50), number=10000)}")

if __name__ == "__main__":
    # result = fibonacci(10)
    # print(result)
    time_fibonacci_rust()
    time_fibonacci()
