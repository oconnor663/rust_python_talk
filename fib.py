#! /usr/bin/env python3

import sys


def fib(x):
    if x <= 1:
        return 1
    else:
        return fib(x - 1) + fib(x - 2)


x = int(sys.argv[1])
print(fib(x))
