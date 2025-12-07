use advent_of_code_2025::day2::{UpdateError, accumulator::Day2Accumulator};

#[test]
fn test_basic_range() {
    let mut acc = Day2Accumulator::new();

    acc.update("11-22").unwrap(); // simple doubled halves in range
    // 11 and 22 are both valid doubled numbers
    assert_eq!(acc.get_sum_part1(), 11 + 22);
    assert_eq!(acc.get_sum_part2(), 11 + 22);
}

#[test]
fn test_empty_input() {
    let mut acc = Day2Accumulator::new();
    let err = acc.update("").unwrap_err();
    assert_eq!(err, UpdateError::EmptyInput);
    assert_eq!(acc.get_sum_part1(), 0);
    assert_eq!(acc.get_sum_part2(), 0);
}

#[test]
fn test_malformed_input() {
    let mut acc = Day2Accumulator::new();
    let err = acc.update("1234").unwrap_err(); // missing '-'
    assert_eq!(err, UpdateError::InvalidInput);

    let err = acc.update("12-ab").unwrap_err(); // second substring is not a number
    assert_eq!(err, UpdateError::InvalidInput);
}

#[test]
fn test_reversed_range() {
    let mut acc = Day2Accumulator::new();
    let err = acc.update("100-10").unwrap_err(); // permissive version
    assert_eq!(err, UpdateError::ReversedRange);
    assert_eq!(acc.get_sum_part1(), 0);
    assert_eq!(acc.get_sum_part2(), 0);
}

#[test]
fn test_multi_digit_range() {
    let mut acc = Day2Accumulator::new();
    acc.update("1000-1015").unwrap();
    // Only 1010 is a valid doubled number
    assert_eq!(acc.get_sum_part1(), 1010);
    assert_eq!(acc.get_sum_part2(), 1010);
}

#[test]
fn test_multiple_updates() {
    let mut acc = Day2Accumulator::new();
    acc.update("11-22").unwrap();
    acc.update("33-44").unwrap();
    // sum of all doubled numbers across both updates
    assert_eq!(acc.get_sum_part1(), 11 + 22 + 33 + 44);
    assert_eq!(acc.get_sum_part2(), 11 + 22 + 33 + 44);
}

#[test]
fn test_example() {
    // Go through the example from the website
    let mut acc = Day2Accumulator::new();
    acc.update("11-22").unwrap();
    assert_eq!(acc.get_sum_part1(), 11 + 22);
    acc.update("95-115").unwrap();
    assert_eq!(acc.get_sum_part1(), 11 + 22 + 99);
    assert_eq!(acc.get_sum_part2(), 11 + 22 + 99 + 111);
    acc.update("998-1012").unwrap();
    assert_eq!(acc.get_sum_part1(), 11 + 22 + 99 + 1010);
    assert_eq!(acc.get_sum_part2(), 11 + 22 + 99 + 111 + 999 + 1010);
    acc.update("1188511880-1188511890").unwrap();
    assert_eq!(acc.get_sum_part1(), 11 + 22 + 99 + 1010 + 1188511885);
    assert_eq!(
        acc.get_sum_part2(),
        11 + 22 + 99 + 111 + 999 + 1010 + 1188511885
    );
    acc.update("222220-222224").unwrap();
    assert_eq!(
        acc.get_sum_part1(),
        11 + 22 + 99 + 1010 + 1188511885 + 222222
    );
    assert_eq!(
        acc.get_sum_part2(),
        11 + 22 + 99 + 111 + 999 + 1010 + 1188511885 + 222222
    );
    acc.update("1698522-1698528").unwrap();
    assert_eq!(
        acc.get_sum_part1(),
        11 + 22 + 99 + 1010 + 1188511885 + 222222
    );
    assert_eq!(
        acc.get_sum_part2(),
        11 + 22 + 99 + 111 + 999 + 1010 + 1188511885 + 222222
    );
    acc.update("446443-446449").unwrap();
    assert_eq!(
        acc.get_sum_part1(),
        11 + 22 + 99 + 1010 + 1188511885 + 222222 + 446446
    );
    assert_eq!(
        acc.get_sum_part2(),
        11 + 22 + 99 + 111 + 999 + 1010 + 1188511885 + 222222 + 446446
    );
    acc.update("38593856-38593862").unwrap();
    assert_eq!(
        acc.get_sum_part1(),
        11 + 22 + 99 + 1010 + 1188511885 + 222222 + 446446 + 38593859
    );
    assert_eq!(
        acc.get_sum_part2(),
        11 + 22 + 99 + 111 + 999 + 1010 + 1188511885 + 222222 + 446446 + 38593859
    );
    acc.update("565653-565659").unwrap();
    assert_eq!(
        acc.get_sum_part1(),
        11 + 22 + 99 + 1010 + 1188511885 + 222222 + 446446 + 38593859
    );
    assert_eq!(
        acc.get_sum_part2(),
        11 + 22 + 99 + 111 + 999 + 1010 + 1188511885 + 222222 + 446446 + 38593859 + 565656
    );
    acc.update("824824821-824824827").unwrap();
    assert_eq!(
        acc.get_sum_part1(),
        11 + 22 + 99 + 1010 + 1188511885 + 222222 + 446446 + 38593859
    );
    assert_eq!(
        acc.get_sum_part2(),
        11 + 22
            + 99
            + 111
            + 999
            + 1010
            + 1188511885
            + 222222
            + 446446
            + 38593859
            + 565656
            + 824824824
    );
    acc.update("2121212118-2121212124").unwrap();
    assert_eq!(
        acc.get_sum_part1(),
        11 + 22 + 99 + 1010 + 1188511885 + 222222 + 446446 + 38593859
    );
    assert_eq!(
        acc.get_sum_part2(),
        11 + 22
            + 99
            + 111
            + 999
            + 1010
            + 1188511885
            + 222222
            + 446446
            + 38593859
            + 565656
            + 824824824
            + 2121212121
    );
}
