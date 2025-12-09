# AOC25 Day 9: Movie Theatre

Welcome to Advent of Code 2025 Day 9 <https://adventofcode.com/2025/day/9>, maintained by abuxton.

## Description

### Part One

You arrive at the North Pole base movie theatre, which has a large tiled floor. The Elves are redecorating and need to find the largest rectangle using red tiles as two opposite corners.

Given a list of red tile coordinates (x, y), find the maximum area of any axis-aligned rectangle that can be formed by choosing any two red tiles as opposite corners.

**Example:**
Red tiles at: 7,1 / 11,1 / 11,7 / 9,7 / 9,5 / 2,5 / 2,3 / 7,3

The largest rectangle has area **50** (corners at 2,1 and 11,5).

**Question:** What is the largest area of any rectangle you can make?

## Solution

- Parse input as comma-separated `x,y` coordinates into a `HashSet<Point>`.
- Try all pairs of points as opposite corners (O(n²) combinations).
- For each pair, compute axis-aligned rectangle bounds: `(min_x, min_y, max_x, max_y)`.
- Calculate area inclusively: `(max_x - min_x + 1) × (max_y - min_y + 1)`.
- Return the maximum area found as `u64` to handle large coordinate values.

Time complexity: O(n²) where n = number of red tiles.

### Visualization

The solution includes optional visualization that shows each new maximum rectangle as it's discovered:

- Displays corner coordinates and dimensions
- Renders the grid (for grids ≤ 100×100)
- Red tiles marked as `#`, rectangle overlay marked as `O`

Example output for the largest rectangle (area 50):
```
..............
..OOOOOOOOOO..
..OOOOOOOOOO..
..OOOOOOOOOO..
..OOOOOOOOOO..
..OOOOOOOOOO..
..............
.........#.#..
..............
```

## Usage

```bash
# Default: input.txt, no visualization
cargo run --release

# Custom input file
cargo run --release -- input=path/to/input.txt

# Enable visualization (shows progress to stderr)
cargo run --release -- visualize=true
cargo run --release -- --visualize

# Run tests (includes visualization tests)
cargo test --release

# Run tests with output
cargo test --release -- --nocapture
```

The binary prints the Part 1 answer (largest rectangle area as a 64-bit unsigned integer) to stdout. Visualization output goes to stderr.

## Notes

- Coordinates can be very large (tested up to ~98,000).
- Area calculations use `u64` to prevent overflow for large rectangles.
- The inclusive boundary calculation means a rectangle from (0,0) to (1,1) has area 4, not 1.
- Visualization automatically skips grids larger than 100×100 to avoid excessive output.

## License

MIT (see LICENSE).
