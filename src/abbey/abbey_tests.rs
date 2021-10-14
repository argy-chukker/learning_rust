use crate::abbey::abbey_problems;

#[cfg(test)]

fn parse_fake_std<T>(fake_std : &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
where T : std::str::FromStr + std::fmt::Debug,
     <T as std::str::FromStr>::Err : std::fmt::Debug
{
    let mut parsed_std = Vec::new();

    let lines = fake_std.split("\n").
	collect::<Vec<&str>>();
    
    for line in lines {

	parsed_std.append(
	    &mut line.
		trim().
		split(" ").collect::<Vec<&str>>().iter().
		map(|d| d.parse::<T>().unwrap()).
		collect::<Vec<T>>()
	)
    }
    Ok(parsed_std)
}

#[test]
fn abbey_test_poblem_1() -> Result<(), String> {
    let fake_std = vec!["3".to_string(), "5".to_string()];
    let attempt = abbey_problems::abbey_problem_1(fake_std);
    match attempt {
	Ok(8) => Ok(()),
	_ => Err(String::from("Test failed"))
    }
}

#[test]
fn abbey_test_poblem_2() -> Result<(), String> {
    let fake_std = parse_fake_std("8
 10 20 30 40 5 6 7 8").unwrap();

    let attempt = abbey_problems::abbey_problem_2(fake_std);
    match attempt {
	Ok(126) => Ok(()),
	_ => Err(String::from("Test failed"))
    }
}

#[test]
fn abbey_test_poblem_3() -> Result<(), String> {
    let fake_std = parse_fake_std("3
100 8
15 245
1945 54").unwrap();

    let attempt = abbey_problems::abbey_problem_3(fake_std);
    let expected = vec![108, 260, 1999]; 
    match attempt {
	Ok(expected) => Ok(()),
	_ => Err(String::from("Test failed"))
    }
}
