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

#[test]
fn test_problem_5_solver() {
    let attempt = euler_problems::problem_5_solver(10);
    assert_eq!(attempt, 2520);
}

#[test]
fn test_problem_6_solver() {
    let attempt = euler_problems::problem_6_solver(10);
    assert_eq!(attempt, 2640);
}

#[test]
fn test_problem_7_solver() {
    let attempt = euler_problems::problem_7_solver(6);
    assert_eq!(attempt, 13);
}

#[test]
fn test_problem_8_solver() {
    let attempt = euler_problems::problem_8_solver(4, None);
    assert_eq!(attempt, 5832);
}

#[test]
fn test_problem_10_solver() {
    let attempt = euler_problems::problem_10_solver(10);
    assert_eq!(attempt, 17);
}

#[test]
fn test_problem_12_solver() {
    let attempt = euler_problems::problem_12_solver(5);
    assert_eq!(attempt, 28);
}

#[test]
fn test_problem_15_solver() {
    let attempt = euler_problems::problem_15_solver(2, 2);
    assert_eq!(attempt, 6);
}

#[test]
fn test_problem_16_solver() {
    let attempt = euler_problems::problem_16_solver(15);
    assert_eq!(attempt, 26);
}

#[test]
fn test_problem_17_solver() {
    let attempt = euler_problems::problem_17_solver(5);
    assert_eq!(attempt, 19);
}

#[test]
fn test_problem_18_solver() {
    let attempt = euler_problems::problem_18_solver(
	Some(
	    vec![
		vec![3],
		vec![7,4],
		vec![2,4,6],
		vec![8,5,9,3]
	    ]
	)
    );
    assert_eq!(attempt, 23);
}

