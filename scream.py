import sys

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

if len(sys.argv) > 1:
    output = ScreamingOutput(sys.argv[1])
else:
    output = ScreamingOutput()

output.write("hello world\n")
