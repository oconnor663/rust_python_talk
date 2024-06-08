class Mess:
    def __init__(self, size):
        print(f"Make a {size} mess.")
        self.size = size

    def __enter__(self):
        return self

    def __exit__(self, *_):
        print(f"Clean up the {self.size} mess.")

    def look(self):
        print(f"Look at this {self.size} mess!")


with Mess("huge") as huge_mess:
    huge_mess.look()
