import copy
import random
import subprocess


def gen():
    N = random.randint(1, 10)
    R = random.randint(1, N)
    B = ''.join(random.choice(['o', '.']) for _ in range(N))
    return N, R, B


def solve(N, R, B):
    ans = 1001001001001

    print("[solve]")
    for bit in range(2 ** N):
        B_copy = list(copy.deepcopy(B))
        shoot_ts = []
        for i in range(N):
            if bit >> i & 1:
                shoot_ts.append(i)

        for t in shoot_ts:
            for diff in range(R):
                B_copy[min(t + diff, N - 1)] = 'o'

        if all(bi == 'o' for bi in B_copy):
            move_cnt = 0 if len(shoot_ts) == 0 else max(shoot_ts)
            ans = min(ans, len(shoot_ts) + move_cnt)

    return ans


while 1:
    N, R, B = gen()
    print(f"{N} {R}\n{B}", file=open("in", "w"))
    expected = solve(N, R, B)
    got_byte = subprocess.check_output("cargo run --bin b < in", shell=True)
    got = int(got_byte.decode().rstrip())
    if expected != got:
        print(f"[found contradiction] expected: {expected}, got={got}")
        print(f"{N} {R}\n{B}")
        break
