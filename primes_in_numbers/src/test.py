from collections import OrderedDict


def prime_factors(n):
    result = OrderedDict()
    i = 2
    test = n
    while i <= test:
        if test % i == 0:
            test /= i
            result[str(i)] = result.get(str(i), 0) + 1
        else:
            i += 1

    output = ""
    for k, v in result.items():
        if v != 1:
            output += "({}**{})".format(k, v)
        else:
            output += "({})".format(k)

    return output
