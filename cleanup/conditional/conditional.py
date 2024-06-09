from secrets import randbits

class Mess:
    def __init__(self, size):
        print(f"Make a {size} mess.")
        self.size = size

    def cleanup(self):
        print(f"Clean up the {self.size} mess.")

    def look(self):
        print(f"Look at the {self.size} mess!")

    def __enter__(self):
        return self

    def __exit__(self, *_):
        self.cleanup()

def random_mess(mess1, mess2):
    if randbits(1):
        mess2.cleanup()
        return mess1
    else:
        mess1.cleanup()
        return mess2

mess1 = Mess("first")
mess2 = Mess("second")
with random_mess(mess1, mess2) as mess:
    mess.look()
