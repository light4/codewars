import math

def removNb(n):
    result = []
    total = n * (n + 1) / 2
    min_a = int(((n - 1) * n / 2) / (n + 1))
    max_a = int(math.sqrt(total + 1))
    for a in range(min_a, max_a):
        b = int((total - a) / (a + 1))
        if a * b + a + b == total:
            result.append((a, b))
            result.append((b, a))
    result.sort(key=lambda x: x[0])
    return result
