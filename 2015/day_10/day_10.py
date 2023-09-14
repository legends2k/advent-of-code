#!/usr/bin/env python3

import sys
import itertools


def look_say(s):
  t = ''
  for (number, l) in itertools.groupby(s):
    t = f'{t}{len(tuple(l))}{number}'
  return t

s = sys.stdin.readline().rstrip()
for _ in range(40):
  s = look_say(s)
print(len(s))
