# Question Digest

* Total Registers: [0, 5]
* Macro to bound a IP to a Register: `#ip N`
* All manipulations to register `N` means on IP
* When IP bound
  1. Before executing each instruction copy IP -> `N`
  2. After each instruction copy `N` to IP
  3. Move to next instruction: add 1 to IP
* IP (and registers) start with value `0`
* If IP causes out of bounds memory read halt program; reset IP to `0`

# Mnemonic to Opcode

Extracted from Day 16 solution:

```
borr: 0
addr: 1
eqrr: 2
addi: 3
eqri: 4
eqir: 5
gtri: 6
mulr: 7
setr: 8
gtir: 9
muli: 10
banr: 11
seti: 12
gtrr: 13
bani: 14
bori: 15
```
