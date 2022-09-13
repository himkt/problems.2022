n = int(input().rstrip())
a = list(map(int, input().rstrip().split()))

ans = 0
while len(a) > 1:
    a = sorted(a)
    print(a)
    amin = a[0]
    amax = a[-1]
    p = amax % amin
    a.pop()
    if p != 0:
        a.append(p)
    ans += 1

print(a)
print(ans)
