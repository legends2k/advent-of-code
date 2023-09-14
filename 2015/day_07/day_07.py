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

  def reset(self):
    self.value = 0


class BinaryGate:

  def __init__(self, output, op):
    self.inputs = [None, None]
    self.output = output
    self.op = op
    self.fixed_input = None
    self.value = None

  def set(self, value, pin = None):
    pin = pin if pin is not None else 0 if self.inputs[0] is None else 1
    self.inputs[pin] = value
    if all(i is not None for i in self.inputs):
      self.value = self.op(*self.inputs)
      self.output.set(self.value)

  def reset(self):
    self.value = None
    self.inputs = [None, None]
    if self.fixed_input:
      self.set(*self.fixed_input)

  def set_fixed_input(self, value, pin):
    self.fixed_input = (value, pin)
    self.set(*self.fixed_input)

circuit = {}
battery = {}

# Return result of conversion as duck typing is idiomatic over isinstance.
# Duck typing (EAFP) > isinstance() > type()
# https://stackoverflow.com/a/1549854/183120
def getWire(name):
  try:
    i = int(name)
    return (i, False)
  except ValueError:
    return (circuit.setdefault(name, Wire()), True)

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
    (input1, isInput1Wire) = getWire(toks[0])
    (input2, isInput2Wire) = getWire(toks[2])
    output = circuit.setdefault(toks[4], Wire())
    gate = circuit[' '.join(toks[0:3])] = \
      BinaryGate(output, ops[toks[1]])
    # if input is a wire make gate its output else make it gate’s fixed input
    if isInput1Wire:
      input1.connectOutput(gate)
    else:
      gate.set_fixed_input(input1, 0)
    if isInput2Wire:
      input2.connectOutput(gate)
    else:
      gate.set_fixed_input(input2, 1)

for (wire, v) in battery.items():
  circuit[wire].set(v)

# Debug code; try with input/sample
# for (name, component) in circuit.items():
#   if not any(i in name for i in ('AND', 'OR', 'RSHIFT', 'LSHIFT', 'NOT')):
#     print(f'{name}: {component.value}')

# Part 1
signal_a = circuit['a'].value
print(signal_a)

# Part 2
battery['b'] = signal_a

for components in circuit.values():
  components.reset()

# circuit['b'].set(signal_a) alone won’t work
for (wire, v) in battery.items():
  circuit[wire].set(v)

print(circuit['a'].value)
