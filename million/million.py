from threading import Thread

X = 0

def add_500k():
    global X
    for _ in range(500_000):
        X += int(1)

thread1 = Thread(target=add_500k)
thread1.start()
thread2 = Thread(target=add_500k)
thread2.start()
thread1.join()
thread2.join()
print(X)
