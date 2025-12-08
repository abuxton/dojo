## Solution Overview

This repository implements both parts in Rust:

- **Part 1 (left-to-right):** Each problem is a horizontal block. Each row (except the last) is a full number. The last row is the operator (`+` or `*`). Problems are separated by a fully blank column. For each problem, apply the operator to all numbers and sum all problem results.
- **Part 2 (right-to-left, columns-as-numbers):** Each problem is anchored by its operator in the bottom row. The problem spans all contiguous non-blank columns adjacent to that operator (until a fully blank column). Within a problem, every column (excluding the operator row) forms a single number by reading digits top-to-bottom. Process columns right-to-left (the rightmost column is the first operand). Apply the operator to all column-numbers and sum all problem results.

## Parsing Rules

- Pad rows to equal width for column access.
- A **blank column** (all spaces) separates problems.
- **Part 1:** Rows → numbers; bottom row → operator.
- **Part 2:** Columns → numbers (top-to-bottom digits); bottom row char → operator; columns consumed right-to-left within a problem.

## Examples

Given the sample worksheet:

```
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
```

**Part 1:**
- 123 * 45 * 6 = 33,210
- 328 + 64 + 98 = 490
- 51 * 387 * 215 = 4,243,455
- 64 + 23 + 314 = 401
**Total:** 4,277,556

**Part 2 (columns as numbers, read right-to-left per problem):**
- Rightmost `+`: columns → 4, 431, 623 → 4 + 431 + 623 = 1,058
- Next `*`: columns → 175, 581, 32 → 175 * 581 * 32 = 3,253,600
- Next `+`: columns → 8, 248, 369 → 8 + 248 + 369 = 625
- Leftmost `*`: columns → 356, 24, 1 → 356 * 24 * 1 = 8,544
**Total:** 3,263,827

## Usage

```bash
# Default input.txt
cargo run --release

# Custom input file
cargo run --release -- path/to/input.txt
```

Output is two lines: Part 1 answer, then Part 2 answer.

## Testing

```bash
cargo test --release -- --nocapture
```

Key tests:
- Example from README (Part 1 and Part 2 totals)
- Single-column add (expects `53`)
- Single-column multiply (expects `45`)
- Multi-problem, multi-number scenarios for Part 1 and Part 2
