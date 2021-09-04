use crate::euler::euler_problems;

#[cfg(test)]

#[test]
fn answer_test_problem_1_solver() {
    let attempt = euler_problems::problem_1_solver(vec![3,5], 1000);
    assert_eq!(attempt, 233168);
}

#[test]
fn answer_test_problem_2_solver() {
    let attempt = euler_problems::problem_2_solver(4000000);
    assert_eq!(attempt, 4613732);
}

#[test]
fn answer_test_problem_3_solver() {
    let attempt = euler_problems::problem_3_solver(600851475143_f64);
    assert_eq!(attempt, 6857);
}

#[test]
fn answer_test_problem_4_solver() {
    let attempt = euler_problems::problem_4_solver(2, 3);
    assert_eq!(attempt, 906609);
}

#[test]
fn answer_test_problem_5_solver() {
    let attempt = euler_problems::problem_5_solver(20);
    assert_eq!(attempt, 232792560);
}

#[test]
fn answer_test_problem_6_solver() {
    let attempt = euler_problems::problem_6_solver(100);
    assert_eq!(attempt, 25164150);
}

#[test]
fn answer_test_problem_7_solver() {
    let attempt = euler_problems::problem_7_solver(10001);
    assert_eq!(attempt, 104743);
}

#[test]
fn answer_test_problem_8_solver() {
    let attempt = euler_problems::problem_8_solver(13, None);
    assert_eq!(attempt, 23514624000);
}

#[test]
fn answer_test_problem_9_solver() {
    let attempt = euler_problems::problem_9_solver();
    assert_eq!(attempt, 31875000);
}

#[test]
fn answer_test_problem_10_solver() {
    let attempt = euler_problems::problem_10_solver(2000000);
    assert_eq!(attempt, 142913828922);
}

#[test]
fn answer_test_problem_11_solver() {
    let attempt = euler_problems::problem_11_solver(4, None);
    assert_eq!(attempt, 70600674);
}

#[test]
fn answer_test_problem_12_solver() {
    let attempt = euler_problems::problem_12_solver(500);
    assert_eq!(attempt, 76576500);
}

#[test]
fn answer_test_problem_13_solver() {
    let attempt = euler_problems::problem_13_solver(None, 10);
    assert_eq!(attempt, 5537376230);
}

#[test]
fn answer_test_problem_14_solver() {
    let attempt = euler_problems::problem_14_solver(1000000);
    assert_eq!(attempt, 837799);
}

#[test]
fn answer_test_problem_15_solver() {
    let attempt = euler_problems::problem_15_solver(20,20);
    assert_eq!(attempt, 137846528820);
}

#[test]
fn answer_test_problem_16_solver() {
    let attempt = euler_problems::problem_16_solver(1000);
    assert_eq!(attempt, 1366);
}

#[test]
fn answer_test_problem_17_solver() {
    let attempt = euler_problems::problem_17_solver(1000);
    assert_eq!(attempt, 21124);
}

#[test]
fn answer_test_problem_18_solver() {
    let attempt = euler_problems::problem_18_solver(None);
    assert_eq!(attempt, 1074);
}

#[test]
fn answer_test_problem_19_solver() {
    let attempt = euler_problems::problem_19_solver(1901, 2000, None, None);
    assert_eq!(attempt, 171);
}

#[test]
fn answer_test_problem_20_solver() {
    let attempt = euler_problems::problem_20_solver(100);
    assert_eq!(attempt, 648);
}

#[test]
fn answer_test_problem_21_solver() {
    let attempt = euler_problems::problem_21_solver(10000);
    assert_eq!(attempt, 31626);
}

#[test]
fn answer_test_problem_23_solver() {
    let attempt = euler_problems::problem_23_solver();
    assert_eq!(attempt, 4179871);
}

#[test]
fn answer_test_problem_25_solver() {
    let attempt = euler_problems::problem_25_solver(999);
    assert_eq!(attempt, 4782);
}

#[test]
fn answer_test_problem_26_solver() {
    let attempt = euler_problems::problem_26_solver(1000_u32);
    assert_eq!(attempt, 983);
}

#[test]
fn answer_test_problem_27_solver() {
    let attempt = euler_problems::problem_27_solver(1000_u32,1000_u32,None);
    assert_eq!(attempt, -59231);
}

#[test]
fn answer_test_problem_28_solver() {
    let attempt = euler_problems::problem_28_solver::<u32>(1001);
    assert_eq!(attempt, 669171001);
}

#[test]
fn answer_test_problem_67_solver() {
    let attempt = euler_problems::problem_67_solver(None);
    assert_eq!(attempt, 7273);
}
