# Day3: Lobby

Welcome — Advent of Code Day 3 implementation in Rust (maintained by abuxton).

## Description

### Part one

You descend a short staircase, enter the surprisingly vast lobby, and are quickly cleared by the security checkpoint. When you get to the main elevators, however, you discover that each one has a red light above it: they're all offline.

"Sorry about that," an Elf apologizes as she tinkers with a nearby control panel. "Some kind of electrical surge seems to have fried them. I'll try to get them online soon."

You explain your need to get further underground. "Well, you could at least take the escalator down to the printing department, not that you'd get much further than that without the elevators working. That is, you could if the escalator weren't also offline."

"But, don't worry! It's not fried; it just needs power. Maybe you can get it running while I keep working on the elevators."

There are batteries nearby that can supply emergency power to the escalator for just such an occasion. The batteries are each labeled with their joltage rating, a single digit from 1..=9. The puzzle input is a multiline file where each line is a bank of batteries (a sequence of digits). From each bank you must choose exactly two batteries (keep their original order) and the bank's output joltage is the two-digit number formed by those digits (for batteries at positions i<j the value is 10*digit[i] + digit[j]). Your task is to pick the two batteries that produce the largest possible two-digit number for each bank, then sum those maxima across all banks.

Example input:
```
987654321111111
811111111111119
234234234234278
818181911112111
```
Example explanation:
- from `987654321111111` choose `9` and `8` -> 98
- from `811111111111119` choose `8` and `9` -> 89
- from `234234234234278` choose `7` and `8` -> 78
- from `818181911112111` choose `9` and `2` -> 92

Total: `98 + 89 + 78 + 92 = 357`

### Part two

The escalator doesn't move. The Elf explains that it probably needs more joltage to overcome the static friction of the system and hits the big red "joltage limit safety override" button. You lose count of the number of times she needs to confirm "yes, I'm sure" and decorate the lobby a bit while you wait.

Now, you need to make the largest joltage by turning on exactly twelve batteries within each bank.

The joltage output for the bank is still the number formed by the digits of the batteries you've turned on; the only difference is that now there will be 12 digits in each bank's joltage output instead of two.

Consider again the example from before:

987654321111111
811111111111119
234234234234278
818181911112111
Now, the joltages are much larger:

In 987654321111111, the largest joltage can be found by turning on everything except some 1s at the end to produce 987654321111.
In the digit sequence 811111111111119, the largest joltage can be found by turning on everything except some 1s, producing 811111111119.
In 234234234234278, the largest joltage can be found by turning on everything except a 2 battery, a 3 battery, and another 2 battery near the start to produce 434234234278.
In 818181911112111, the joltage 888911112111 is produced by turning on everything except some 1s near the front.
The total output joltage is now much larger: 987654321111 + 811111111119 + 434234234278 + 888911112111 = 3121910778619.

What is the new total output joltage?

## Getting Started

Requirements
- Rust toolchain (rustup + cargo)

Build
```bash
cd /Users/abuxton/src/github/dojo/aoc25/day3
cargo build
```

## Usage

By convention the program reads `input.txt` from the crate root unless you pass a filename as the first argument.

Run with default input:
```bash
cargo run --
```

Run with explicit input file:
```bash
cargo run -- path/to/input.txt
```

Expected output for the README example:
```
357
```

Behavior and assumptions
- Lines that are empty or contain fewer than two digits contribute `0`.
- Non-digit characters are ignored when extracting digits from a line.
- Digits retain their original order; you may not rearrange batteries.

## Tests

Unit tests live in `src` and include example-based checks. Run:
```bash
cargo test
```

## Implementation

See `src/main.rs` for the Rust implementation. The solver:
- Parses lines, extracts digits,
- For each line computes the largest two-digit value obtainable by taking any pair i<j,
- Sums those maxima and prints the total.

## Contributing

Contributions welcome. See repository-level contributing guidelines:
- ../../../CONTRIBUTING.md

## License

This project is licensed under the MIT License — see repository `LICENSE.md`.
