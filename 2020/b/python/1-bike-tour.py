from typing import List

def main():
    T = int(input())
    for t in range(1, T+1):
        N = int(input())
        H = list(map(int, input().split()))
        ans = num_peaks(H)
        print('Case #{}: {}'.format(t, ans))

def num_peaks(heights: List[int]) -> int:
    n = len(heights)
    count = 0
    for i in range(1, n-1):  # ignore first and last
        if heights[i-1] < heights[i] > heights[i+1]:
            count += 1
    return count

main()
