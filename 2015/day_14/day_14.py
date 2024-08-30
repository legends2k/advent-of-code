#!/usr/bin/env python3

import sys

class Raindeer:

  def __init__(self, name, speed, stamina, rest):
    self.name = name
    self.speed = int(speed)
    self.stamina = int(stamina)
    self.rest = int(rest)

  def __repr__(self):
    return self.name

  def __str__(self):
    return (f'{self.name}: '
            f'{self.speed} km/s for {self.stamina}s; rest {self.rest}s')

raindeers = []
for line in sys.stdin:
  name, _, _, speed, _, _, stamina, *_, rest, _ = line[:-1].split(' ')
  raindeers.append(Raindeer(name, speed, stamina, rest))

# NOTE: puzzle input
stop = 2503

# Part 1: optimize simulation
positions = []
for r in raindeers:
  unit = r.stamina + r.rest
  batches = stop // unit
  batches += 1 if ((batches * unit) + r.stamina) <= stop else 0
  positions.append(batches * r.speed * r.stamina)
print(max(positions))

# Part 2: simulate
positions = [0] * len(raindeers)
points = [0] * len(raindeers)
for t in range(stop):
  max_pos = 0
  leaders = []
  for i, r in enumerate(raindeers):
    unit = r.stamina + r.rest
    batches = t // unit
    is_flying = (t - (batches * unit)) < r.stamina
    positions[i] += r.speed if is_flying else 0
    if positions[i] > max_pos:
      max_pos = positions[i]
      leaders = [i]
    elif positions[i] == max_pos:
      leaders.append(i)
  for l in leaders:
    points[l] += 1
print(max(points))
