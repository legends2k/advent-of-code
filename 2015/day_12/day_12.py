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

def obj_walk(obj: any):
  to_visit = [obj]
  while to_visit:
    o = to_visit.pop()
    if isinstance(o, list):
      to_visit.extend(o)
    elif isinstance(o, dict) and 'red' not in o and 'red' not in o.values():
      for k, v in o.items():
        to_visit.append(k)
        to_visit.append(v)
    elif isinstance(o, int):
        yield o

input_ = sys.stdin.readline().rstrip().encode()
c = Counter()
j = json.loads(input_, parse_int=lambda s: c.parse(s))
print(c.total)
print(sum(obj_walk(j)))
