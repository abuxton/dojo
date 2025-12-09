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

### Part Two

Red tiles are connected in order by straight, axis-aligned green lines (wrapping last to first). All tiles on those lines and all tiles inside the loop are green. Using two red tiles as opposite corners, find the largest rectangle that is fully contained in the red/green region (rectangle corners must be red; all tiles inside must be red or green).

**Example:** The largest rectangle using only red/green tiles has area **24** (between 9,5 and 2,3).

## Solution Outline

- Parse input as comma-separated `x,y` coordinates (ordered).
- Part 1:
  - Try all pairs of red tiles as opposite corners.
  - Area is inclusive: `(max_x - min_x + 1) × (max_y - min_y + 1)`.
  - Track maximum area (u64).
- Part 2:
  - Build the rectilinear polygon from the ordered red tiles (edges between consecutive points, wrapping).
  - A rectangle is valid if:
    - Opposite corners are red (given).
    - The other two corners are inside/on the polygon.
    - No polygon edge crosses the open interior of the rectangle (axis-aligned crossing check).
  - Evaluate all pairs of red tiles; keep the maximum inclusive area (u64).
- Geometry helpers: point-in-polygon (even-odd, boundary-inclusive) for axis-aligned polygons; interior-crossing checks for rectangle vs. polygon edges.

Time complexity: O(n² · m) worst-case (n red tiles, m edges≈n). Suitable for typical AoC input sizes.

## Visualization (optional)

A visualization helper can render red tiles (`#`) and an optional rectangle overlay (`O`). In Part 1, you can enable progress visualization:

- Use `visualize=true` (stderr) to print each new maximum (only if the grid is ≤ 100×100).
- Output for the example largest rectangle (area 50):
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
# Default: part 1, input.txt, no visualization
cargo run --release

# Part selection: part=1 or part=2
cargo run --release -- part=2

# Custom input file
cargo run --release -- input=path/to/input.txt

# Enable visualization (part 1 only, prints to stderr)
cargo run --release -- visualize=true
cargo run --release -- part=1 visualize=true
```

## Testing

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --release
```

## Notes

- Coordinates can be large; areas are computed as `u64`.
- Inclusive area: rectangle from (0,0) to (1,1) has area 4.
- Visualization auto-skips grids larger than 100×100.

## License

MIT (see LICENSE).
