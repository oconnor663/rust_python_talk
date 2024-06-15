from threading import Thread, Lock

X = 0
X_LOCK = Lock()

def add_500k():
    global X, X_LOCK
    for _ in range(500_000):
        with X_LOCK:
            X += int(1)

thread1 = Thread(target=add_500k)
thread1.start()
thread2 = Thread(target=add_500k)
thread2.start()
thread1.join()
thread2.join()
print(X)
