# https://atcoder.jp/contests/abc273/submissions/38169395

n = int(input())
ans = 1

for i in range(1, n + 1):
  ans *= i

print(ans)
