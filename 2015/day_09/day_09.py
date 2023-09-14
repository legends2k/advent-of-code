#!/usr/bin/env python3

import sys
import itertools

locations = {}
costs = {}

for line in sys.stdin:
  src, _, dst, _, cost = line[:-1].split(' ', maxsplit=5)
  s = locations.setdefault(src, len(locations))
  d = locations.setdefault(dst, len(locations))
  costs[(s, d)] = int(cost)
  costs[(d, s)] = int(cost)

all_routes = {route: sum(costs[leg] for leg in itertools.pairwise(route))
              for route in itertools.permutations(range(len(locations)))}

# Part 1
min_route = min(all_routes.items(), key=lambda p: p[1])
print(f'Shortest route distance: {min_route[1]}')

# Part 2
min_route = max(all_routes.items(), key=lambda p: p[1])
print(f'Longest route distance: {min_route[1]}')


# Hand-written, recursive implementation of non-desctrutive permutation;
# not part of the puzzle
#
# f([a, b, c, d]) -> a + f([b, c ,d])
#                         = (a, b, c, d), (a, b, d, c) .. (a, d, c, b)
#                     -> b + f([c, d]) = (b, c, d), (b, d, c)
#                     -> c + f([b, d]) = (c, b, d), (c, d, b)
#                     -> d + f([b, c]) = (d, b, c), (d, c, b)
#                 -> b + f([a, c, d])
#                         = (b, a, c, d), (b, a, d, c) .. (b, d, c, a)
#                     -> a + f([c, d]) = (a, c, d), (a, d, c)
#                     -> c + f([a, d]) = (c, a, d), (c, d, a)
#                     -> d + f([a, c]) = (d, a, c), (d, c, a)
#                 -> c + f([a, b, d])
#                         = (c, a, b, d), (c, a, d, b) .. (c, d, b, a)
#                     -> a + f([b, d]) = (a, b, d), (a, d, b)
#                     -> b + f([a, d]) = (b, a, d), (b, d, a)
#                     -> d + f([a, b]) = (d, a, b), (d, b, a)
#                 -> d + f([a, b, c])
#                         = (d, a, b, c), (d, a, c, b) .. (d, c, b, a)
#                     -> a + f([b, c]) = (a, b, c), (a, c, b)
#                     -> b + f([a, c]) = (b, a, c), (b, c, a)
#                     -> c + f([a, b]) = (c, a, b), (c, b, a)
def permute(iterable):
  if len(iterable) == 1:
    return [iterable]
  results = []
  for i in iterable:
    # Use a list for len(iterable) to work; no len(genexps)
    list_sans_i = [j for j in iterable if j != i]
    # list.append shouldn’t be used as it’ll append the genexp
    results.extend((i, *sublist) for sublist in permute(list_sans_i))
  return results
