import operator

def partsOfLen_b(n, l, i):
    return list(map(lambda x: operator.add([i], x), partsOfLen((n - i), (l - 1))))

def partsOfLen(n, l):
    if l == 1:
        return [[n]]
    else:
        return foldl(operator.add, [], list(map(lambda i: partsOfLen_b(n, l, i), range(1, n // l + 1))))

def partitions(n):
    result = []
    for i in range(1, n + 1):
        result += partsOfLen(n, i)
    return result

def foldl(op, i, a_list):
    result = i
    for i in a_list:
        result = op(result, i)
    return result

def prod(n):
    return set(map(lambda x: foldl(operator.mul, 1, x), partitions(n)))

def range_b(a_list):
    return max(a_list) - min(a_list)

def average(a_list):
    return float(sum(a_list)) / len(a_list)

def median(a_list):
    n = len(a_list)
    if n % 2 == 0:
        return (a_list[n // 2] + a_list[n // 2 - 1]) / 2.0
    else:
        return a_list[(n - 1) // 2]

def part(n):
    a_list = sorted(prod(n))
    result = "Range: {} Average: {:.2f} Median: {:.2f}".format(range_b(a_list), average(a_list), median(a_list))
    return result
