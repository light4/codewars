#!/usr/bin/env python2

cache = {}

def partitions(n):
    if n in cache:
        return cache[n]
    result = {(n,)}
    for i in range(1, n):
        sub_parts = partitions(n - i)
        for sub_part in sub_parts:
            part = tuple(sorted(sub_part + (i,)))
            result.add(part)
    cache[n] = result
    return result

def prods(n):
    return set(map(lambda part: reduce(lambda a,b: a*b, part), partitions(n)))

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
    a_list = sorted(prods(n))
    result = "Range: {} Average: {:.2f} Median: {:.2f}".format(range_b(a_list), average(a_list), median(a_list))
    return result
