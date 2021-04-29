def main():
    T = int(input())
    for t in range(1, T+1):
        program = input()
        x, y = evaluate(program)
        x %= 10**9
        y %= 10**9
        print('Case #{}: {} {}'.format(t, x+1, y+1))

def evaluate(program: str) -> (int, int):
    if not program:
        return (0, 0)
    else:
        head, rest = program[0], program[1:]
        if head in 'NESW':
            dx, dy = cardinal(head)
            x, y = evaluate(rest)
            return (dx + x, dy + y)
        else:
            head = int(head)
            assert 2 <= head <= 9
            inner, rest = get_subprogram(rest)
            x1, y1 = evaluate(inner)
            x2, y2 = evaluate(rest)
            return (head*x1 + x2, head*y1 + y2)

def get_subprogram(rest: str) -> (str, str):
    assert rest and rest[0] == '('
    depth = 0
    for i, c in enumerate(rest):
        if c == '(':
            depth += 1
        elif c == ')':
            depth -= 1

        if depth == 0:
            return rest[1:i], rest[i+1:]
    assert False

def cardinal(d: str) -> (int, int):
    if d == 'N':
        return (0, -1)
    elif d == 'E':
        return (1, 0)
    elif d == 'S':
        return (0, 1)
    else:
        assert d == 'W'
        return (-1, 0)

main()
