# AOC25 Day 9: Movie Theatre

Welcome to Advent of Code 2025 Day 9 <https://adventofcode.com/2025/day/9>, maintained by abuxton.

## Description

### Part One

You arrive at the North Pole base movie theatre, which has a large tiled floor. The Elves are redecorating and need to find the largest rectangle using red tiles as two opposite corners.

Given a list of red tile coordinates (x, y), find the maximum area of any axis-aligned rectangle that can be formed by choosing any two red tiles as opposite corners.

**Example:**
Red tiles at: 7,1 / 11,1 / 11,7 / 9,7 / 9,5 / 2,5 / 2,3 / 7,3

The largest rectangle has area **50** (corners at 2,5 and 11,1).

**Question:** What is the largest area of any rectangle you can make?

## Solution

- Parse input as comma-separated `x,y` coordinates into a `HashSet<Point>`.
- Try all pairs of points as opposite corners.
- For each pair, compute axis-aligned rectangle bounds: `(min_x, min_y, max_x, max_y)`.
- Calculate area: `(max_x - min_x) * (max_y - min_y)`.
- Return the maximum area found.

Time complexity: O(n²) where n = number of red tiles.

## Usage

```bash
# Default: input.txt
cargo run --release

# Custom input file
cargo run --release -- input=path/to/input.txt

# Run tests
cargo test --release
```

The binary prints the Part 1 answer (largest rectangle area).

## License

MIT (see LICENSE).
