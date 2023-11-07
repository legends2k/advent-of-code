#!/usr/bin/env python3

import sys

confusing = (105, 108, 111)  # i, l, o
alpha = b'abcdefghijklmnopqrstuvwxyz'
digits = b'0123456789abcdefghijklmnop'
lut = bytes.maketrans(alpha, digits)

def is_valid(p: bytes) -> bool:
  straight = False
  doubles = list()
  if p[0] in confusing:
    return False
  for i in range(1, len(p)):
    cur = p[i]
    if cur in confusing:
      return False
    prev = p[i-1]
    if not straight:
      if (i + 1) == len(p):
        return False
      else:
        next_ = p[i+1]
        if ((prev + 1) == cur) and (cur == (next_ - 1)):
          straight = True
    if len(doubles) < 2 and cur == prev and cur != next and cur not in doubles:
      doubles.append(cur)
  return straight and (len(doubles) >= 2)

def int_to_pwd(i: int) -> str:
  base = 26
  l = bytearray()
  while i >= base:
    i, r = divmod(i, base)
    l.append(alpha[0] + r)
  l.append(alpha[0] + i)
  l.reverse()
  return bytes(l)

def inc(p: bytes) -> bytes:
  """Returns |p| incremented skipping confusing letters."""
  for i, c in enumerate(p):
    if c in confusing:
      b = bytearray(p[:i])
      b.append(c + 1)
      b.extend(b'a' * (7 - i))
      return bytes(b)
  n = int(p.translate(lut), base=26) + 1
  return int_to_pwd(n)

def next_pwd(p: bytes) -> bytes:
  while not is_valid(p):
    p = inc(p)
  return p

input_ = sys.stdin.readline().rstrip().encode('utf8')
p1 = next_pwd(input_)
print(p1)
p2 = next_pwd(inc(p1))
print(p2)
