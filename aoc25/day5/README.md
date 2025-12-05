# Advent of Code 2025 Day 5: Cafeteria

Welcome to Advent of Code 2025 Day 5, maintained by abuxton.

## Description

Advent of Code 2025 Day 5, implemented in Rust via vibe coding and testing.

### Part One

As the forklifts break through the wall, the Elves are delighted to discover that there was a cafeteria on the other side after all.

You can hear a commotion coming from the kitchen. "At this rate, we won't have any time left to put the wreaths up in the dining hall!" Resolute in your quest, you investigate.

"If only we hadn't switched to the new inventory management system right before Christmas!" another Elf exclaims. You ask what's going on.

The Elves in the kitchen explain the situation: because of their complicated new inventory management system, they can't figure out which of their ingredients are fresh and which are spoiled. When you ask how it works, they give you a copy of their database (your puzzle input).

The database operates on ingredient IDs. It consists of a list of fresh ingredient ID ranges, a blank line, and a list of available ingredient IDs. For example:

```
3-5
10-14
16-20
12-18

1
5
8
11
17
32
```

The fresh ID ranges are inclusive: the range `3-5` means that ingredient IDs 3, 4, and 5 are all fresh. The ranges can also overlap; an ingredient ID is fresh if it is in any range.

The Elves are trying to determine which of the available ingredient IDs are fresh. In this example, this is done as follows:

- Ingredient ID 1 is **spoiled** because it does not fall into any range.
- Ingredient ID 5 is **fresh** because it falls into range 3-5.
- Ingredient ID 8 is **spoiled**.
- Ingredient ID 11 is **fresh** because it falls into range 10-14.
- Ingredient ID 17 is **fresh** because it falls into range 16-20 as well as range 12-18.
- Ingredient ID 32 is **spoiled**.

So, in this example, **3** of the available ingredient IDs are fresh.

**Question:** Process the database file from the new inventory management system. How many of the available ingredient IDs are fresh?

### Part Two

"Thank you so much!" the Elves exclaim. "Now we know which ingredients we can use. But we also need to know which ones to throw away so we don't accidentally use spoiled ingredients."

**Question:** How many of the available ingredient IDs are **spoiled** (not in any fresh range)?

For the example above, **3** ingredient IDs are spoiled (1, 8, and 32).

## Getting Started

### Requirements

- Rust toolchain (rustup + cargo)
- Input file `input.txt` in the crate root (or pass a custom path as CLI argument)

### Build

```bash
cd /Users/abuxton/src/github/dojo/aoc25/day5
cargo build --release
```

## Usage

Run with default input file (`input.txt`):

```bash
cargo run --release
```

Run with a custom input file:

```bash
cargo run --release -- my_input.txt
```

**Expected output** for the example above:
```
3
3
```

The first line is Part 1 (fresh ingredient IDs), the second line is Part 2 (spoiled ingredient IDs).

### Input Format

The input consists of two sections separated by a blank line:

1. **Fresh ingredient ID ranges** (one per line, format: `a-b` where a ≤ b, inclusive)
   - Supports large numbers up to u64::MAX (18,446,744,073,709,551,615)
   - Ranges can overlap
   - Empty lines are ignored
2. **Available ingredient IDs to check** (one per line, positive integers)
   - Also supports u64 range
   - Empty lines are ignored

Example:
```
3-5
10-14
16-20
12-18

1
5
8
11
17
32
```

### Algorithm

**Part 1 (Fresh ingredients):**
- For each ingredient ID, check if it falls within ANY of the fresh ranges (inclusive)
- Count IDs that ARE in at least one range
- Time complexity: O(n × m) where n = queries, m = ranges



### Error Handling

The implementation provides detailed error messages for:
- Invalid range format (not `a-b`)
- Numbers too large for u64 (> 18,446,744,073,709,551,615)
- Reversed ranges (where `a > b`)
- Missing or empty query section
- File read errors

Example error output:
```
Error solving puzzle: Invalid number '263168346238540123456789' in range on line 1: number too large to fit in target type
```

## Tests

Unit tests are included in `src/main.rs`. Run:

```bash
cargo test
```

Or for release builds:

```bash
cargo test --release
```

Tests cover:
- Example from README (3 fresh, 3 spoiled)
- All IDs fresh
- All IDs spoiled
- Overlapping ranges
- Invalid range formats
- Reversed ranges
- Large numbers (u64 range)
- Empty query sections

## Implementation

See `src/main.rs` for the Rust implementation. The solver:
- Parses fresh ingredient ranges from first section (with validation)
- Parses available ingredient IDs from second section (with validation)
- Counts IDs in any range (Part 1: fresh)
- Counts IDs not in any range (Part 2: spoiled)
- Uses u64 for all numbers to support very large ingredient IDs
- Provides detailed error messages with line numbers

## Performance Notes

For large inputs with many ranges and queries, consider:
- Merging overlapping ranges to reduce comparisons
- Using interval tree or binary search for O(log m) lookups
- Current implementation is simple and sufficient for typical AoC inputs

## Contributing

Contributions are always welcome! If you're interested in contributing, please review our [contributing guidelines](../../../CONTRIBUTING.md).

## License

This project is licensed under the MIT License - see the [LICENSE.md](../../../LICENSE.md) file for details.
