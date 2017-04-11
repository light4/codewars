#!/bin/env python3

import unittest

def destroy(input_set=None):
    base = "a b c d e f g h i j k l m n o p q r s t u v w x y z"
    for input_lst in input_set:
        for c in input_lst:
            base = base.replace(c, '_')

    return base

class TestDestroy(unittest.TestCase):
    def test_basic1(self):
        input_set = (['A', 'b'], ['C', 'd'])
        self.assertEqual(destroy(input_set), "a _ c _ e f g h i j k l m n o p q r s t u v w x y z")

    def test_basic2(self):
        input_set = (['B', 'b'], ['C', 'm', 'f'])
        self.assertEqual(destroy(input_set), "a _ c d e _ g h i j k l _ n o p q r s t u v w x y z")


if __name__ == '__main__':
    unittest.main()
