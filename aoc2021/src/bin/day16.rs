use aoc_helpers::parsing::*;
use aoc_helpers::runner::*;

use aoc_2021_libs::packet;

fn main() {
    let solution = Solution {};
    run(&solution);
}

struct Solution {}

impl AocSolution for Solution {
    fn year(&self) -> u32 {
        2021
    }
    fn day(&self) -> u32 {
        16
    }

    fn part_one(&self, input: &str) -> String {
        part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        part_two(input).to_string()
    }
}

pub fn part_one(input: &str) -> u64 {
    let bytes = packet::hex_to_bytes(input.trim());
    let mut cursor = 0;
    let packet = packet::packet_parser(&bytes, &mut cursor);
    let mut sum = 0;
    packet::version_sum(&packet, &mut sum);
    sum
}

pub fn part_two(input: &str) -> u128 {
    let bytes = packet::hex_to_bytes(input.trim());
    let mut cursor = 0;
    let packet = packet::packet_parser(&bytes, &mut cursor);
    packet::resolve(&packet)
}

#[cfg(test)]
mod day16_tests {
    use super::*;

    #[test]
    fn literal_test() {
        // "D2FE28" -> 110100101111111000101000
        let bytes = packet::hex_to_bytes("D2FE28");
        let mut cursor = 0;
        let packet = packet::packet_parser(&bytes, &mut cursor);
        assert_eq!(packet.value, 2021);
    }

    #[test]
    fn length_operator_test() {
        // "38006F45291200" -> 00111000000000000110111101000101001010010001001000000000
        let bytes = packet::hex_to_bytes("38006F45291200");
        let mut cursor = 0;
        let packet = packet::packet_parser(&bytes, &mut cursor);
        assert_eq!(packet.value, 0);
        assert_eq!(packet.sub_packets.len(), 2);
        assert_eq!(packet.sub_packets[0].value, 10);
        assert_eq!(packet.sub_packets[1].value, 20);
    }

    #[test]
    fn count_operator_test() {
        // "EE00D40C823060" -> 11101110000000001101010000001100100000100011000001100000
        let bytes = packet::hex_to_bytes("EE00D40C823060");
        let mut cursor = 0;
        let packet = packet::packet_parser(&bytes, &mut cursor);
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
