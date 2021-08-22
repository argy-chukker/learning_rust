use crate::euler::euler_problems;

#[cfg(test)]

#[test]
fn test_problem_1_solver() {
    let attempt = euler_problems::problem_1_solver(vec![3,5], 10);
    assert_eq!(attempt, 23);
}

#[test]
fn test_problem_3_solver() {
    let attempt = euler_problems::problem_3_solver(13195_f64);
    assert_eq!(attempt, 29);
}

#[test]
fn test_problem_4_solver() {
    let attempt = euler_problems::problem_4_solver(2, 2);
    assert_eq!(attempt, 9009);
}

