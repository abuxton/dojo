# ping

Welcome to ping — a minimal Rust example demonstrating rootless ICMP (ping) using a SOCK_DGRAM socket with the ICMP protocol.

This crate shows how to build and send an ICMPv4 Echo Request without requiring a raw socket (root) on systems that allow unprivileged datagram ICMP (depends on kernel configuration). The implementation uses the socket2 crate and includes a small ICMP checksum helper, CLI, RTT timing, and unit tests for checksum logic.

## Requirements

- Rust toolchain (rustup + cargo)
- Internet access to ping the target address
- The socket2 crate is already included in Cargo.toml (used by the example)
- Platform notes:
  - Some Linux systems restrict unprivileged ICMP even for SOCK_DGRAM; you may need CAP_NET_RAW, or kernel config like ping_group_range.
  - macOS and Linux differ: macOS expects a correct checksum and includes the IPv4 header in replies; Linux may override checksum/identifier and omit the IP header.

## Build

From the crate root:

```bash
cd /Users/abuxton/src/github/dojo/rust/ping
cargo build
```

## Usage

Run the binary with an optional destination and optional payload string.

Default destination: 1.1.1.1
Default payload: "hello"

Examples:

Run with defaults:

```shell
cargo run --
```

Ping a specific host:

```shell
cargo run -- 8.8.8.8
```

Ping with a custom payload:

```shell
cargo run -- 8.8.8.8 "custom payload"

```

### Notes:

- The destination port is ignored for ICMP; you can pass IPv4 address or host (resolver behavior depends on your system).
- The program prints a single reply line with type, code, sequence, payload, and measured RTT in ms.

## Tests

Run unit tests (checksum and packet build tests included):

## Caveats & Troubleshooting

- Permission denied or bind errors:
  - Some systems require additional capabilities (e.g., CAP_NET_RAW) or other kernel configuration to permit unprivileged ICMP. See your distribution docs for ping_group_range or consider running with appropriate privileges for testing.
- Platform differences:
  - macOS includes the IPv4 header in received datagrams and requires a correct ICMP checksum.
  - Linux kernels may rewrite checksum/identifier for SOCK_DGRAM+IPPROTO_ICMP; observed behavior can vary by kernel version and distro.
- IPv6 is not implemented in this example (ICMPv6 uses different packet format and protocol).

## Implementation notes

- The code demonstrates:
  - Creating a SOCK_DGRAM socket with ICMP protocol via socket2.
  - Building an ICMP Echo Request (identifier, sequence, payload).
  - Calculating ICMP checksum (ones' complement sum).
  - Receiving a reply and parsing the ICMP header safely (bounds-checked).
  - Measuring RTT using Instant.

## Contributing

Contributions welcome. Open an issue or PR with improvements (CLI, IPv6, better error handling, async support).

## License

This project is licensed under the DBAD License — see LICENSE.md in the repo root.
