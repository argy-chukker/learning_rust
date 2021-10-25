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

pub fn digits<T>(n : T) -> Result<Vec<T>, Box<dyn std::error::Error>> 
where T : num::NumCast + num::Unsigned {
    let n_128 : u128 = num::NumCast::from(n).ok_or("Incompatible n")?;

    let digits_n : u128 = (n_128 as f64).log10() as u128;

    let mut digits : Vec<T> = Vec::new();

    let mut div = 1;
    for _i in 0..=digits_n {
	let new_digit = (n_128 / div) % 10;
	let new_digit = num::NumCast::from(new_digit).ok_or("Problem calculating")?;
	digits.push(new_digit);
	div *= 10;
    };
    Ok(digits)
}
