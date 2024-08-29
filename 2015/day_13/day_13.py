#!/usr/bin/env python3

import sys
import itertools
import math

def permute(n: int, happy: list[list[int]]) -> int:
  max_happiness_delta = -math.inf
  for c in itertools.permutations(range(n)):
    delta = happy[c[-1]][c[0]] + happy[c[0]][c[-1]]
    for p in itertools.pairwise(c):
      delta += happy[p[0]][p[1]] + happy[p[1]][p[0]]
    max_happiness_delta = max(max_happiness_delta, delta)
  return max_happiness_delta

knights = {}
happy = []

for line in sys.stdin:
  a, _, sign, mag, *_, b = line[:-2].split(' ')
  x = knights.setdefault(a, len(knights))
  y = knights.setdefault(b, len(knights))
  # TODO: Consider folding two pairwise happiness into one i.e. A-B + B-A.
  happy.extend([[] for _ in range(x+1-len(happy))])
  happy[x].extend([0 for _ in range(y+1-len(happy[x]))])
  happy[x][y] = int(mag) * (-1 if sign[0] == 'l' else 1)

count_knights = max(knights.values()) + 1

# Day 1
print(permute(count_knights, happy))
