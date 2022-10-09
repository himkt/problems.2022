import copy
import random
import subprocess


while 1:
    n = random.randint(1, 10)
    arange = [i for i in range(1, n + 1)]
    a = copy.deepcopy(arange)
    random.shuffle(a)
    b = copy.deepcopy(arange)
    random.shuffle(b)

    with open("in", "w") as f:
        print(n, file=f)
        print(" ".join(map(str, a)), file=f)
        print(" ".join(map(str, b)), file=f)

    #expect = subprocess.check_output(["cargo", "run", "--bin", "b2", "<", "in"])
    #result = subprocess.check_output(["cargo", "run", "--bin", "b", "<", "in"])

    expect = subprocess.check_output("cargo run --release --bin b2 < in", shell=True)
    result = subprocess.check_output("cargo run --release --bin b < in", shell=True)

    print(expect, result)
    if expect != result:
        break
