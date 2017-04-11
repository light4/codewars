class Fib:
    def __init__(self):
        self.previous = 0
        self.current = 0

    def __iter__(self):
        return self

    def __next__(self):
        if self.current == 0:
            next = 1
            self.previous = self.current
            self.current = next
        else:
            next = self.previous + self.current
            self.previous = self.current
            self.current = next
        return self.current


def product_Fib(prod):
    a = Fib()
    while a.previous * a.current < prod:
        next(a)
    if a.previous * a.current == prod:
        return [a.previous, a.current, True]
    else:
        return [a.previous, a.current, False]


def productFib(prod):
    a, b = 0, 1
    while prod > a * b:
        a, b = b, a + b
    return [a, b, prod == a * b]
