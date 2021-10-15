pub fn apply_to_test_cases<T, S, W> (
    logic : T,
    std_in : Vec<String>,
    test_size : usize,
    problem_n : u32)
      -> Result<Vec<W>, Box<dyn std::error::Error>>
where T : Fn(Vec<S>) -> W, S : std::str::FromStr + std::clone::Clone, W : std::fmt::Display,
      <S as std::str::FromStr>::Err : 'static + std::error::Error
{
    let n = std_in[0].parse::<usize>()?;
    let mut results = Vec::new();

    let parsed_input : Vec<S> = std_in.iter().map(|i| i.parse::<S>().unwrap()).collect();
    
    for i in 0..n {
	results.push(
	    logic(parsed_input[1+i*test_size..1+(i+1)*test_size].to_vec())
	);
    };

    let print_str = Iterator::reduce(
	results.iter().map(|num| num.to_string()),
	|s, t| s + " " + &t
    ).ok_or("")?;

    println!("Result for problem {}: {}", problem_n, print_str);

    Ok(results)
}
