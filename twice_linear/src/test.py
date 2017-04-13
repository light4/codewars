import bisect

def dbl_linear_b(n):
    index = 0
    test = [1]
    while index < n - 1:
        next_1 = test[index] * 2 + 1
        next_2 = test[index] * 3 + 1
        index_1 = bisect.bisect_left(test, next_1)
        if len(test) == index_1 or next_1 != test[index_1]:
            test.insert(index_1, next_1)
        index_2 = bisect.bisect_left(test, next_2)
        if len(test) == index_2 or next_2 != test[index_2]:
            test.insert(index_2, next_2)
        index += 1
    return test[n]


from collections import deque

def dbl_linear(n):
    u, q2, q3 = 1, deque([]), deque([])
    for _ in range(n):
        q2.append(2 * u + 1)
        q3.append(3 * u + 1)
        u = min(q2[0], q3[0])
        if u == q2[0]: q2.popleft()
        if u == q3[0]: q3.popleft()
    return u
