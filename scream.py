#! /usr/bin/env python3

import argparse
import sys

parser = argparse.ArgumentParser()
parser.add_argument("paths", nargs="*")
parser.add_argument("-m", "--message", default="hello world\n")

class ScreamingOutput:
    def __init__(self, path=None):
        if path is not None:
            self.file = open(path, "w")
        else:
            self.file = None

    def write(self, string):
        all_caps = string.upper()
        if self.file is not None:
            self.file.write(all_caps)
        else:
            sys.stdout.write(all_caps)

args = parser.parse_args()
if args.paths:
    outputs = [ScreamingOutput(path) for path in args.paths]
else:
    outputs = [ScreamingOutput()]
for output in outputs:
    output.write(args.message)
