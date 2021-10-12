use crate::abbey::abbey_problems;

#[cfg(test)]

#[test]
fn abbey_test_poblem_1() -> Result<(), String> {
    let fake_std = vec!["3".to_string(), "5".to_string()];
    let attempt = abbey_problems::abbey_problem_1(fake_std);
    match attempt {
	Ok(8) => Ok(()),
	_ => Err(String::from("Test failed"))
    }
}
