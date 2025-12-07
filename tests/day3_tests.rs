use advent_of_code_2025::day3::accumulator::Day3Accumulator;

#[test]
fn test_case_example() {
    let mut acc = Day3Accumulator::new();

    acc.update("987654321111111").unwrap();
    assert_eq!(acc.get_total_joltage(), 98);
    acc.update("811111111111119").unwrap();
    assert_eq!(acc.get_total_joltage(), 98 + 89);
    acc.update("234234234234278").unwrap();
    assert_eq!(acc.get_total_joltage(), 98 + 89 + 78);
    acc.update("818181911112111").unwrap();
    assert_eq!(acc.get_total_joltage(), 98 + 89 + 78 + 92);
}
