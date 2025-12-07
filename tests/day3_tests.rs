use advent_of_code_2025::day3::accumulator::Day3Accumulator;

#[test]
fn test_case_example() {
    let mut acc = Day3Accumulator::new();
    let mut total_joltage_part1: u64 = 0;
    let mut total_joltage_part2: u64 = 0;

    acc.update("987654321111111").unwrap();
    total_joltage_part1 += 98;
    assert_eq!(acc.get_total_joltage_part1(), total_joltage_part1);
    total_joltage_part2 += 987654321111;
    assert_eq!(acc.get_total_joltage_part2(), total_joltage_part2);
    acc.update("811111111111119").unwrap();
    total_joltage_part1 += 89;
    assert_eq!(acc.get_total_joltage_part1(), total_joltage_part1);
    total_joltage_part2 += 811111111119;
    assert_eq!(acc.get_total_joltage_part2(), total_joltage_part2);
    acc.update("234234234234278").unwrap();
    total_joltage_part1 += 78;
    assert_eq!(acc.get_total_joltage_part1(), total_joltage_part1);
    total_joltage_part2 += 434234234278;
    assert_eq!(acc.get_total_joltage_part2(), total_joltage_part2);
    acc.update("818181911112111").unwrap();
    total_joltage_part1 += 92;
    assert_eq!(acc.get_total_joltage_part1(), total_joltage_part1);
    total_joltage_part2 += 888911112111;
    assert_eq!(acc.get_total_joltage_part2(), total_joltage_part2);
}
