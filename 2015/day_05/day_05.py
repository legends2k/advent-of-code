#!/usr/bin/env python3

import sys
import functools
import itertools


def is_nice(s: bytes):
  naughty = (b'ab', b'cd', b'pq', b'xy')
  if any(n in s for n in naughty):
    return False
  vowels = (ord(b'a'), ord(b'e'), ord(b'i'), ord(b'o'), ord(b'u'))
  vowel_count = functools.reduce(lambda count, i: count + (i in vowels), s, 0)
  if vowel_count < 3:
    return False
  for p in itertools.pairwise(s):
    if p[0] == p[1]:
      return True
  return False

words = [line.rstrip('\n').encode() for line in sys.stdin]
print(len(words))
print(f'Count of nice strings: {sum((is_nice(w) for w in words))}')
