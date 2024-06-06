functions = []

for i in range(10):
    functions.append(lambda: print(i))

for f in functions:
    f()
