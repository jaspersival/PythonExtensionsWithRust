def print_fibonacci(first, second, length):
    if length == 0:
        return

    print(first + second, end=" ")

    print_fibonacci(second, first+second, length-1)