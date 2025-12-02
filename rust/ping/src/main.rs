use socket2::{Domain, Protocol, Socket, Type};
use std::error::Error;
use std::net::UdpSocket;
use std::time::{Duration, Instant};

/// Compute ICMP checksum (ones' complement of 16-bit sum)
fn icmp_checksum(buf: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    let mut i = 0;
    while i + 1 < buf.len() {
        let word = u16::from_be_bytes([buf[i], buf[i + 1]]) as u32;
        sum = sum.wrapping_add(word);
        i += 2;
    }
    if i < buf.len() {
        // pad odd byte with zero in the low-order byte (network order)
        let word = (buf[i] as u32) << 8;
        sum = sum.wrapping_add(word);
    }
    // fold carries
    while (sum >> 16) != 0 {
        sum = (sum & 0xffff).wrapping_add(sum >> 16);
    }
    !(sum as u16)
}

/// Build a simple ICMPv4 Echo Request packet with provided identifier, sequence and payload.
/// The returned packet has checksum field filled.
fn build_echo_request(identifier: u16, sequence: u16, payload: &[u8]) -> Vec<u8> {
    let mut packet: Vec<u8> = Vec::with_capacity(8 + payload.len());
    packet.push(8u8); // type: Echo Request
    packet.push(0u8); // code
    packet.push(0u8); // checksum hi (placeholder)
    packet.push(0u8); // checksum lo (placeholder)
    packet.extend_from_slice(&identifier.to_be_bytes());
    packet.extend_from_slice(&sequence.to_be_bytes());
    packet.extend_from_slice(payload);

    let checksum = icmp_checksum(&packet);
    packet[2] = (checksum >> 8) as u8;
    packet[3] = (checksum & 0xff) as u8;
    packet
}

fn main() -> Result<(), Box<dyn Error>> {
    // CLI: first arg is destination host (default 1.1.1.1), second optional arg is payload string
    let mut args = std::env::args().skip(1);
    let dest = args.next().unwrap_or_else(|| "1.1.1.1".to_string());
    let payload_str = args.next().unwrap_or_else(|| "hello".to_string());
    // ensure destination has a port (port is ignored for ICMP over SOCK_DGRAM)
    let dest_addr = if dest.contains(':') {
        dest
    } else {
        format!("{dest}:0")
    };

    // Create a datagram socket with ICMP protocol (may be allowed without root on some systems)
    let sock = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::ICMPV4))?;
    let udp: UdpSocket = sock.into();

    // Optional: set a read timeout so recv_from doesn't block forever
    udp.set_read_timeout(Some(Duration::from_secs(3)))?;

    // identifier from pid (fits in u16)
    let identifier: u16 = (std::process::id() & 0xffff) as u16;
    let sequence: u16 = 1;
    let payload = payload_str.as_bytes();

    let packet = build_echo_request(identifier, sequence, payload);

    let send_time = Instant::now();
    udp.send_to(&packet, &dest_addr)?;

    let mut buffer = vec![0u8; 1500];
    let (size, from_addr) = match udp.recv_from(&mut buffer) {
        Ok(v) => v,
        Err(e) => return Err(format!("recv_from failed: {e}").into()),
    };
    let rtt = Instant::now().duration_since(send_time);

    // macOS includes the IPv4 header; Linux typically does not
    #[cfg(target_os = "macos")]
    const IP_HEADER_LEN: usize = 20;
    #[cfg(not(target_os = "macos"))]
    const IP_HEADER_LEN: usize = 0;

    if size < IP_HEADER_LEN + 8 {
        return Err(format!("reply too short: {} bytes", size).into());
    }
    let data = &buffer[IP_HEADER_LEN..size];

    // safe parsing with bounds checks
    // let reply_type = data.get(0).copied().ok_or("missing type")?;
    let reply_type = data.first().copied().ok_or("missing type")?;
    let reply_code = data.get(1).copied().ok_or("missing code")?;
    // checksum at 2..3, identifier at 4..5, sequence at 6..7
    let reply_seq = {
        let b6 = *data.get(6).ok_or("missing seq byte 0")?;
        let b7 = *data.get(7).ok_or("missing seq byte 1")?;
        ((b6 as u16) << 8) | (b7 as u16)
    };
    let reply_payload = &data.get(8..).unwrap_or(&[]);

    println!(
        "Received ICMP reply from {}: type={}, code={}, sequence={}, payload={:?}, rtt_ms={:.3}",
        from_addr,
        reply_type,
        reply_code,
        reply_seq,
        reply_payload,
        rtt.as_secs_f64() * 1000.0
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_even_length_roundtrip() {
        // build packet with checksum zeroed, compute checksum, insert and verify whole-packet checksum == 0
        let identifier = 0x1234u16;
        let sequence = 0x0001u16;
        let payload = b"abcd"; // even length payload
        // let mut packet = Vec::new();
        // packet.push(8u8);
        // packet.push(0u8);
        // packet.push(0u8);
        // packet.push(0u8);
        let mut packet = vec![8u8, 0u8, 0u8, 0u8];
        packet.extend_from_slice(&identifier.to_be_bytes());
        packet.extend_from_slice(&sequence.to_be_bytes());
        packet.extend_from_slice(payload);

        let checksum = icmp_checksum(&packet);
        packet[2] = (checksum >> 8) as u8;
        packet[3] = (checksum & 0xff) as u8;

        // recomputing checksum over packet with checksum field set should yield 0
        assert_eq!(icmp_checksum(&packet), 0u16);
    }

    #[test]
    fn checksum_odd_length_roundtrip() {
        let identifier = 0x4321u16;
        let sequence = 0x0002u16;
        let payload = b"abc"; // odd length payload
        // let mut packet = Vec::new();
        // packet.push(8u8);
        // packet.push(0u8);
        // packet.push(0u8);
        // packet.push(0u8);
        let mut packet = vec![8u8, 0u8, 0u8, 0u8];
        packet.extend_from_slice(&identifier.to_be_bytes());
        packet.extend_from_slice(&sequence.to_be_bytes());
        packet.extend_from_slice(payload);

        let checksum = icmp_checksum(&packet);
        packet[2] = (checksum >> 8) as u8;
        packet[3] = (checksum & 0xff) as u8;

        // recomputing checksum over packet with checksum field set should yield 0
        assert_eq!(icmp_checksum(&packet), 0u16);
    }

    #[test]
    fn build_echo_request_sets_checksum() {
        let id = 0x1111u16;
        let seq = 0x2222u16;
        let payload = b"pingpayload";
        let packet = build_echo_request(id, seq, payload);
        // packet[2..4] should be non-zero for non-empty payload
        assert!(packet[2] != 0 || packet[3] != 0);
        // recomputing checksum should be zero
        assert_eq!(icmp_checksum(&packet), 0u16);
        // identifier and sequence preserved
        assert_eq!(u16::from_be_bytes([packet[4], packet[5]]), id);
        assert_eq!(u16::from_be_bytes([packet[6], packet[7]]), seq);
    }
}
