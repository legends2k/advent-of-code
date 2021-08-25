# Question Digest

- Rounds
  * Turns per player
    + Order: top-left to bottom-right (reading order) at round start, despite unit movement in-between
    + Alive units get to take a turn
    + A turn consists of move and attack
    + No diagonal move/attack
    + Identify: unit looks for its enemies/targets; no targets, end combat
    + Move: adjacent cell of a target; if none end turn
      - Choose adj. cell of least steps; tie break in reading order

# Solution Steps

* Part I: path finding
  - [x] Given empty map, src cell and bunch of dst cells
    + Board can be an array of enums: wall, vacant, goblin(id), elf(id)
    + List of players
      * Sort top-left to bottom-right
      * Deduce in-range cells that are adj to target
      * Do BFS and keep broadening range; stop when an in-range cell is found
  - [x] Find in-range points; vacant, adjacent cells of targets
* Part II: attack
