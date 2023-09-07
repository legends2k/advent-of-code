#!/usr/bin/env python3

import sys
from array import array


class Board:

  def __init__(self):
    # 16 * u64 = 1024 bits
    self._data = array('Q', bytes(8 * 16 * 1000))

  def count(self):
    return sum((i.bit_count() for i in self._data))

  def on(self, x, y):
    i, b = divmod(x, 64)
    idx = y * 16 + i
    v = self._data[idx]
    self._data[idx] = v | (1 << b)

  def off(self, x, y):
    i, b = divmod(x, 64)
    idx = y * 16 + i
    v = self._data[idx]
    self._data[idx] = v & (~(1 << b))

  def toggle(self, x, y):
    i, b = divmod(x, 64)
    idx = y * 16 + i
    v = self._data[idx]
    self._data[idx] = v ^ (1 << b)

  def operate(self, op, start, stop):
    f = self.on if op == 'on' else (self.off if op == 'off' else self.toggle)
    for col in range(start[1], stop[1]+1):
      for row in range(start[0], stop[0]+1):
        f(row, col)

b = Board()

for line in sys.stdin:
  if line.startswith('turn'):
    toks = line.split(' ')
    start = tuple(int(t) for t in toks[2].split(','))
    stop = tuple(int(t) for t in toks[4].split(','))
    b.operate(toks[1], start, stop)
  elif line.startswith('toggle'):
    toks = line.split(' ')
    start = tuple(int(t) for t in toks[1].split(','))
    stop = tuple(int(t) for t in toks[3].split(','))
    b.operate(toks[0], start, stop)

print(f'Light count: {b.count()}')
