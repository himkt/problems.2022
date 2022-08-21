

n = int(input().rstrip())
a = list(map(int, input().rstrip().split()))

ret = []
for i in range(1 << n):
    b = []
    for q in range(n):
        if (i >> q) & 1:
            b.append(a[q])
    if not b:
        continue
    ret.append(b)

print(sorted(ret))
cnt = {}
minmax = []
for b in ret:
    bmin = min(b)
    bmax = max(b)
    cnt[bmin] = cnt.get(bmin, [])
    cnt[bmin].append(bmax)
    print(bmin, bmax)
    minmax.append((bmin, bmax))

print(sorted(minmax))

#print(cnt)
print()
for key in sorted(cnt.keys()):
    _cnt = {}
    for _key in cnt[key]:
        _cnt[_key] = _cnt.get(_key, 0)
        _cnt[_key] += 1
    #print(key, cnt[key])
    print(key, _cnt)

ans = 0
for mn, mx in minmax:
    ans += mn * mx
    #ans %= 998244353

print(f"ans={ans}")
