import subprocess
import datetime

m = 0

chars = 'a b c d e f g h'.split()
for n in range(1, 9):
    with open("in", "w") as f:
        print(f"{n} {m}", file=f)
        for c in chars[:n]:
            print(c, file=f)

    s = datetime.datetime.now()
    subprocess.call("python3 src/bin/d.py < in", shell=True, stdout=subprocess.DEVNULL)
    t = datetime.datetime.now()
    elapsed = t - s
    print(f"n={n}, elapsed={elapsed}")

subprocess.call("rm in", shell=True)
