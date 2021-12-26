use std::str::FromStr;

pub fn star_one(input: &str) -> u64 {
    let packet = input.trim().parse::<Packet>().unwrap();

    packet.version_sum()
}

pub fn star_two(input: &str) -> u64 {
    let packet = input.trim().parse::<Packet>().unwrap();

    packet.resolve()
}

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    Literal {
        type_id: TypeId,
        version: u8,
        value: u64,
    },
    Operator {
        type_id: TypeId,
        version: u8,
        sub_packets: Vec<Packet>,
    },
}

impl Packet {
    fn version_sum(&self) -> u64 {
        match self {
            Packet::Literal { version, .. } => *version as u64,
            Packet::Operator {
                version,
                sub_packets,
                ..
            } => *version as u64 + sub_packets.iter().fold(0, |acc, p| acc + p.version_sum()),
        }
    }

    fn resolve(&self) -> u64 {
        match self {
            Packet::Literal { value, .. } => *value,
            Packet::Operator {
                type_id,
                sub_packets,
                ..
            } => match type_id {
                TypeId::Sum => sub_packets.iter().map(|p| p.resolve()).sum(),
                TypeId::Product => sub_packets.iter().map(|p| p.resolve()).product(),
                TypeId::Min => sub_packets.iter().map(|p| p.resolve()).min().unwrap(),
                TypeId::Max => sub_packets.iter().map(|p| p.resolve()).max().unwrap(),
                TypeId::Gt => {
                    let p1 = sub_packets[0].resolve();
                    let p2 = sub_packets[1].resolve();

                    if p1 > p2 {
                        1
                    } else {
                        0
                    }
                }
                TypeId::Lt => {
                    let p1 = sub_packets[0].resolve();
                    let p2 = sub_packets[1].resolve();

                    if p1 < p2 {
                        1
                    } else {
                        0
                    }
                }
                TypeId::Eq => {
                    let p1 = sub_packets[0].resolve();
                    let p2 = sub_packets[1].resolve();

                    if p1 == p2 {
                        1
                    } else {
                        0
                    }
                }

                TypeId::Literal => unreachable!(),
            },
        }
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = to_bytes(s);
        let (p, _) = parse_packet(0, &bytes);

        return Ok(p);
    }
}

#[derive(Debug, Eq, PartialEq)]
enum TypeId {
    Sum = 0,
    Product = 1,
    Min = 2,
    Max = 3,
    Literal = 4,
    Gt = 5,
    Lt = 6,
    Eq = 7,
}

impl TypeId {
    fn is_literal(&self) -> bool {
        matches!(self, TypeId::Literal)
    }

    fn is_operator(&self) -> bool {
        !self.is_literal()
    }
}

impl TryFrom<u8> for TypeId {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::Sum as u8 => Ok(Self::Sum),
            x if x == Self::Product as u8 => Ok(Self::Product),
            x if x == Self::Min as u8 => Ok(Self::Min),
            x if x == Self::Max as u8 => Ok(Self::Max),
            x if x == Self::Literal as u8 => Ok(Self::Literal),
            x if x == Self::Gt as u8 => Ok(Self::Gt),
            x if x == Self::Lt as u8 => Ok(Self::Lt),
            x if x == Self::Eq as u8 => Ok(Self::Eq),
            _ => Err(format!("Invalid TypeId {}", value)),
        }
    }
}

fn extract_type_and_version(at: usize, from: &[u8]) -> (u8, u8) {
    let v = extract_bits(3, at, from);
    let t = extract_bits(3, at + 3, from);

    (v as u8, t as u8)
}

fn mask_out_bits(count: u8, at: u8, from: u8) -> u8 {
    if count == 8 && at == 0 {
        return from;
    }

    let shift = (8 - at) - count;
    let mask = !(!0_u8 << count) << shift;

    (mask & from) >> shift
}

fn to_bytes(input: &str) -> Vec<u8> {
    let chars: Vec<_> = input.chars().collect();

    chars
        .chunks(2)
        .map(|c| {
            let (h, l) = (c[0], c[1]);

            ((h.to_digit(16).unwrap() << 4) | (l.to_digit(16).unwrap())) as u8
        })
        .collect()
}

fn debug_print(at: usize, from: &[u8]) {
    let byte = at / 8;
    let bit_in_byte = at % 8;

    if byte != 0 {
        let space = " ".repeat(bit_in_byte - 1);
        println!("{:b}{:b}", from[byte - 1], from[byte]);
        println!("^{}^", space);
        println!("|{}|", space);
        println!("{}{}{}", byte * 8, space, at);
    } else {
        let space = " ".repeat(bit_in_byte);
        println!("{:b}", from[byte]);
        println!("{}^", space);
        println!("{}|", space);
        println!("{}{}", space, at);
    }
}

fn parse_packet(at: usize, from: &[u8]) -> (Packet, usize) {
    let (version, type_id) = extract_type_and_version(at, &from);
    let type_id: TypeId = type_id.try_into().unwrap();

    if type_id.is_literal() {
        let (value, new_index) = parse_literal(at, &from);

        let p = Packet::Literal {
            type_id,
            version,
            value,
        };

        (p, new_index)
    } else {
        let (length_type_id, length, new_index) = parse_operator(at, &from);

        let (sub_packets, new_index) = if length_type_id == 0 {
            let mut result = vec![];
            let mut current_index = new_index;

            while (current_index + 6) < (new_index + length as usize) {
                let (p, op_index) = parse_packet(current_index, &from);
                result.push(p);
                current_index = op_index;
            }

            (result, current_index)
        } else {
            let mut result = vec![];
            let mut current_index = new_index;

            for _ in 0..length {
                let (p, op_index) = parse_packet(current_index, &from);
                result.push(p);
                current_index = op_index;
            }

            (result, current_index)
        };

        let packet = Packet::Operator {
            type_id,
            version,
            sub_packets,
        };

        (packet, new_index)
    }
}

fn parse_literal(at: usize, from: &[u8]) -> (u64, usize) {
    let (_, t) = extract_type_and_version(at, &from);
    assert!(t == 4, "Literal packets should have type 4");

    let mut bit_index: usize = at + 6;
    let mut nibbles = vec![];
    loop {
        let first_bit = extract_bits(1, bit_index, from);
        let should_continue = first_bit == 1;

        let value = extract_bits(4, bit_index + 1, from);
        nibbles.push(value);

        bit_index += 5;
        if !should_continue {
            break;
        }
    }

    let nibble_count = nibbles.len();
    let value = nibbles
        .into_iter()
        .enumerate()
        .fold(0_u64, |acc, (n_idx, v)| {
            acc | (v << (nibble_count - n_idx - 1) * 4)
        });

    (value, bit_index)
}

fn parse_operator(at: usize, from: &[u8]) -> (u8, u32, usize) {
    let (_, t) = extract_type_and_version(at, &from);
    assert!(t != 4, "Operator packets should NOT have type 4");

    let bit_index: usize = at + 6;
    let length_type_id = extract_bits(1, bit_index, &from) as u8;

    if length_type_id == 0 {
        // Next 15 bits contain the length of the sub packets

        // extract_bits can't handle counts larger than 8
        let b1 = extract_bits(8, bit_index + 1, from);
        let b2 = extract_bits(7, bit_index + 9, from);

        (
            length_type_id,
            ((b1 << 7) | b2).try_into().unwrap(),
            bit_index + 16,
        )
    } else {
        // Next 11 bits contain the number of sub packets.

        // extract_bits can't handle counts larger than 8
        let b1 = extract_bits(8, bit_index + 1, from);
        let b2 = extract_bits(3, bit_index + 9, from);

        (
            length_type_id,
            ((b1 << 3) | b2).try_into().unwrap(),
            bit_index + 12,
        )
    }
}

fn extract_bits(count: u8, at: usize, from: &[u8]) -> u64 {
    let start_byte_index: usize = (at / 8).into();

    // The index into the first byte
    let bit_index: u8 = (at % 8) as u8;

    // Count 5 at index 5
    // 0b00000000, 0b00000000
    //        ^       ^
    //        |       |
    //        |       |
    //      start    end
    if bit_index + count <= 8 {
        // The whole result is contained in this byte
        mask_out_bits(count, bit_index, from[start_byte_index]) as u64
    } else {
        // The result is split over a byte boundary

        let remainder = 8 - bit_index;
        let b1 = mask_out_bits(remainder, bit_index, from[start_byte_index]);
        let b2 = mask_out_bits(count - remainder, 0, from[start_byte_index + 1]);

        ((b1 << (count - remainder)) | b2).into()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        extract_bits, extract_type_and_version, mask_out_bits, parse_literal, parse_operator,
        star_one, star_two, to_bytes, Packet,
    };

    #[test]
    fn test_star_one() {
        let cases = [
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ];

        for (input, expected) in cases {
            assert_eq!(
                star_one(input),
                expected,
                "Expected correct answer for input `{}`",
                input
            );
        }
    }

    #[test]
    fn test_star_two() {
        let cases = [
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];

        for (input, expected) in cases {
            assert_eq!(
                star_two(input),
                expected,
                "Expected correct answer for input `{}`",
                input
            );
        }
    }

    #[test]
    fn test_to_bytes() {
        let result = to_bytes("D2FE");

        assert_eq!(result, vec![0xD2, 0xFE]);
    }

    #[test]
    fn test_extract_type_and_version() {
        let cases = [
            (0b000_000_00, (0, 0)),
            (0b000_000_11, (0, 0)),
            (0b100_100_00, (4, 4)),
            (0b101_101_00, (5, 5)),
            (0b111_001_00, (7, 1)),
            (0b001_111_00, (1, 7)),
        ];

        for (byte, (et, ev)) in cases {
            let (t, v) = extract_type_and_version(0, &[byte]);

            assert_eq!(t, et, "Wrong type for {:b}", byte);
            assert_eq!(v, ev, "Wrong version for {:b}", byte);
        }
    }

    #[test]
    fn test_mask_out_bits() {
        let cases = [
            ((0b100_000_00, 1, 0), 1),
            ((0b000_000_01, 1, 7), 1),
            ((0b000_100_00, 1, 3), 1),
            ((0b001_010_00, 3, 2), 0b101),
            ((0, 8, 0), 0),
        ];

        for ((byte, count, at), e) in cases {
            let b = mask_out_bits(count, at, byte);

            assert_eq!(
                b, e,
                "Invalid bits({}) extracted from {:b} at {}",
                count, byte, at
            );
        }
    }

    #[test]
    fn test_extract_bits() {
        let cases: &[((&[u8], u8, usize), u64)] = &[
            ((&[0b00101101], 6, 2), 0b101101),
            ((&[0b00000101, 0b01000000], 5, 5), 0b10101),
            ((&[0b00000001, 0b01011010], 8, 7), 0b10101101),
            ((&[0b00000010, 0b00001101, 0b01111000], 8, 7), 0b00000110),
            ((&[0b00000010, 0b00001101, 0b01111000], 3, 15), 0b101),
        ];

        for ((slice, count, at), e) in cases {
            let b = extract_bits(*count, *at, slice);

            assert_eq!(
                b, *e,
                "Invalid bits({}) extracted from {:?} at {}. Got {:b} expected {:b}",
                *count, &slice, *at, b, *e
            );
        }
    }

    #[test]
    fn test_parse_literal() {
        let bytes = to_bytes("D2FE28");

        let (literal, bit_index) = parse_literal(0, &bytes);

        assert_eq!(literal, 2021);
        assert_eq!(bit_index, 21);
    }

    #[test]
    fn test_parse_operator_zero_type_length_id() {
        let bytes = to_bytes("38006F45291200");

        let (type_length_id, length, bit_index) = parse_operator(0, &bytes);

        assert_eq!(type_length_id, 0);
        assert_eq!(length, 27);
        assert_eq!(bit_index, 22);
    }

    #[test]
    fn test_parse_operator_one_type_length_id() {
        let bytes = to_bytes("EE00D40C823060");

        let (type_length_id, length, bit_index) = parse_operator(0, &bytes);

        assert_eq!(type_length_id, 1);
        assert_eq!(length, 3);
        assert_eq!(bit_index, 18);
    }

    #[test]
    fn test_full_packet_parse_simple() {
        let input = "8A004A801A8002F478";
        let packet: Result<Packet, _> = input.parse();

        assert!(packet.is_ok(), "Failed to parse `{}` as Packet", input);
    }
}
