def merge_count(a):
    n = len(a)
    if n <= 1:
        return 0

    cnt = 0
    b = a[0:(n // 2)]
    c = a[(n // 2):]

    cnt += merge_count(b)
    cnt += merge_count(c)

    ai = 0
    bi = 0
    ci = 0

    while ai < n:
        if bi < len(b) and (ci == len(c) or b[bi] <= c[ci]):
            a[ai] = b[bi]
            ai += 1
            bi += 1
        else:
            cnt += (n // 2) - bi
            a[ai] = c[ci]
            ai += 1
            ci += 1

    return cnt


CHAR2NUM = {c: i for i, c in enumerate("atcoder")}
s = [CHAR2NUM[c] for c in input().rstrip()]
print(merge_count(s))
