n, k = list(map(int, input().rstrip().split()))
print(f"n={n}, k={k}")

for a in range(1, n + 1):
    for b in range(1, n + 1):
        for c in range(1, n + 1):
            ab = a + b
            bc = b + c
            ca = c + a
            if ab % k == 0 and bc % k == 0 and ca % k == 0:
                print(a, b, c)
