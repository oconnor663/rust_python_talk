from threading import Lock


class Mutex:
    def __init__(self, val):
        self._val = val
        self._lock = Lock()

    def __enter__(self):
        self._lock.acquire()
        return self._val

    def __exit__(self, exc_type, exc_value, traceback):
        del exc_type, exc_value, traceback
        self._lock.release()


MY_MAP = Mutex({})

with MY_MAP as map:
    map["foo"] = 42

MAPS_LIST = []

with MY_MAP as map:
    MAPS_LIST.append(map)
