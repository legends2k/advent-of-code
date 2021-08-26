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

# Part 2: Looping Code

Running elfcode again with register 0 set to 1 seems to make it loop forever.  Printing the registers show

```
before                           --> instr id --> after

[0, 0, 0, 10550400, 1, 10551345] --> instr 19 --> [0, 1, 0, 10550400, 2, 10551345]
[0, 1, 0, 10550400, 2, 10551345] --> instr 20 --> [0, 1, 1, 10550400, 3, 10551345]
[0, 1, 1, 10550400, 3, 10551345] --> instr 21 --> [0, 1, 1, 1, 4, 10551345]
[0, 1, 1, 1, 4, 10551345] --> instr 22 --> [0, 1, 1, 0, 5, 10551345]
[0, 1, 1, 0, 5, 10551345] --> instr 23 --> [0, 1, 1, 0, 6, 10551345]
[0, 1, 1, 0, 6, 10551345] --> instr 24 --> [0, 1, 1, 0, 8, 10551345]
[0, 1, 1, 0, 8, 10551345] --> instr 25 --> [0, 1, 2, 0, 9, 10551345]
[0, 1, 2, 0, 9, 10551345] --> instr 26 --> [0, 1, 2, 0, 10, 10551345]
[0, 1, 2, 0, 10, 10551345] --> instr 27 --> [0, 1, 2, 0, 11, 10551345]
[0, 1, 2, 0, 11, 10551345] --> instr 28 --> [0, 1, 2, 0, 3, 10551345]
[0, 1, 2, 0, 3, 10551345] --> instr 29 --> [0, 1, 2, 2, 4, 10551345]
[0, 1, 2, 2, 4, 10551345] --> instr 30 --> [0, 1, 2, 0, 5, 10551345]
[0, 1, 2, 0, 5, 10551345] --> instr 31 --> [0, 1, 2, 0, 6, 10551345]
[0, 1, 2, 0, 6, 10551345] --> instr 32 --> [0, 1, 2, 0, 8, 10551345]
[0, 1, 2, 0, 8, 10551345] --> instr 33 --> [0, 1, 3, 0, 9, 10551345]
[0, 1, 3, 0, 9, 10551345] --> instr 34 --> [0, 1, 3, 0, 10, 10551345]
[0, 1, 3, 0, 10, 10551345] --> instr 35 --> [0, 1, 3, 0, 11, 10551345]
[0, 1, 3, 0, 11, 10551345] --> instr 36 --> [0, 1, 3, 0, 3, 10551345]
[0, 1, 3, 0, 3, 10551345] --> instr 37 --> [0, 1, 3, 3, 4, 10551345]
[0, 1, 3, 3, 4, 10551345] --> instr 38 --> [0, 1, 3, 0, 5, 10551345]
[0, 1, 3, 0, 5, 10551345] --> instr 39 --> [0, 1, 3, 0, 6, 10551345]
[0, 1, 3, 0, 6, 10551345] --> instr 40 --> [0, 1, 3, 0, 8, 10551345]
[0, 1, 3, 0, 8, 10551345] --> instr 41 --> [0, 1, 4, 0, 9, 10551345]
[0, 1, 4, 0, 9, 10551345] --> instr 42 --> [0, 1, 4, 0, 10, 10551345]
[0, 1, 4, 0, 10, 10551345] --> instr 43 --> [0, 1, 4, 0, 11, 10551345]
[0, 1, 4, 0, 11, 10551345] --> instr 44 --> [0, 1, 4, 0, 3, 10551345]
```

We see that instruction pointer loops with [3, 4, 5, 6, 8, 9, 10, 11] instructions.

With following register values

| Register |    Value |
|----------|---------:|
| A        |        0 |
| B        |        1 |
| C        |        1 |
| D        | 10550400 |
| E        | 10551345 |
| I        |        3 |

below snippet loops forever!

``` elfcode
 1: seti 1 7 1
 2: seti 1 8 2

 3: mulr 1 2 3
 4: eqrr 3 5 3
 5: addr 3 4 4
 6: addi 4 1 4
 7: addr 1 0 0
 8: addi 2 1 2
 9: gtrr 2 5 3
10: addr 4 3 4
11: seti 2 1 4

12: addi 1 1 1
13: gtrr 1 5 3
14: addr 3 4 4
15: seti 1 8 4
```

Pseudocode (remember `I` increments silently after every instruction)

```
B = 1
C = 1
D = B * C
D = (D == E) ? 1 : 0
I += D
I += 1
A += B
C += 1
D = (C > E) ? 1 : 0
I += D
I = 2
B += 1
D = (B > E) ? 1 : 0
I += D
I = 1
```

In Rust

``` rust
B = 1
while B <= E {
  C = 1
  while C <= E {
    D = B * C
    if D == E {
      A += B
    }
    C += 1
  }
  B += 1
}
```

In C

``` c
int a = 0;
for (int b = 1; b <= e; ++b) {
  for (int c = 1; c <= e; ++e) {
    if ((b * c) == e)
      a += b;
  }
}
```

This is a very unoptimised implementation of finding all factors of `E` and summing it to `A`.
