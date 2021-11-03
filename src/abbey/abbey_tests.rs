use crate::abbey::abbey_problems;

#[cfg(test)]

fn parse_fake_std<T>(fake_std: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: std::str::FromStr + std::fmt::Debug,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut parsed_std = Vec::new();

    let lines = fake_std.split("\n").collect::<Vec<&str>>();

    for line in lines {
        parsed_std.append(
            &mut line
                .trim()
                .split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|d| d.parse::<T>().unwrap())
                .collect::<Vec<T>>(),
        )
    }
    Ok(parsed_std)
}

#[test]
fn abbey_test_problem_1() -> Result<(), String> {
    let fake_std = vec!["3".to_string(), "5".to_string()];
    let attempt = abbey_problems::abbey_problem_1(fake_std);
    match attempt {
        Ok(8) => Ok(()),
        _ => Err(String::from("Test failed")),
    }
}

#[test]
fn abbey_test_problem_2() -> Result<(), String> {
    let fake_std = parse_fake_std(
        "8
 10 20 30 40 5 6 7 8",
    )
    .unwrap();

    let attempt = abbey_problems::abbey_problem_2(fake_std);
    match attempt {
        Ok(126) => Ok(()),
        _ => Err(String::from("Test failed")),
    }
}

#[test]
fn abbey_test_problem_3() -> Result<(), String> {
    let fake_std = parse_fake_std(
        "3
100 8
15 245
1945 54",
    )
    .unwrap();

    let attempt = abbey_problems::abbey_problem_3(fake_std);
    let expected = vec![108, 260, 1999];
    match attempt {
        Ok(out) => {
            if out == expected {
                Ok(())
            } else {
                Err(String::from("Test failed"))
            }
        }
        _ => Err(String::from("Test failed")),
    }
}

#[test]
fn abbey_test_problem_4() -> Result<(), String> {
    let fake_std = parse_fake_std(
        "3
5 3
2 8
100 15",
    )
    .unwrap();

    let attempt = abbey_problems::abbey_problem_4(fake_std);
    let expected = vec![3, 2, 15];
    match attempt {
        Ok(out) => {
            if out == expected {
                Ok(())
            } else {
                Err(String::from("Test failed"))
            }
        }
        _ => Err(String::from("Test failed")),
    }
}

#[test]
fn abbey_test_problem_5() -> Result<(), String> {
    let fake_std = parse_fake_std(
        "3
7 3 5
15 20 40
300 550 137",
    )
    .unwrap();

    let attempt = abbey_problems::abbey_problem_5(fake_std);
    let expected = vec![3, 15, 137];
    match attempt {
        Ok(out) => {
            if out == expected {
                Ok(())
            } else {
                Err(String::from("Test failed"))
            }
        }
        _ => Err(String::from("Test failed")),
    }
}

#[test]
fn abbey_test_problem_15() -> Result<(), String> {
    let fake_std = parse_fake_std("1 3 5 7 9 11 295 297 299 300 298 296 12 10 8 6 4 2").unwrap();

    let attempt = abbey_problems::abbey_problem_15(fake_std);
    let expected = (300, 1);
    match attempt {
        Ok(out) => {
            if out == expected {
                Ok(())
            } else {
                Err(String::from("Test failed"))
            }
        }
        _ => Err(String::from("Test failed")),
    }
}

#[test]
fn abbey_test_problem_6() -> Result<(), String> {
    let fake_std = parse_fake_std(
        "3
12 8
11 -3
400 5",
    )
    .unwrap();

    let attempt = abbey_problems::abbey_problem_6(fake_std);
    let expected = vec![2, -4, 80];
    match attempt {
        Ok(out) => {
            if out == expected {
                Ok(())
            } else {
                Err(String::from("Test failed"))
            }
        }
        _ => Err(String::from("Test failed")),
    }
}

#[test]
fn abbey_test_problem_7() -> Result<(), String> {
    let fake_std = parse_fake_std("5 495 353 168 -39 22").unwrap();

    let attempt = abbey_problems::abbey_problem_7(fake_std);
    let expected = vec![257, 178, 76, -39, -6];
    match attempt {
        Ok(out) => {
            if out == expected {
                Ok(())
            } else {
                Err(String::from("Test failed"))
            }
        }
        _ => Err(String::from("Test failed")),
    }
}

#[test]
fn abbey_test_problem_8() -> Result<(), String> {
    let fake_std = parse_fake_std(
        "2
5 2 3
3 0 10",
    )
    .unwrap();

    let attempt = abbey_problems::abbey_problem_8(fake_std);
    let expected = vec![21, 30];
    match attempt {
        Ok(out) => {
            if out == expected {
                Ok(())
            } else {
                Err(String::from("Test failed"))
            }
        }
        _ => Err(String::from("Test failed")),
    }
}

#[test]
fn abbey_test_problem_9() -> Result<(), String> {
    let fake_std = parse_fake_std(
        "2
3 4 5
1 2 4",
    )
    .unwrap();

    let attempt = abbey_problems::abbey_problem_9(fake_std);
    let expected = vec![1, 0];
    match attempt {
        Ok(out) => {
            if out == expected {
                Ok(())
            } else {
                Err(String::from("Test failed"))
            }
        }
        _ => Err(String::from("Test failed")),
    }
}

#[test]
fn abbey_test_problem_11() -> Result<(), String> {
    let fake_std = parse_fake_std(
        "3
11 9 1
14 90 232
111 15 111",
    )
    .unwrap();

    let attempt = abbey_problems::abbey_problem_11(fake_std);
    let expected = vec![1, 16, 21];
    match attempt {
        Ok(out) => {
            if out == expected {
                Ok(())
            } else {
                Err(String::from("Test failed"))
            }
        }
        _ => Err(String::from("Test failed")),
    }
}

#[test]
fn abbey_test_problem_13() -> Result<(), String> {
    let fake_std = parse_fake_std(
        "3
9 15 1776",
    )
    .unwrap();

    let attempt = abbey_problems::abbey_problem_13(fake_std);
    let expected = vec![9, 11, 60];
    match attempt {
        Ok(out) => {
            if out == expected {
                Ok(())
            } else {
                Err(String::from("Test failed"))
            }
        }
        _ => Err(String::from("Test failed")),
    }
}

#[test]
fn abbey_test_problem_20() -> Result<(), String> {
    let fake_std = parse_fake_std(
        "4
abracadabra/
pear tree/
o a kak ushakov lil vo kashu kakao/
my pyx/",
    )
    .unwrap();

    let attempt = abbey_problems::abbey_problem_20(fake_std);
    let expected = vec![5, 4, 13, 2];
    match attempt {
        Ok(out) => {
            if out == expected {
                Ok(())
            } else {
                Err(String::from("Test failed"))
            }
        }
        _ => Err(String::from("Test failed")),
    }
}
