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

def look_at_file_descriptors():
    output1 = ScreamingOutput("/dev/null")
    output2 = ScreamingOutput("/dev/null")
    print(output1.file.fileno(), output2.file.fileno())
    silly_map1 = {"output": output1}
    silly_map2 = {"output": output2}
    silly_map1["other_map"] = silly_map2
    silly_map2["other_map"] = silly_map1

look_at_file_descriptors()
look_at_file_descriptors()
look_at_file_descriptors()
