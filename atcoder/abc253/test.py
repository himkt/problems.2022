import random
import subprocess


N = 10


def gen_bruteforce():
    for n in range(1, N + 1):
        for a in range(1, N + 1):
            for b in range(1, N + 1):
                yield n, a, b


def gen_random():
    while 1:
        n = random.randint(1, N)
        a = random.randint(1, N)
        b = random.randint(1, N)
        yield n, a, b


for n, a, b in gen_bruteforce():
    print(f"{n} {a} {b}")

    all = list(range(1, n + 1))
    all = [ci for ci in all if ci % a != 0 and ci % b != 0]

    print(f"{n} {a} {b}", file=open("in", "w"))
    output = subprocess.check_output("cargo run --bin d < in", shell=True)
    v = int(output.decode().rstrip())

    expected = sum(all)
    if v != expected:
        print(f"{n} {b} {b}, got={v}, expected={expected}")
        break
