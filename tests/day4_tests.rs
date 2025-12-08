use advent_of_code_2025::day4::solver::Day4Solver;

#[test]
fn test_case_example() {
    let mut solver = Day4Solver::default();
    let input = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    for line in input.lines() {
        solver.add_row(&line).expect("Failed to add row");
    }
    solver.finalize_input();
    solver.init_gpu().unwrap();

    assert_eq!(solver.solve().expect("Solver failed"), 13);
}
