from typing import List

def main():
    T = int(input())
    for t in range(1, T+1):
        N, D = map(int, input().split())
        X = list(map(int, input().split()))
        assert len(X) == N
        result = latest_start(X, D)
        print('Case #{}: {}'.format(t, result))

def latest_start(bus_freqs: List[int], end_day: int) -> int:
    '''
    Days are numbered from 1 to end_day inclusive.
    If bus_freqs[-1] is 4, then the last bus only runs on days
    that are multiples of 4: 4, 8, 12, etc...

    Find the latest day to take bus[0] s.t. you can schedule all
    busses one after the other (two on the same day is allowed) and
    be done on or before end_day.

    - take last bus; find largest # divisible by it and <= end_day
      - if that # is 0, throw (promise violated)
    - recurse on list[:-1] and that #  (return the result)
    - base case: empty list; return end_day
    '''
    assert all(map((lambda x: x > 0), bus_freqs))
    assert end_day >= 1

    # Had to re-write iteratively because of MLE.
    '''
    if not bus_freqs:
        return end_day
    else:
        x = bus_freqs[-1]

        d = largest_multiple_le(x, end_day)
        assert d > 0

        return latest_start(bus_freqs[:-1], d)
    '''

    while bus_freqs:
        # Schedule the last bus.
        x = bus_freqs[-1]
        d = largest_multiple_le(x, end_day)
        assert d > 0

        # Consider the subproblem arising from bus_freqs[:-1].
        bus_freqs.pop()
        end_day = d

    return end_day

def largest_multiple_le(x: int, t: int) -> int:
    '''Return the largest multiple of x that is <= t.'''
    assert x >= 0 and t >= 0
    if x == 0:
        return 0
    else:
        return x * (t // x)

main()
