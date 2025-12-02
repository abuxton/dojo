# aoc25 Day 2 — Gift Shop

Advent of Code 2025 — Day 2: Gift Shop.

Brief: given a single line of comma-separated ID ranges (e.g. `11-22,95-115,...`), find all IDs that are formed by repeating a sequence of digits. Sum all such invalid IDs present in the ranges.

## Input format
- A single line containing comma-separated ranges.
- Each range is `start-end` where start and end are inclusive non-negative integers with no leading zeros.
- Example input file (`input.txt`):
```
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
```

## Problem parts

- Part 1: An ID is invalid if its decimal representation is exactly two repetitions of the same digit sequence (e.g. `55`, `6464`, `123123`). Count and sum all such IDs in the given ranges.
- Part 2: Extended rule — an ID is invalid if it is formed by repeating some digit sequence at least twice (e.g. `123123`, `1212121212`, `1111111`). Count and sum all such IDs in the given ranges.

### --- Day 2: Gift Shop ---

#### Part One

You get inside and take the elevator to its only other stop: the gift shop. "Thank you for visiting the North Pole!" gleefully exclaims a nearby sign. You aren't sure who is even allowed to visit the North Pole, but you know you can access the lobby through here, and from there you can access the rest of the North Pole base.

As you make your way through the surprisingly extensive selection, one of the clerks recognizes you and asks for your help.

As it turns out, one of the younger Elves was playing on a gift shop computer and managed to add a whole bunch of invalid product IDs to their gift shop database! Surely, it would be no trouble for you to identify the invalid product IDs for them, right?

They've even checked most of the product ID ranges already; they only have a few product ID ranges (your puzzle input) that you'll need to check. For example:

11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
(The ID ranges are wrapped here for legibility; in your input, they appear on a single long line.)

The ranges are separated by commas (,); each range gives its first ID and last ID separated by a dash (-).

Since the young Elf was just doing silly patterns, you can find the invalid IDs by looking for any ID which is made only of some sequence of digits repeated twice. So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.

None of the numbers have leading zeroes; 0101 isn't an ID at all. (101 is a valid ID that you would ignore.)

Your job is to find all of the invalid IDs that appear in the given ranges. In the above example:

11-22 has two invalid IDs, 11 and 22.
95-115 has one invalid ID, 99.
998-1012 has one invalid ID, 1010.
1188511880-1188511890 has one invalid ID, 1188511885.
222220-222224 has one invalid ID, 222222.
1698522-1698528 contains no invalid IDs.
446443-446449 has one invalid ID, 446446.
38593856-38593862 has one invalid ID, 38593859.
The rest of the ranges contain no invalid IDs.
Adding up all the invalid IDs in this example produces 1227775554.

What do you get if you add up all of the invalid IDs?

#### Part Two

The clerk quickly discovers that there are still invalid IDs in the ranges in your list. Maybe the young Elf was doing other silly patterns as well?

Now, an ID is invalid if it is made only of some sequence of digits repeated at least twice. So, 12341234 (1234 two times), 123123123 (123 three times), 1212121212 (12 five times), and 1111111 (1 seven times) are all invalid IDs.

From the same example as before:

11-22 still has two invalid IDs, 11 and 22.
95-115 now has two invalid IDs, 99 and 111.
998-1012 now has two invalid IDs, 999 and 1010.
1188511880-1188511890 still has one invalid ID, 1188511885.
222220-222224 still has one invalid ID, 222222.
1698522-1698528 still contains no invalid IDs.
446443-446449 still has one invalid ID, 446446.
38593856-38593862 still has one invalid ID, 38593859.
565653-565659 now has one invalid ID, 565656.
824824821-824824827 now has one invalid ID, 824824824.
2121212118-2121212124 now has one invalid ID, 2121212121.

Adding up all the invalid IDs in this example produces 4174379265.

## Usage

Build and run from the crate directory:

```bash
cd /Users/abuxton/src/github/dojo/aoc25/day2/aoc-day2
# default reads 'input.txt' in crate root
cargo run --release

# or provide an explicit input file
cargo run --release -- my_input.txt
```

The program prints two lines:

```
part1: <sum-of-IDs-matching-part1>
part2: <sum-of-IDs-matching-part2>
```

## Example

Example input (same as above). Expected output for the example:

```
part1: 1227775554
part2: 4174379265
```

## Implementation notes

- Part 1 can be solved by enumerating lengths k (half-lengths) and summing values x*(10^k+1) for k-digit x in range, avoiding full-range enumeration.
- Part 2 requires considering any repetition count >= 2. One approach: for each possible base length k (1..), and repetition count r (2.. while number length <= max digits), generate numbers of the form x repeated r times and count those within each range — again avoid iterating every ID by computing x ranges algebraically.
- Use u128 (or bigint) to avoid overflow for large IDs.
- Validate and skip tokens with leading zeros or malformed ranges.

## Testing

Run unit tests:

```bash
cargo test
```

Add example-based tests in `src` or `tests/` to verify correctness for both parts.

## Notes

- For performance with huge ranges, prefer arithmetic-range computations over enumerating each integer.
- See source at `src/main.rs` for the exact algorithm and numeric types used.

## Contributing & License

Contributions welcome. See repository-level contributing guidelines:
- ../../../CONTRIBUTING.md

See repository license:
- ../../../LICENSE.md
