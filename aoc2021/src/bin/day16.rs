use aoc_helpers::*;

const YEAR: u32 = 2021;
const DAY: u32 = 16;

fn main() {
    let input = get_input(YEAR, DAY);
    if prompt_for_part(1) {
        let result = part_one(&input);
        println!("Part one: {}", result);
        if prompt_to_submit() {
            println!("{}", submit_answer(YEAR, DAY, 1, &result.to_string()));
        }
    }
    if prompt_for_part(2) {
        let result = part_two(&input);
        println!("Part two: {}", result);
        if prompt_to_submit() {
            println!("{}", submit_answer(YEAR, DAY, 2, &result.to_string()));
        }
    }
}

pub struct Packet {
    version: u8,
    type_id: u8,
    value: u128,
    sub_packets: Vec<Packet>,
}

impl Packet {
    pub fn new(version: u8, type_id: u8) -> Packet {
        Packet {
            version: version,
            type_id: type_id,
            value: 0,
            sub_packets: Vec::new(),
        }
    }
}

pub fn hex_to_bytes(hex_str: &str) -> Vec<u8> {
    // One byte (u8) is exactly 2 hex digits
    let mut bytes: Vec<u8> = Vec::with_capacity(hex_str.len() / 2);
    let mut i = 0usize;
    while i < hex_str.len() {
        let byte = u8::from_str_radix(&hex_str[i..i + 2], 16).unwrap();
        bytes.push(byte);
        i += 2;
    }
    bytes
}

pub fn get_bit(bytes: &Vec<u8>, cursor: usize) -> u8 {
    if cursor >= bytes.len() * 8 {
        panic!("cursor ({}) out of bounds ({})", cursor, bytes.len() * 8);
    }
    (bytes[cursor / 8] >> (8 - (cursor % 8) - 1)) & 0b00000001
}

pub fn parse_literal(bytes: &Vec<u8>, cursor: &mut usize) -> u128 {
    // The two most significant bits of the literal number is the last two
    // bits of the packet's first byte (since the header occupies the first
    // 6 bits).
    let mut nibble: u8 = 0;
    // These two bits will never be enough for a literal on their own.  The
    // first bit _should_ always be a 1 (since more bytes are incoming), and
    // we'll need at least 3 more bits to complete this nibble.
    let mut bits_needed = 5;
    let mut literal = 0u128;
    loop {
        while bits_needed > 0 {
            nibble = (nibble << 1) | get_bit(bytes, *cursor);
            *cursor += 1;
            bits_needed -= 1;
        }
        literal = (literal << 4) + (nibble & 0b00001111) as u128;
        if nibble & 0b00010000 == 0 {
            break;
        }
        nibble = 0;
        bits_needed = 5;
    }
    literal
}

pub fn packet_parser(bytes: &Vec<u8>, cursor: &mut usize) -> Packet {
    // Initialize the packet.
    let mut version = 0;
    let mut type_id = 0;
    for _i in 0..3 {
        version = (version << 1) | get_bit(bytes, *cursor);
        *cursor += 1;
    }
    for _i in 0..3 {
        type_id = (type_id << 1) | get_bit(bytes, *cursor);
        *cursor += 1;
    }
    let mut packet = Packet::new(version, type_id);
    if type_id == 4 {
        packet.value = parse_literal(bytes, cursor);
        return packet;
    }
    let length_type = get_bit(bytes, *cursor);
    *cursor += 1;
    if length_type == 0 {
        let mut length = 0usize;
        for _i in 0..15 {
            length = (length << 1) | get_bit(bytes, *cursor) as usize;
            *cursor += 1;
        }
        length += *cursor;
        while *cursor < length {
            let sub_packet = packet_parser(bytes, cursor);
            packet.sub_packets.push(sub_packet);
        }
    } else {
        let mut count = 0usize;
        for _i in 0..11 {
            count = (count << 1) | get_bit(bytes, *cursor) as usize;
            *cursor += 1;
        }
        for _i in 0..count {
            let sub_packet = packet_parser(bytes, cursor);
            packet.sub_packets.push(sub_packet);
        }
    }
    packet
}

pub fn version_sum(packet: &Packet, initial: &mut u64) {
    *initial += packet.version as u64;
    for sp in &packet.sub_packets {
        version_sum(&sp, initial);
    }
}

pub fn resolve(packet: &Packet) -> u128 {
    return match packet.type_id {
        0 => packet
            .sub_packets
            .iter()
            .fold(0, |acc, sp| acc + resolve(sp)),
        1 => packet
            .sub_packets
            .iter()
            .fold(1, |acc, sp| acc * resolve(sp)),
        2 => packet
            .sub_packets
            .iter()
            .fold(u128::MAX, |acc, sp| std::cmp::min(acc, resolve(sp))),
        3 => packet
            .sub_packets
            .iter()
            .fold(0, |acc, sp| std::cmp::max(acc, resolve(sp))),
        4 => packet.value,
        5 => {
            if resolve(&packet.sub_packets[0]) > resolve(&packet.sub_packets[1]) {
                1
            } else {
                0
            }
        }
        6 => {
            if resolve(&packet.sub_packets[0]) < resolve(&packet.sub_packets[1]) {
                1
            } else {
                0
            }
        }
        7 => {
            if resolve(&packet.sub_packets[0]) == resolve(&packet.sub_packets[1]) {
                1
            } else {
                0
            }
        }
        _ => panic!("unexpected type_id: {}", packet.type_id),
    };
}

pub fn part_one(input: &str) -> u64 {
    let bytes = hex_to_bytes(input.trim());
    let mut cursor = 0;
    let packet = packet_parser(&bytes, &mut cursor);
    let mut sum = 0;
    version_sum(&packet, &mut sum);
    sum
}

pub fn part_two(input: &str) -> u128 {
    let bytes = hex_to_bytes(input.trim());
    let mut cursor = 0;
    let packet = packet_parser(&bytes, &mut cursor);
    resolve(&packet)
}

#[cfg(test)]
mod day16_tests {
    use super::*;

    #[test]
    fn literal_test() {
        // "D2FE28" -> 110100101111111000101000
        let bytes = hex_to_bytes("D2FE28");
        let mut cursor = 0;
        let packet = packet_parser(&bytes, &mut cursor);
        assert_eq!(packet.value, 2021);
    }

    #[test]
    fn length_operator_test() {
        // "38006F45291200" -> 00111000000000000110111101000101001010010001001000000000
        let bytes = hex_to_bytes("38006F45291200");
        let mut cursor = 0;
        let packet = packet_parser(&bytes, &mut cursor);
        assert_eq!(packet.value, 0);
        assert_eq!(packet.sub_packets.len(), 2);
        assert_eq!(packet.sub_packets[0].value, 10);
        assert_eq!(packet.sub_packets[1].value, 20);
    }

    #[test]
    fn count_operator_test() {
        // "EE00D40C823060" -> 11101110000000001101010000001100100000100011000001100000
        let bytes = hex_to_bytes("EE00D40C823060");
        let mut cursor = 0;
        let packet = packet_parser(&bytes, &mut cursor);
        assert_eq!(packet.value, 0);
        assert_eq!(packet.sub_packets.len(), 3);
        assert_eq!(packet.sub_packets[0].value, 1);
        assert_eq!(packet.sub_packets[1].value, 2);
        assert_eq!(packet.sub_packets[2].value, 3);
    }

    #[test]
    fn samples_part1() {
        assert_eq!(part_one("8A004A801A8002F478"), 16);
        assert_eq!(part_one("620080001611562C8802118E34"), 12);
        assert_eq!(part_one("C0015000016115A2E0802F182340"), 23);
        assert_eq!(part_one("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn samples_part2() {
        assert_eq!(part_two("C200B40A82"), 3);
        assert_eq!(part_two("04005AC33890"), 54);
        assert_eq!(part_two("880086C3E88112"), 7);
        assert_eq!(part_two("CE00C43D881120"), 9);
        assert_eq!(part_two("D8005AC2A8F0"), 1);
        assert_eq!(part_two("F600BC2D8F"), 0);
        assert_eq!(part_two("9C005AC2F8F0"), 0);
        assert_eq!(part_two("9C0141080250320F1802104A08"), 1);
    }
}
