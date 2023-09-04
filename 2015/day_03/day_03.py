#!/usr/bin/env python3

import sys

input_ = b''.join(line.rstrip('\n').encode() for line in sys.stdin)

# use ord to convert bytes into int as iteration of bytes yeild int
signs = {ord(b'<'): (-1, 0),
         ord(b'>'): ( 1, 0),
         ord(b'v'): ( 0, 1),
         ord(b'^'): ( 0,-1)}
pos = (0, 0)
visited = {(0, 0): 1}

# Part 1
for s in input_:
  step = signs[s]
  pos = (step[0] + pos[0], step[1] + pos[1])
  visited[pos] = visited.get(pos, 0) + 1

print(f'Unique houses visited: {len(visited)}')

# Part 2
visited = {(0, 0): 2}
pos = [(0, 0), (0, 0)]
for i, s in enumerate(input_):
  step = signs[s]
  p = pos[i % 2]
  p = (step[0] + p[0], step[1] + p[1])
  pos[i % 2] = p
  visited[p] = visited.get(p, 0) + 1

print(f'Unique houses visited by both: {len(visited)}')
