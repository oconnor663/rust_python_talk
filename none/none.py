import sys

class ScreamingOutput:
    def __init__(self, maybe_path):
        if maybe_path:
            self.file = open(maybe_path, "w")
        else:
            self.file = None

    def write(self, string):
        all_caps = string.upper()
        if self.file:
            return self.file.write(all_caps)
        else:
            return sys.stdout.write(all_caps)

path = sys.argv[1] if len(sys.argv) > 1 else None
output = ScreamingOutput(path)
output.write("hello world\n")
