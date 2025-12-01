
# aoc25-day2

Welcome to aoc25-day2

## Description

--- Part Two ---
Count the number of times the dial points at 0 for the whole sequence of rotations.
This includes clicks that occur during a rotation as well as the final position at the end of each rotation.

The dial has 100 positions (0..99) and starts at 50. Rotations are lines like `L68` or `R48`.
- `L` rotates left (decreasing positions)
- `R` rotates right (increasing positions)
Positions wrap modulo 100.

## Getting Started

Clone this repository and build the crate.

### Usage

Place your puzzle input in the crate root as `input.txt`, or pass a path as the first argument.

Example `input.txt`:
```
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
```

Run with the default file:
```bash
cd /Users/abuxton/src/github/dojo/aoc25/day2/aoc-day2
cargo run --release
```

Or run with an explicit file:
```bash
cargo run --release -- my_input.txt
```

Expected output for the example data:
```
part1: 3
part2: 6
```

- part1: times the dial ends a rotation pointing at 0
- part2: times any click (during rotations or at the end) lands on 0

## Testing

Run unit tests:
```bash
cargo test
```

## Contributing

Contributions are always welcome! If you're interested in contributing, please review our [contributing guidelines](../../../CONTRIBUTING.md).

## License

This project is licensed under the DBAD License - see the [LICENSE.md](../../../LICENSE.md) file for details.
...existing code...
```// filepath: /Users/abuxton/src/github/dojo/aoc25/day2/aoc-day2/README.md
...existing code...
# aoc25-day2

Welcome to aoc25-day2

## Description

--- Part Two ---
Count the number of times the dial points at 0 for the whole sequence of rotations.
This includes clicks that occur during a rotation as well as the final position at the end of each rotation.

The dial has 100 positions (0..99) and starts at 50. Rotations are lines like `L68` or `R48`.
- `L` rotates left (decreasing positions)
- `R` rotates right (increasing positions)
Positions wrap modulo 100.

## Getting Started

Clone this repository and build the crate.

### Usage

Place your puzzle input in the crate root as `input.txt`, or pass a path as the first argument.

Example `input.txt`:
```
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
```

Run with the default file:
```bash
cd /Users/abuxton/src/github/dojo/aoc25/day2/aoc-day2
cargo run --release
```

Or run with an explicit file:
```bash
cargo run --release -- my_input.txt
```

Expected output for the example data:
```
part1: 3
part2: 6
```

- part1: times the dial ends a rotation pointing at 0
- part2: times any click (during rotations or at the end) lands on 0

## Testing

Run unit tests:
```bash
cargo test
```

## Contributing

Contributions are always welcome! If you're interested in contributing, please review our [contributing guidelines](../../../CONTRIBUTING.md).

## License

This project is licensed under the DBAD License - see the [LICENSE.md](../../../LICENSE.md) file for details.
x
