use crate::euler::euler_problems;

#[cfg(test)]
#[test]
fn hint_test_problem_1_solver() {
    let attempt = euler_problems::problem_1_solver(vec![3, 5], 10);
    assert_eq!(attempt, 23);
}

#[test]
fn hint_test_problem_3_solver() {
    let attempt = euler_problems::problem_3_solver(13195_f64);
    assert_eq!(attempt, 29);
}

#[test]
fn hint_test_problem_4_solver() {
    let attempt = euler_problems::problem_4_solver(2, 2);
    assert_eq!(attempt, 9009);
}

#[test]
fn hint_test_problem_5_solver() {
    let attempt = euler_problems::problem_5_solver(10);
    assert_eq!(attempt, 2520);
}

#[test]
fn hint_test_problem_6_solver() {
    let attempt = euler_problems::problem_6_solver(10);
    assert_eq!(attempt, 2640);
}

#[test]
fn hint_test_problem_7_solver() {
    let attempt = euler_problems::problem_7_solver(6);
    assert_eq!(attempt, 13);
}

#[test]
fn hint_test_problem_8_solver() {
    let attempt = euler_problems::problem_8_solver(4, None);
    assert_eq!(attempt, 5832);
}

#[test]
fn hint_test_problem_10_solver() {
    let attempt = euler_problems::problem_10_solver(10);
    assert_eq!(attempt, 17);
}

#[test]
fn hint_test_problem_12_solver() {
    let attempt = euler_problems::problem_12_solver(5);
    assert_eq!(attempt, 28);
}

#[test]
fn hint_test_problem_15_solver() {
    let attempt = euler_problems::problem_15_solver(2, 2);
    assert_eq!(attempt, 6);
}

#[test]
fn hint_test_problem_16_solver() {
    let attempt = euler_problems::problem_16_solver(15);
    assert_eq!(attempt, 26);
}

#[test]
fn hint_test_problem_17_solver() {
    let attempt = euler_problems::problem_17_solver(5);
    assert_eq!(attempt, 19);
}

#[test]
fn hint_test_problem_18_solver() {
    let attempt = euler_problems::problem_18_solver(Some(vec![
        vec![3],
        vec![7, 4],
        vec![2, 4, 6],
        vec![8, 5, 9, 3],
    ]));
    assert_eq!(attempt, 23);
}

#[test]
fn hint_test_problem_20_solver() {
    let attempt = euler_problems::problem_20_solver(10);
    assert_eq!(attempt, 27);
}

#[test]
fn hint_test_problem_25_solver() {
    let attempt = euler_problems::problem_25_solver(2);
    assert_eq!(attempt, 12);
}

#[test]
fn hint_test_problem_26_solver() {
    let attempt = euler_problems::problem_26_solver(10_u32);
    assert_eq!(attempt, 7);
}

#[test]
fn hint_test_problem_28_solver() {
    let attempt = euler_problems::problem_28_solver::<u32>(5);
    assert_eq!(attempt, 101);
}

#[test]
fn hint_test_problem_29_solver() {
    let attempt = euler_problems::problem_29_solver(5, 5);
    assert_eq!(attempt, 15);
}

#[test]
fn hint_test_problem_30_solver() {
    let attempt = euler_problems::problem_30_solver(4);
    assert_eq!(attempt, 19316);
}

#[test]
fn hint_test_problem_35_solver() {
    let attempt = euler_problems::problem_35_solver(100);
    assert_eq!(attempt, 13);
}

#[test]
fn hint_test_problem_40_solver() {
    let attempt = euler_problems::problem_40_solver(vec![12]);
    assert_eq!(attempt, 1);
}

#[test]
fn hint_test_problem_47_solver() {
    let attempt = euler_problems::problem_47_solver(2);
    assert_eq!(attempt, 14);
    let attempt = euler_problems::problem_47_solver(3);
    assert_eq!(attempt, 644);
}

#[test]
fn hint_test_problem_48_solver() {
    let attempt = euler_problems::problem_48_solver(10);
    assert_eq!(attempt, 405071317);
}

#[test]
fn hint_test_problem_50_solver() {
    let attempt = euler_problems::problem_50_solver(100);
    assert_eq!(attempt, 41);

    let attempt = euler_problems::problem_50_solver(1000);
    assert_eq!(attempt, 953);
}

#[test]
fn hint_test_problem_51_solver() {
    let attempt = euler_problems::problem_51_solver(7);
    assert_eq!(attempt, 56003);
}

#[test]
fn hint_test_problem_54_solver() -> Result<(), String> {
    let attempt = euler_problems::problem_54_solver(true);
    match attempt {
        Ok(3) => Ok(()),
        _ => Err(String::from("Test failed")),
    }
}

#[test]
fn hint_test_problem_57_solver() {
    let attempt = euler_problems::problem_57_solver(8);
    assert_eq!(attempt, 1);
}

#[test]
fn hint_test_problem_65_solver() {
    let attempt = euler_problems::problem_65_solver(10);
    assert_eq!(attempt, 17);
}

#[test]
fn hint_test_problem_69_solver() {
    let attempt = euler_problems::problem_69_solver(10);
    assert_eq!(attempt, 6);
}

#[test]
fn hint_test_problem_71_solver() {
    let attempt = euler_problems::problem_71_solver(8);
    assert_eq!(attempt, 2);
}

#[test]
fn hint_test_problem_72_solver() {
    let attempt = euler_problems::problem_72_solver(8);
    assert_eq!(attempt, 21);
}

#[test]
fn hint_test_problem_73_solver() {
    let attempt = euler_problems::problem_73_solver(8);
    assert_eq!(attempt, 3);
}

#[test]
fn hint_test_problem_78_solver() {
    let attempt = euler_problems::problem_78_solver(7);
    assert_eq!(attempt, 5);
}

#[test]
fn hint_test_problem_100_solver() {
    let attempt = euler_problems::problem_100_solver(20);
    assert_eq!(attempt, 15);

    let attempt = euler_problems::problem_100_solver(22);
    assert_eq!(attempt, 85);
}
