def main():
    T = int(input())
    for t in range(1, T+1):
        W, H, L, U, R, D = map(int, input().split())
        result = success_chance(W, H, L, U, R, D)
        print('Case #{}: {}'.format(t, result))

def success_chance(W, H, L, U, R, D) -> float:
    # 1-indexed in both dimensions!
    # Row indices wrap mod 2!
    grid = [[None] * (W+1) for h in range(2)]

    for h in range(1, H+1):
        for w in range(1, W+1):
            if h == w == 1:
                grid[h % 2][w] = 1.0
            elif U <= h <= D and L <= w <= R:
                grid[h % 2][w] = 0.0
            else:
                above = grid[(h-1) % 2][w] if h != 1 else 0.0
                if w == W:
                    above *= 2.0
                left = grid[h % 2][w-1] if w != 1 else 0.0
                if h == H:
                    left *= 2.0
                grid[h % 2][w] = (above + left) / 2

    return grid[H % 2][W]

main()
