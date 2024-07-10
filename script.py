#! /usr/bin/env python3

import argparse
import re
import urllib.request

parser = argparse.ArgumentParser()
parser.add_argument("url")
parser.add_argument("regex")
parser.add_argument("-i", "--insensitive", action="store_true")

args = parser.parse_args()
flags = 0
if args.insensitive:
    flags |= re.IGNORECASE
regex = re.compile(args.regex, flags)
with urllib.request.urlopen(args.url) as response:
    for line in response.read().splitlines():
        line = line.decode()
        if regex.search(line):
            print(line.strip())
