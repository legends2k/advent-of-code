#!/usr/bin/env python3

import sys

class Box:

  def __init__(self, l, w, h):
    l = int(l)
    w = int(w)
    h = int(h)
    self.areas = (l * w, w * h, h * l)

  def wrapper(self):
    return (2 * sum(self.areas)) + min(self.areas)


boxes_ = (Box(*line.rstrip('\n').split('x', maxsplit=3)) for line in sys.stdin)

# Part 1
total_wrapper = sum((b.wrapper() for b in boxes_))
print(total_wrapper)
