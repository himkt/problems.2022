import subprocess


def compute_bruteforce(n):
    score = 0

    for k in range(1, n + 1):
        s = str(k)
        c = 0
        for si in s:
            if si == '1':
                c += 1
            else:
                break
        score += c

    return score


space = range(1, 100000)
points = len(space)
for n in space:
    print(f"{n:05d} / {points}\r", end="")
    expect = compute_bruteforce(n)
    with open("in", "w") as f:
        print(n, file=f)

    raw = subprocess.check_output(
        f"cargo run --bin a < in",
        shell=True,
        stderr=subprocess.DEVNULL,
    )
    actual = int(raw.decode("utf-8").rstrip())
    if expect != actual:
        print(f"n={n}, truth={expect}, actual={actual}")
        exit(1)

exit(0)
