def solve_lazy(h, w):
    ret = []
    for hi in range(0, h + 1):
        for wi in range(0, w + 1):
            vs1 = [hi * w, (h - hi) * wi, (h - hi) * (w - wi)]
            vs1 = sorted(vs1)
            r1 = vs1[-1] - vs1[0]

            vs2 = [h * wi, hi * (w - wi), (h - hi) * (w - wi)]
            vs2 = sorted(vs2)
            r2 = vs2[-1] - vs2[0]
            ret.extend([r1, r2])

    for hi1 in range(0, h + 1):
        for hi2 in range(hi1, h + 1):
            vs = [hi1 * w, (hi2 - hi1) * w, (h - hi2) * w]
            vs = sorted(vs)
            r = vs[-1] - vs[0]
            ret.append(r)

    for wi1 in range(0, w + 1):
        for wi2 in range(wi1, w + 1):
            vs = [h * wi1, h * (wi2 - wi1), h * (w - wi2)]
            vs = sorted(vs)
            r = vs[-1] - vs[0]
            ret.append(r)

    # print(ret)
    return min(ret)


h, w = list(map(int, input().rstrip().split()))
print(solve_lazy(h, w))
