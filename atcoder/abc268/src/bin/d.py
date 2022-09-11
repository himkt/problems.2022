import itertools
import collections


def validate(username) -> bool:
    return username not in t and 3 <= len(username) <= 16


n, m = list(map(int, input().rstrip().split()))
s = [input().rstrip() for _ in range(n)]
t = set(input().rstrip() for _ in range(m))

if n == 1:
    if validate(s[0]):
        print(s[0])
    else:
        print("-1")
    exit(0)

num_chars = sum(len(si) for si in s)
num_init_underscores = n - 1
num_total_chars = num_chars + num_init_underscores
if num_total_chars > 16:
    print("-1")
    exit(0)

num_remain = 16 - num_total_chars
positions = [i for i in range(num_init_underscores)]
arange = [i for i in range(n)]
for perm in itertools.permutations(arange):
    username_fragments = [s[i] for i in perm]

    queue: collections.deque = collections.deque()
    queue.append(([], 0))

    while len(queue) > 0:
        (candidates, buf) = queue.popleft()
        if buf > num_remain:
            continue

        num_underscores = [1 for _ in range(num_init_underscores)]
        for p in candidates:
            num_underscores[p] += 1

        username = ""
        for i in range(n):
            username += username_fragments[i]
            if i != n - 1:
                username += "_" * num_underscores[i]

        if validate(username):
            print(username)
            exit(0)

        for position in positions:
            new_buf = buf + 1
            if num_total_chars + new_buf <= 16:
                queue.append((candidates + [position], new_buf))

print("-1")

