import threading
from time import time, sleep

program_start = time()


def thread():
    sleep(3)
    print("Sleep thread done, elapsed:", time() - program_start)


threading.Thread(target=thread).start()

# Do 5-second calculation in main thread:
calc_start = time()
while time() < calc_start + 5:
    sum(range(10000))
print("Main thread done, elapsed:", time() - program_start)
