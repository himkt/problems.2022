import copy
import random
import subprocess

while 1:
    #n = random.randint(1, 100_000)
    #m = random.randint(1, 1_000_000_000)
    n = random.randint(1, 3)
    m = random.randint(1, 10_000)
    arange = list(i for i in range(1, n + 1))
    permut = copy.deepcopy(arange)
    random.shuffle(permut)
    with open("in", "w") as f:
        print(f"{n} {m}", file=f)
        print(" ".join(map(str, permut)), file=f)

    r1 = subprocess.check_output("cargo run --bin b  --release < in", shell=True)
    r2 = subprocess.check_output("cargo run --bin b2 --release < in", shell=True)
    print(r1.decode())
    print(r2.decode())
    if r1 != r2:
        break
