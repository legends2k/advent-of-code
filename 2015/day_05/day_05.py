#!/usr/bin/env python3

import sys
import functools
from collections import defaultdict


# NOTE: bespoke implementation of itertools.pairwise
def sliding_window(iterable: bytes, window: int):
  n = len(iterable)
  if window >= n:
    yield iterable
  elif window > 0:
    yield from (iterable[i:i+window] for i in range(0, len(iterable)-window+1))

def is_nice(s: bytes):
  naughty = (b'ab', b'cd', b'pq', b'xy')
  if any(n in s for n in naughty):
    return False
  vowels = (ord(b'a'), ord(b'e'), ord(b'i'), ord(b'o'), ord(b'u'))
  vowel_count = functools.reduce(lambda count, i: count + (i in vowels), s, 0)
  if vowel_count < 3:
    return False
  for p in sliding_window(s, 2):
    if p[0] == p[1]:
      return True
  return False

def is_rule_2_1_complaint(s: bytes):
  d = defaultdict(list)
  for i, w in enumerate(sliding_window(s, 2)):
    occurances = d[w]
    # handle "aaaa" case; check for any prior non-overlapping occurances
    if any((j != (i-1) for j in occurances)):
      return True
    else:
      occurances.append(i)
  return False

def is_rule_2_2_complaint(s: bytes):
  for i in range(0, len(s)-2):
    if s[i] == s[i+2]:
      return True
  return False

def is_real_nice(s: bytes):
  return is_rule_2_1_complaint(s) and is_rule_2_2_complaint(s)

words = [line.rstrip('\n').encode() for line in sys.stdin]
print(f'Count of nice strings: {sum((is_nice(w) for w in words))}')
print(f'Count of really nice strings: {sum((is_real_nice(w) for w in words))}')
