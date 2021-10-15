pub fn apply_to_test_cases<T, S, W> (
    logic : T, std_in : Vec<String>, problem_n : u32)
      -> Result<Vec<W>, Box<dyn std::error::Error>>
where T : Fn(S, S) -> W, S : std::str::FromStr, W : std::fmt::Display,
      <S as std::str::FromStr>::Err : 'static + std::error::Error
{
    let n = std_in[0].parse::<usize>()?;
    let mut results = Vec::new();

    for i in 0..n {
	results.push(
	    logic(std_in[1 + 2*i].parse::<S>()?, std_in[2 + 2*i].parse::<S>()?)
	);
    };

    let print_str = Iterator::reduce(
	results.iter().map(|num| num.to_string()),
	|s, t| s + " " + &t
    ).ok_or("")?;

    println!("Result for problem {}: {}", problem_n, print_str);

    Ok(results)
}
