use crate::euler::euler_problems;

#[cfg(test)]

#[test]
fn test_problem_1_solver() {
    let attempt = euler_problems::problem_1_solver(vec![3,5], 10);
    assert_eq!(attempt, 23);
}

