#!/usr/bin/env python3

import sys
import functools
import array


input_ = b''.join((line.rstrip('\n').encode() for line in sys.stdin))
signs = array.array('b', [1, -1])

# Part 1
floor = functools.reduce(lambda sum, i: signs[i - ord(b'(')] + sum, input_, 0)
print(f'Final floor: {floor}')

# Part 2
floor = 0
for i, c in enumerate(input_, 1):
  floor += signs[c - ord(b'(')]
  if floor == -1:
    print(f'First input to reach cellar: {i}')
    break
