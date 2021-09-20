* Program runs infinite loops
* Analyze assembly code
* ~~Make it halt by integer underflow by only changing register~~ `0`
* Least non-negative `int` in `R0` to halt after fewest instruction executions
* `bani`, `banr`, `bori`, `borr` interpret their input as numbers
* Numeric bitwise op verification failing results in infinite testing


* Ignore instructions 0 to 5, the numeric binary ops check routine
  - Program essentially starts from L1 i.e. R3 = 6, R4 = 0
* 

R0 0
R1 0
R2 0
IP 6
R4 0
R5 0

L0:
    R5 = R4 | 0x10000
    R4 = 0x1C4E46
L3:
    R2 = R5 & 0xFF
    R4 = R4 + R2
    R4 = R4 & 0xFFFFFF
    R4 = R4 * 0x1016B
    R4 = R4 & 0xFFFFFF
    R2 = 256 > R5         ; jump to L2 if R5 < 256 else L1
    R3 = R2 + R3
    R3 = R3 + 1
    R3 = 27
L1:
    R2 = 0
L4:
    R1 = R2 + 1
    R1 = R1 * 0x100
    R1 = R1 > R5
    R3 = R1 + R3
    R3 = R3 + 1
    R3 = 25               ; jump to L5
    R2 = R2 + 1
    R3 = 17               ; jump to L4
L5:
    R5 = R2
    R3 = 7                ; jump to L3
L2:
    R2 = R4 == R0
    R3 = R2 + R3
    R3 = 5                ; jump to L0

``` c
loop {
  r5 = r4 | 0x10000;
  r4 = 0x1C4E46;
  loop {
    r2 = r5 & 0xFF;
    r4 += r2;
    r4 &= 0xFFFFFF;
    r4 *= 0x1016B;
    r4 &= 0xFFFFFF;
    if (r5 >= 256) {
      for (r2 = 0; r1 <= r5; r2++) {
        r1 = (r2 + 1) * 0x100;
      }
      r5 = r2;
    } else {
      if (r4 == r0)
        return;
      else
        break;
    }
  }
}
```

Part 1: whatever `r4` reaches first when checked against `r0`
  - Happens in second iteration of the inner `loop`
  - Iteration 1: r5 is set from `65536` to `256`
  - Iteration 2: r5 is set from `256` to `1`

Part 2
  - tried analysing assembly
  - looked for patterns by printing `r4` values
  - looks like most solved it by looking for a repeat in `r4` value

An optimisation would be to cut the innermost loop (`r1`, `r2`) and directly do `r5 /= 256`.
