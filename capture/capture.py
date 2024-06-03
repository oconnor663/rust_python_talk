functions = []
for i in range(10):
    def i_plus_1():
        return i + 1
    functions.append(i_plus_1)
print([f() for f in functions])
