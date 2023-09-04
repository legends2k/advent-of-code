#!/usr/bin/env python3

import sys
import hashlib

input_ = b''.join(line.rstrip('\n').encode() for line in sys.stdin)

i = 1
found = False
while True:
  data = input_ + str(i).encode()
  if hashlib.md5(data).hexdigest().startswith('00000') and not found:
    print(i)
    found = True
  if hashlib.md5(data).hexdigest().startswith('000000'):
    break
  i += 1

print(i)
