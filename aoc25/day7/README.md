# Advent of Code 2025 Day 7: Laboratories

Welcome to Advent of Code 2025 Day 7 <https://adventofcode.com/2025/day/7>, maintained by abuxton.

## Description

### Part One
(Problem statement retained from puzzle.)

A tachyon beam enters at `S` and moves downward through empty space (`.`). When it hits a splitter (`^`), the beam stops and two new beams start from the immediate left and right of the splitter, moving downward. Count how many times beams are split.

### Part Two
A single quantum particle follows all possible paths. At each splitter, timelines fork: one goes left, one goes right (both continue downward). When a path leaves the grid, that timeline terminates. Count the total number of timelines produced.

## Usage

```bash
# From day7 directory, using input.txt
cargo run --release

# With a different input file
cargo run --release -- path/to/input.txt

# Run tests
cargo test --release
```

The binary prints two lines:
1. Part One answer (number of splits)
2. Part Two answer (number of timelines)

## Implementation Notes

- Part 1: BFS of beams; splitters deduped so each splitter contributes at most one split count.
- Part 2: Work-queue DP with `u128` counts; timelines fork left/right at splitters, move down otherwise; paths exiting the grid add to the total.

## License

MIT (see LICENSE).
