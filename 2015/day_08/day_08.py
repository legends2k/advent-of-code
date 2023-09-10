#!/usr/bin/env python3

import sys
import re
import functools

# Escape backslash though in raw strings as they’ve special meaning in regex
# e.g. `\s` matches whitespace chars: `\\s` matches a backslash followed by s.
pattern = re.compile(rb'(\\[\\"])|(\\x[0-9a-fA-F]{2})', flags=re.ASCII)

def mem_len(s: bytes):
  # subtract two for enclosing quotes
  return len(re.sub(pattern, b'-', s)) - 2

def enc_len(s: bytes):
  # add two for new additional quotes
  return 2+sum(1 if c not in (rb'\"') else 2 for c in s)

code, mem, enc = functools.reduce(
  lambda total, i: (total[0] + len(i),
                    total[1] + mem_len(i),
                    total[2] + enc_len(i)),
  (line.rstrip('\n').encode() for line in sys.stdin),
  (0, 0, 0))
print(f'{code} - {mem} = {code-mem}')
print(f'{enc} - {code} = {enc-code}')
