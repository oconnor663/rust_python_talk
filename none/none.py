import sys

class Output:
    def __init__(self, maybe_path):
        if maybe_path:
            self.file = open(maybe_path, "w")
        else:
            self.file = None

    def write(self, string):
        if self.file:
            return self.file.write(string)
        else:
            return sys.stdout.write(string)

    def close(self):
        self.file.close()

path = sys.argv[1] if len(sys.argv) > 1 else None
output = Output(path)
output.write("hello world\n")
output.close()
