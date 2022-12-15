import functools
import timeit

from .fibonacci import fibonacci


def time_fibonacci():
    print(f"Python: {timeit.timeit(functools.partial(fibonacci, 50), number=10000)}")

if __name__ == "__main__":
    # result = fibonacci(10)
    # print(result)
    time_fibonacci()
