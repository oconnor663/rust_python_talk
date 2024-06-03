#! /usr/bin/env python3

import threading
import time

def background_work(v):
    time.sleep(0.1)
    v.append(42)

def main():
    v = []
    assert len(v) == 0
    thread = threading.Thread(target=background_work, args=[v])
    thread.start()
    # OOPS: swap these two lines
    thread.join()
    assert 42 in v

main()
