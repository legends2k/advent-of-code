#!/usr/bin/env python3

import sys


class Wire:

  def __init__(self, *outputs, op = None):
    self.value = 0
    self.outputs = [o for o in outputs]
    self.op = op or (lambda x: x)

  def connectOutput(self, o):
    self.outputs.append(o)

  def set(self, value):
    self.value = self.op(value)
    for c in self.outputs:
      c.set(self.value)


class BinaryGate:

  def __init__(self, op, output):
    self.inputs = [None, None]
    self.output = output
    self.value = None
    self.op = op

  def set(self, value):
    if self.inputs[0] is None:
      self.inputs[0] = value
    else:
      self.inputs[1] = value
    if all(i is not None for i in self.inputs):
      self.value = self.op(*self.inputs)
      self.output.set(self.value)


circuit = {}
battery = {}

def getWire(s):
  try:
    i = int(s)
    return i
  except ValueError:
    return circuit.setdefault(s, Wire())

ops = { 'AND': lambda x, y: x & y,
        'OR': lambda x, y: x | y,
        'LSHIFT': lambda x, y: x << y,
        'RSHIFT': lambda x, y: x >> y}

for line in sys.stdin:
  toks = line.rstrip('\n').split(' ')
  if toks[0] == 'NOT':
    output = circuit.setdefault(toks[3], Wire())
    gate = circuit[' '.join(t for t in toks if t != '->')] = \
      Wire(output, op=lambda x: ~x & 65535)
    input_ = circuit.setdefault(toks[1], Wire())
    input_.connectOutput(gate)
  elif toks[1] == '->':
    try:
      i = int(toks[0])
      battery[toks[2]] = i
    except ValueError:
      output = circuit.setdefault(toks[2], Wire())
      input_ = circuit.setdefault(toks[0], Wire())
      input_.connectOutput(output)
  elif toks[1] in ('AND', 'OR', 'RSHIFT', 'LSHIFT'):
    input1 = getWire(toks[0])
    input2 = getWire(toks[2])
    output = circuit.setdefault(toks[4], Wire())
    gate = circuit[' '.join(toks[0:3])] = \
      BinaryGate(ops[toks[1]], output)
    if isinstance(input1, Wire):
      input1.connectOutput(gate)
    else:
      gate.inputs[0] = input1
    if isinstance(input2, Wire):
      input2.connectOutput(gate)
    else:
      gate.inputs[1] = input2

for (wire, v) in battery.items():
  circuit[wire].set(v)

# Debug code; try with input/sample
# for (name, component) in circuit.items():
#   if not any(i in name for i in ('AND', 'OR', 'RSHIFT', 'LSHIFT', 'NOT')):
#     print(f'{name}: {component.value}')

# Part 1
print(circuit['a'].value)
