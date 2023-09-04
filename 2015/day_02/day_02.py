#!/usr/bin/env python3

import sys

class Box:

  def __init__(self, l, w, h):
    l = int(l)
    w = int(w)
    h = int(h)
    self.areas = (l * w, w * h, h * l)
    self.volume = l * w * h
    s1, s2, *rest = sorted((l, w, h))
    self.binding_perimeter = 2 * (s1 + s2)

  def wrapper(self):
    return (2 * sum(self.areas)) + min(self.areas)

  def ribbon(self):
    return self.binding_perimeter + self.volume

boxes_ = [Box(*line.rstrip('\n').split('x', maxsplit=3)) for line in sys.stdin]

# Part 1
total_wrapper = sum((b.wrapper() for b in boxes_))
print(f'Total wrapper: {total_wrapper}')

# Part 2
total_ribbon = sum((b.ribbon() for b in boxes_))
print(f'Total ribbon: {total_ribbon}')
