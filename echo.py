#! /usr/bin/env python3

import argparse

parser = argparse.ArgumentParser()
parser.add_argument("words", nargs="*")
parser.add_argument("-n", action="store_true")

args = parser.parse_args()
print(" ".join(args.words).upper(), end='')
if not args.n:
    print()
