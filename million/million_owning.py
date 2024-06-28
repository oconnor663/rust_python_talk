from threading import Lock, Thread

class OwningMutex:
    def __init__(self, val):
        self._val = val
        self._lock = Lock()

    def __enter__(self):
        self._lock.acquire()
        return self._val

    def __exit__(self, *_):
        self._lock.release()

X = OwningMutex([0])

def add_500k():
    global X
    for _ in range(500_000):
        with X as x:
            x[0] += int(1)
        x[0] += int(1)

thread1 = Thread(target=add_500k)
thread1.start()
thread2 = Thread(target=add_500k)
thread2.start()
thread1.join()
thread2.join()
with X as x:
    print(x[0])
