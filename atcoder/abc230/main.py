n = int(input().rstrip())
print("input: ", n)
for ni in reversed(range(1, n + 1)):
    print(f"{n} / {ni:2d} => {n // ni}")
