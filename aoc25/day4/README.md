# Advent of Code 2025 Day 4 Printing Department

Welcome to Advent of Code 2025, maintained by abuxton.

## Description

Advent of code 2025 Day 4, implemented in Rust via vibe coding and testing.

### Part One

You ride the escalator down to the printing department. They're clearly getting ready for Christmas; they have lots of large rolls of paper everywhere, and there's even a massive printer in the corner (to handle the really big print jobs).

Decorating here will be easy: they can make their own decorations. What you really need is a way to get further into the North Pole base while the elevators are offline.

"Actually, maybe we can help with that," one of the Elves replies when you ask for help. "We're pretty sure there's a cafeteria on the other side of the back wall. If we could break through the wall, you'd be able to keep moving. It's too bad all of our forklifts are so busy moving those big rolls of paper around."

If you can optimize the work the forklifts are doing, maybe they would have time to spare to break through the wall.

The rolls of paper (`@`) are arranged on a large grid; the Elves even have a helpful diagram (your puzzle input) indicating where everything is located.

For example:

```
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
```

The forklifts can only access a roll of paper if there are **fewer than four rolls of paper** in the eight adjacent positions (including diagonals). If you can figure out which rolls of paper the forklifts can access, they'll spend less time looking and more time breaking down the wall to the cafeteria.

In this example, there are **13** rolls of paper that can be accessed by a forklift (marked with `x`):

```
..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.
```

**Question:** Consider your complete diagram of the paper roll locations. How many rolls of paper can be accessed by a forklift?

### Part Two

"Excellent, that should give them plenty of time to work on that wall!" one of the Elves exclaims. "Let's get started."

The forklifts start moving rolls of paper. As they work, more rolls become accessible. Each time the forklifts remove all currently accessible rolls, new rolls may become accessible (since they now have fewer neighbors).

For the example above, the process looks like this:

**Initial state (13 accessible rolls marked with `x`):**
```
..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.
```

**After removing those 13 rolls (12 more accessible):**
```
.....xx@..
.@@.@.@.@@
@@@@@...@@
@.@@@@....
.@.@@@@.@.
.@@@@@@@.@
.@.@.@.@x@
..@@@.@@@@
.@@@@@@@@.
...x@@@...
```

This process continues until no more rolls can be removed. The forklifts keep working until they've removed all accessible rolls.

**Question:** How many total rolls will the forklifts remove before they can't remove any more?

For the example above, the answer is **43**.

## Getting Started

### Requirements

- Rust toolchain (rustup + cargo)
- Input file `input.txt` in the crate root (or pass a custom path as CLI argument)

### Build

```bash
cd /Users/abuxton/src/github/dojo/aoc25/day4
cargo build --release
```

## Usage

Run with default input file (`input.txt`):

```bash
cargo run --release
```

Run with a custom input file:

```bash
cargo run --release -- path/to/input.txt
```

**Expected output** for the example above:
```
13
43
```

The first line is the answer to Part One, the second line is the answer to Part Two.

### Input Format

- Grid of characters where:
  - `@` represents a roll of paper
  - `.` represents empty space
- Each line is a row of the grid
- All rows should have the same length (rectangular grid)

### Algorithm

**Part One:**
For each roll of paper (`@`):
1. Count how many of its 8 neighbors (orthogonal + diagonal) are also rolls (`@`)
2. If the count is **less than 4**, the roll is accessible
3. Sum the total number of accessible rolls

**Part Two:**
Iteratively remove accessible rolls:
1. Find all currently accessible rolls (< 4 `@` neighbors)
2. Remove all of them simultaneously (replace with `.`)
3. Repeat until no more rolls are accessible
4. Return the total count of removed rolls

## Tests

Unit tests are included in `src/main.rs`. Run:

```bash
cargo test
```

Tests cover:
- Part 1 example from the README (expected: 13)
- Part 2 example from the README (expected: 43)
- Edge cases (single roll, fully surrounded grid, iterative removal)

## Implementation

See `src/main.rs` for the Rust implementation. The solver includes:

- `solve()`: Part One - counts initially accessible rolls
- `solve_part2()`: Part Two - iteratively removes accessible rolls
- `count_neighbors()`: Helper to count `@` neighbors for a position
- `find_accessible_positions()`: Helper to find all accessible positions in current grid state

## Contributing

Contributions are always welcome! If you're interested in contributing, please review our [contributing guidelines](../../../CONTRIBUTING.md).

## License

This project is licensed under the MIT License - see the [LICENSE.md](../../../LICENSE.md) file for details.
