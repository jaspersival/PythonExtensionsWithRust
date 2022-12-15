def python_calculate_pi(iterations):
    pi = 0.0
    n = 0

    while n < iterations:
        pi += (4 / (n * 2 + 1) * (1 if n % 2 == 0 else -1))
        n += 1
    return pi