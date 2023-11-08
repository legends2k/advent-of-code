#!/usr/bin/env python3

import json
import sys

class Counter:

  def __init__(self):
    self.total = 0

  def parse(self, s: str) -> int:
    i = int(s)
    self.total += i
    return i

input_ = sys.stdin.readline().rstrip().encode()
c = Counter()
j = json.loads(input_, parse_int=lambda s: c.parse(s))
print(c.total)
