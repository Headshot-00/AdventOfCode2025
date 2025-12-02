use advent_of_code_2025::day1::sm::Day1StateMachine;

#[test]
fn test_case_example() {
    let mut machine = Day1StateMachine::new(100);

    assert_eq!(machine.get_state(), 50);
    assert_eq!(machine.get_part_1_counter(), 0);
    assert_eq!(machine.get_zero_counter(), 0);

    machine.update("L68");
    assert_eq!(machine.get_state(), 82);
    assert_eq!(machine.get_part_1_counter(), 0);
    assert_eq!(machine.get_zero_counter(), 1);

    machine.update("L30");
    assert_eq!(machine.get_state(), 52);
    assert_eq!(machine.get_part_1_counter(), 0);
    assert_eq!(machine.get_zero_counter(), 1);

    machine.update("R48");
    assert_eq!(machine.get_state(), 0);
    assert_eq!(machine.get_part_1_counter(), 1);
    assert_eq!(machine.get_zero_counter(), 2);

    machine.update("L5");
    assert_eq!(machine.get_state(), 95);
    assert_eq!(machine.get_part_1_counter(), 1);
    assert_eq!(machine.get_zero_counter(), 2);

    machine.update("R60");
    assert_eq!(machine.get_state(), 55);
    assert_eq!(machine.get_part_1_counter(), 1);
    assert_eq!(machine.get_zero_counter(), 3);

    machine.update("L55");
    assert_eq!(machine.get_state(), 0);
    assert_eq!(machine.get_part_1_counter(), 2);
    assert_eq!(machine.get_zero_counter(), 4);

    machine.update("L1");
    assert_eq!(machine.get_state(), 99);
    assert_eq!(machine.get_part_1_counter(), 2);
    assert_eq!(machine.get_zero_counter(), 4);

    machine.update("L99");
    assert_eq!(machine.get_state(), 0);
    assert_eq!(machine.get_part_1_counter(), 3);
    assert_eq!(machine.get_zero_counter(), 5);

    machine.update("R14");
    assert_eq!(machine.get_state(), 14);
    assert_eq!(machine.get_part_1_counter(), 3);
    assert_eq!(machine.get_zero_counter(), 5);

    machine.update("L82");
    assert_eq!(machine.get_state(), 32);
    assert_eq!(machine.get_part_1_counter(), 3);
    assert_eq!(machine.get_zero_counter(), 6);
}
