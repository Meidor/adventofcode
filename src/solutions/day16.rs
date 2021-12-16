use bitvec::prelude::*;
use hex::FromHex;

#[derive(Debug, PartialEq)]
enum OperatorType {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    Literal = 4,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
}

impl From<u8> for OperatorType {
    fn from(v: u8) -> Self {
        match v {
            0 => OperatorType::Sum,
            1 => OperatorType::Product,
            2 => OperatorType::Minimum,
            3 => OperatorType::Maximum,
            4 => OperatorType::Literal,
            5 => OperatorType::GreaterThan,
            6 => OperatorType::LessThan,
            7 => OperatorType::EqualTo,
            _ => panic!("Invalid operator {}", v),
        }
    }
}

struct BitsPacket {
    version: u8,
    operator: OperatorType,
    length: usize,
    value: u64,
    sub_packets: Vec<BitsPacket>,
}

impl BitsPacket {
    fn from_hex_string(input: &str) -> BitsPacket {
        let bytes = Vec::from_hex(input).unwrap();
        let bits = BitSlice::<Msb0, u8>::from_slice(&bytes).unwrap();
        BitsPacket::from_bitslice(bits)
    }

    fn from_bitslice(bits: &BitSlice<Msb0, u8>) -> BitsPacket {
        let mut pp: usize = 0;
        let version = BitsPacket::parse_u8(&bits[0..3]);
        let operator = BitsPacket::parse_operator(&bits[3..6]);
        pp += 6;
        if operator == OperatorType::Literal {
            let mut is_last = false;
            let mut vbits: BitVec<Msb0, u8> = bitvec![Msb0, u8; 0];
            while !is_last {
                is_last = !bits[pp];
                vbits.push(bits[pp + 1]);
                vbits.push(bits[pp + 2]);
                vbits.push(bits[pp + 3]);
                vbits.push(bits[pp + 4]);
                pp += 5;
            }
            let value = BitsPacket::parse_u64(&vbits);
            BitsPacket {
                version,
                operator,
                value,
                length: pp,
                sub_packets: vec![],
            }
        } else {
            let length_type = bits[pp];
            pp += 1;
            let length_value: u64;
            if length_type {
                length_value = BitsPacket::parse_u64(&bits[pp..pp + 11]);
                pp += 11;
            } else {
                length_value = BitsPacket::parse_u64(&bits[pp..pp + 15]);
                pp += 15;
            }
            let sub_packets = BitsPacket::get_subpackets(&bits[pp..], length_type, length_value);
            let spl = sub_packets.iter().map(|p| p.length).sum::<usize>();
            BitsPacket {
                version,
                value: BitsPacket::calculate_value(&operator, &sub_packets),
                length: pp + spl,
                operator,
                sub_packets,
            }
        }
    }

    fn get_subpackets(
        bits: &BitSlice<Msb0, u8>,
        length_type: bool,
        length_value: u64,
    ) -> Vec<BitsPacket> {
        let mut pp: usize = 0;
        let mut packets: Vec<BitsPacket> = vec![];
        if !length_type {
            let mut total_length = 0;
            while total_length < length_value {
                let p = BitsPacket::from_bitslice(&bits[pp..]);
                pp += p.length;
                total_length += p.length as u64;
                packets.push(p);
            }
        } else {
            let mut packet_num = 0;
            while packet_num < length_value {
                let p = BitsPacket::from_bitslice(&bits[pp..]);
                pp += p.length;
                packet_num += 1;
                packets.push(p);
            }
        }
        packets
    }

    fn calculate_value(operator: &OperatorType, packets: &[BitsPacket]) -> u64 {
        match operator {
            OperatorType::Sum => BitsPacket::sum(packets),
            OperatorType::Product => BitsPacket::prod(packets),
            OperatorType::Minimum => BitsPacket::min(packets),
            OperatorType::Maximum => BitsPacket::max(packets),
            OperatorType::GreaterThan => BitsPacket::gt(packets),
            OperatorType::LessThan => BitsPacket::lt(packets),
            OperatorType::EqualTo => BitsPacket::eq(packets),
            _ => panic!("Invalid operator {:?}", operator),
        }
    }

    fn get_string(slice: &BitSlice<Msb0, u8>) -> String {
        let mut s = String::new();
        for bit in slice.iter() {
            s += if *bit { "1" } else { "0" };
        }
        s
    }

    fn parse_u64(bits: &BitSlice<Msb0, u8>) -> u64 {
        u64::from_str_radix(&BitsPacket::get_string(bits), 2).unwrap()
    }

    fn parse_u8(bits: &BitSlice<Msb0, u8>) -> u8 {
        u8::from_str_radix(&BitsPacket::get_string(bits), 2).unwrap()
    }

    fn parse_operator(bits: &BitSlice<Msb0, u8>) -> OperatorType {
        BitsPacket::parse_u8(bits).into()
    }

    fn version_sum(&self) -> u64 {
        let mut result: u64 = self.version as u64;
        result += self
            .sub_packets
            .iter()
            .map(|p| p.version_sum())
            .sum::<u64>();
        result
    }

    fn sum(packets: &[BitsPacket]) -> u64 {
        packets.iter().map(|p| p.value).sum::<u64>()
    }

    fn prod(packets: &[BitsPacket]) -> u64 {
        packets
            .iter()
            .map(|p| p.value)
            .reduce(|a, b| a * b)
            .unwrap()
    }

    fn min(packets: &[BitsPacket]) -> u64 {
        packets.iter().map(|p| p.value).min().unwrap()
    }

    fn max(packets: &[BitsPacket]) -> u64 {
        packets.iter().map(|p| p.value).max().unwrap()
    }

    fn gt(packets: &[BitsPacket]) -> u64 {
        if packets[0].value > packets[1].value {
            1
        } else {
            0
        }
    }

    fn lt(packets: &[BitsPacket]) -> u64 {
        if packets[0].value < packets[1].value {
            1
        } else {
            0
        }
    }

    fn eq(packets: &[BitsPacket]) -> u64 {
        if packets[0].value == packets[1].value {
            1
        } else {
            0
        }
    }
}

#[inline]
pub fn part_one(lines: &[String]) -> String {
    let packet = BitsPacket::from_hex_string(&lines[0]);
    packet.version_sum().to_string()
}

#[inline]
pub fn part_two(lines: &[String]) -> String {
    let packet = BitsPacket::from_hex_string(&lines[0]);
    packet.value.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input_one() -> Vec<String> {
        vec!["A0016C880162017C3686B18A3D4780".to_string()]
    }

    fn test_input_two() -> Vec<String> {
        vec!["9C0141080250320F1802104A08".to_string()]
    }

    #[test]
    fn test_parse_literal_packet() {
        let packet = BitsPacket::from_hex_string("D2FE28");

        let expected_version = 6;
        let actual_version = packet.version;

        let expected_type = OperatorType::Literal;
        let actual_type = packet.operator;

        let expected_value = 2021u64;
        let actual_value = packet.value;

        let expected_length = 21;
        let actual_length = packet.length;

        assert_eq!(expected_version, actual_version);
        assert_eq!(expected_type, actual_type);
        assert_eq!(expected_value, actual_value);
        assert_eq!(expected_length, actual_length);
    }

    #[test]
    fn test_parse_operator_packet() {
        let packet = BitsPacket::from_hex_string("38006F45291200");

        let expected_value_a: u64 = 10;
        let expected_value_b: u64 = 20;

        let actual_value_a = packet.sub_packets[0].value;
        let actual_value_b = packet.sub_packets[1].value;

        assert_eq!(expected_value_a, actual_value_a);
        assert_eq!(expected_value_b, actual_value_b);
    }

    #[test]
    fn test_part_one() {
        let expected = "31";
        let actual = part_one(&test_input_one());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = "1";
        let actual = part_two(&test_input_two());
        assert_eq!(expected, actual);
    }
}
