#!/usr/bin/env python3

import sys
import itertools
from collections.abc import Iterable

def rle(s: Iterable[any]) -> Iterable[int]:
  """Returns a generator object of run-length encoded |s|."""
  # chain.from_iterable() constructor of itertools.chain but is different from
  # the default chain() constructor; it flattens and chains.
  return itertools.chain.from_iterable(((len(tuple(ls)), int(c))
                                        for c, ls in itertools.groupby(s)))

def length(it: Iterable[int]) -> int:
  # This is similar to Rust’s slice::chuncks
  if sys.version_info.major == 3 and sys.version_info.minor <= 11:
    return sum(l for l in itertools.islice(it, None, None, 2))
  return sum((i[0] for i in itertools.batched(it, 2)))

s = sys.stdin.readline().rstrip()
input_ = rle(s)
for i in range(50):
  input_ = rle(input_)
  if i == 39:
    t = tuple(input_)
    print(f'Length after 40 iterations: {length(t)}')
    input_ = t

print(f'Length after 50 iterations: {length(input_)}')
