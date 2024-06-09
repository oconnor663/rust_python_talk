import sys

class Output:
    def __init__(self):
        if len(sys.argv) > 1:
            self.file = open(sys.argv[1], "w")
        else:
            self.file = None

    def write(self, string):
        if self.file:
            return self.file.write(string)
        else:
            return sys.stdout.write(string)

    def close(self):
        self.file.close()

output = Output()
output.write("hello world\n")
output.close()
