'''
In general, we want to split the paths into disjoint subsets that we can count easier.

Consider the column of squares above (R, U): [(R, y) with 1 <= y < U].  A path last touches this column by reaching this square from (1, 1) -> (R, y), then going right.  The probability of this is binom(dx + dy, dx) / 2^(dx + dy + 1), where dx = R-1 and dy = y-1: there are dx + dy + 1 moves with 2 choices, and the number of move patterns that result in touching (R, y) last is binom(dx + dy, dx).

Similarly, there is a row to the left of (L, D), and paths can touch that row.
Each path that reaches the goal only last touches our specified row or column at a unique point, so these are disjoint probabilities that we can add.

To handle the case of very large numbers, we use logarithms.
'''

def solve(W, H, L, U, R, D):
    logfac = [0]
    for x in xrange(1, W + H + 5):
        logfac.append(logfac[-1] + math.log(x))

    def ways(x1, y1, x2, y2):  # log half prob. x1, y1 -> x2, y2
        dx, dy = x2 - x1, y2 - y1
        binom = logfac[dx+dy] - logfac[dx] - logfac[dy]
        return binom - (dx + dy + 1) * math.log(2)

    ans = 0
    if D < H:
        for x in range(1, L):
            ans += math.exp(ways(1, 1, x, D))
    if R < W:
        for y in range(1, U):
            ans += math.exp(ways(1, 1, R, y))
    return ans
