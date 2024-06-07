functions = []

for i in range(10):
    # OOPS: This captures i by reference.
    functions.append(lambda: print(i))

for f in functions:
    f()
