import threading
import time

def background_work(v):
    time.sleep(0.1)
    v.append(42)


v = []
assert len(v) == 0
thread = threading.Thread(target=background_work, args=[v])
thread.start()
# OOPS: I swapped these two lines.
assert 42 in v
thread.join()
