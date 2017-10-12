import random
import threading
import time

def pirate(earth):
    while True:
        if random.choice([True, False]):
            print("Yarr, my treasure be at ", len(earth))
            earth.append("Treasure")
        else:
            earth.append("Clues")
        time.sleep(0.01)


def niffler(earth):
    pos = 0;
    while True:
        if len(earth) < 1:
            continue
        if earth[pos] == "Treasure":
            print("Stole some treasure at pos ", pos)
            del earth[pos]
        pos += 1
        if pos >= len(earth):
            pos = 0
        time.sleep(0.015)

earth = []
nif = threading.Thread(target=niffler, args=(earth,), daemon=True)
nif.start()
rustbeard = threading.Thread(target=pirate, args=(earth,), daemon=True)
rustbeard.start()
def watcher(earth):
    while True:
        print("State of the earth: ", earth)
        time.sleep(2)
watcher = threading.Thread(target=watcher, args=(earth,), daemon=True)
watcher.start()
nif.join()
rustbeard.join();
watcher.join();
