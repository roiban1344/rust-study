def transform(x: int) -> int:
    d: list[int] = []
    while x > 0:
        q = x // 10
        d.append(x - q * 10)
        x = q
    d.sort()
    y = 0
    for e in reversed(d):
        y = y * 10 + e
    z = 0
    for e in d:
        z = z * 10 + e
    return y - z

def main():
    n = 1729
    for _ in range(10):
        n = transform(n)
        print(n)

main()