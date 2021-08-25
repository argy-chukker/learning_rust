use reduce::Reduce;

use crate::euler::utils;
use std::convert::TryInto;

pub fn problem_1_solver(a : Vec<u32>, b :u32) -> u32 {
    utils::multiples_under(a, b).iter().sum()
}

pub fn problem_2_solver(a :u32) -> u32 {
    let mut answer = 0;

    let mut sequence = utils::FibonacciSequence::new_fibonacci();
    
    let mut fibonacci_number = sequence.next();
    
    loop {
        if fibonacci_number >= a {break};
	if utils::is_even(fibonacci_number) {
	    answer += fibonacci_number;
	}

	fibonacci_number = sequence.next()
    }
    answer
}

pub fn problem_3_solver(n : f64) -> u32 {
    let n_sqrt = n.sqrt() as u32;
    let mut candidates = utils::primes_below(n_sqrt);
    let mut result = 0;
    for c in candidates.iter().rev() {
	if (n % *c as f64) as u32 == 0 {result = *c; break;}
    }
    candidates = utils::primes_below(result);
    for c in candidates.iter().rev() {
	if (n % *c as f64) as u32 == 0 {result = *c; break;}
    }
    result
}

pub fn problem_4_solver(a : usize, b : u32) -> u32 {
    let base = 10_u32.pow(b) - 1;
    let mut candidates = vec![base; a];
    let mut results = Vec::new();
    let mut idx = 0;
    loop {
	let current_candidates = candidates.clone();
	let current_candidate = Reduce::reduce(current_candidates.into_iter(),|x, z| z * x).unwrap();
        let is_palindrome = utils::is_palindrome(current_candidate);
        if is_palindrome {results.push(current_candidate);};
	candidates[idx] -= 1;
        if idx == 0 {
	    idx += 1;
	    for i in 1..a {candidates[i] = base;}
	} else if candidates[idx] == candidates[idx-1] {
	    idx = (idx + 1) % a;
	}
	if candidates[0] < 10_u32.pow(b-1) {break;};
    };
    *results.iter().max().unwrap()
}

pub fn problem_5_solver(n : u32) -> u32 {
    let primes_below = utils::primes_below(n);

    let mut prime_not_factors : Vec<u32> = Vec::new();

    for p in primes_below.into_iter() {
	if n % p != 0 {
	    prime_not_factors.push(p);
	}
    }

    let factor = n * Iterator::reduce(prime_not_factors.into_iter(), |a, b| a * b).unwrap();
    let mut result = factor;

    loop {
	let mut divides_all = true;
	for i in 1..n {
	    divides_all &= result % i == 0;
	}
	if divides_all {break;};
	result += factor;
    };
    result
}

pub fn problem_6_solver(n : u32) -> u32 {
    let mut difference = 0;

    for i in 1..(n+1) {
	for j in 1..(n+1) {
	    if i == j {continue;};
	    difference += i*j;
	}
    }
    difference
}

pub fn problem_7_solver(n : u32) -> u32 {
    utils::nth_prime(n)
}


pub fn problem_8_solver(n : u64, many_digits_n : Option<Vec<u64>>) -> u64 {

    let default_many_digits : Vec<u64>= vec![
7,3,1,6,7,1,7,6,5,3,1,3,3,0,6,2,4,9,1,9,2,2,5,1,1,9,6,7,4,4,2,6,5,7,4,7,4,2,3,5,5,3,4,9,1,9,4,9,3,4,
9,6,9,8,3,5,2,0,3,1,2,7,7,4,5,0,6,3,2,6,2,3,9,5,7,8,3,1,8,0,1,6,9,8,4,8,0,1,8,6,9,4,7,8,8,5,1,8,4,3,
8,5,8,6,1,5,6,0,7,8,9,1,1,2,9,4,9,4,9,5,4,5,9,5,0,1,7,3,7,9,5,8,3,3,1,9,5,2,8,5,3,2,0,8,8,0,5,5,1,1,
1,2,5,4,0,6,9,8,7,4,7,1,5,8,5,2,3,8,6,3,0,5,0,7,1,5,6,9,3,2,9,0,9,6,3,2,9,5,2,2,7,4,4,3,0,4,3,5,5,7,
6,6,8,9,6,6,4,8,9,5,0,4,4,5,2,4,4,5,2,3,1,6,1,7,3,1,8,5,6,4,0,3,0,9,8,7,1,1,1,2,1,7,2,2,3,8,3,1,1,3,
6,2,2,2,9,8,9,3,4,2,3,3,8,0,3,0,8,1,3,5,3,3,6,2,7,6,6,1,4,2,8,2,8,0,6,4,4,4,4,8,6,6,4,5,2,3,8,7,4,9,
3,0,3,5,8,9,0,7,2,9,6,2,9,0,4,9,1,5,6,0,4,4,0,7,7,2,3,9,0,7,1,3,8,1,0,5,1,5,8,5,9,3,0,7,9,6,0,8,6,6,
7,0,1,7,2,4,2,7,1,2,1,8,8,3,9,9,8,7,9,7,9,0,8,7,9,2,2,7,4,9,2,1,9,0,1,6,9,9,7,2,0,8,8,8,0,9,3,7,7,6,
6,5,7,2,7,3,3,3,0,0,1,0,5,3,3,6,7,8,8,1,2,2,0,2,3,5,4,2,1,8,0,9,7,5,1,2,5,4,5,4,0,5,9,4,7,5,2,2,4,3,
5,2,5,8,4,9,0,7,7,1,1,6,7,0,5,5,6,0,1,3,6,0,4,8,3,9,5,8,6,4,4,6,7,0,6,3,2,4,4,1,5,7,2,2,1,5,5,3,9,7,
5,3,6,9,7,8,1,7,9,7,7,8,4,6,1,7,4,0,6,4,9,5,5,1,4,9,2,9,0,8,6,2,5,6,9,3,2,1,9,7,8,4,6,8,6,2,2,4,8,2,
8,3,9,7,2,2,4,1,3,7,5,6,5,7,0,5,6,0,5,7,4,9,0,2,6,1,4,0,7,9,7,2,9,6,8,6,5,2,4,1,4,5,3,5,1,0,0,4,7,4,
8,2,1,6,6,3,7,0,4,8,4,4,0,3,1,9,9,8,9,0,0,0,8,8,9,5,2,4,3,4,5,0,6,5,8,5,4,1,2,2,7,5,8,8,6,6,6,8,8,1,
1,6,4,2,7,1,7,1,4,7,9,9,2,4,4,4,2,9,2,8,2,3,0,8,6,3,4,6,5,6,7,4,8,1,3,9,1,9,1,2,3,1,6,2,8,2,4,5,8,6,
1,7,8,6,6,4,5,8,3,5,9,1,2,4,5,6,6,5,2,9,4,7,6,5,4,5,6,8,2,8,4,8,9,1,2,8,8,3,1,4,2,6,0,7,6,9,0,0,4,2,
2,4,2,1,9,0,2,2,6,7,1,0,5,5,6,2,6,3,2,1,1,1,1,1,0,9,3,7,0,5,4,4,2,1,7,5,0,6,9,4,1,6,5,8,9,6,0,4,0,8,
0,7,1,9,8,4,0,3,8,5,0,9,6,2,4,5,5,4,4,4,3,6,2,9,8,1,2,3,0,9,8,7,8,7,9,9,2,7,2,4,4,2,8,4,9,0,9,1,8,8,
8,4,5,8,0,1,5,6,1,6,6,0,9,7,9,1,9,1,3,3,8,7,5,4,9,9,2,0,0,5,2,4,0,6,3,6,8,9,9,1,2,5,6,0,7,1,7,6,0,6,
0,5,8,8,6,1,1,6,4,6,7,1,0,9,4,0,5,0,7,7,5,4,1,0,0,2,2,5,6,9,8,3,1,5,5,2,0,0,0,5,5,9,3,5,7,2,9,7,2,5,
7,1,6,3,6,2,6,9,5,6,1,8,8,2,6,7,0,4,2,8,2,5,2,4,8,3,6,0,0,8,2,3,2,5,7,5,3,0,4,2,0,7,5,2,9,6,3,4,5,0,
    ];

    let many_digits_n = many_digits_n.unwrap_or(default_many_digits);
    
    let mut result = 0_u64;

    for i in n..many_digits_n.len() as u64 {

	let candidate_vec = many_digits_n[((i-n) as usize)..i as usize].to_vec();
	let candidate = Iterator::reduce(candidate_vec.into_iter(), |a, b| a * b).unwrap();

	if candidate > result {result = candidate;};
    }
    
    result
}

pub fn problem_9_solver() -> u32 {

    let mut b = 1;
    let mut a = 0.5;

    loop {
	a = (1000.0*1000.0 - 2.0*1000.0*b as f32)/(2.0*(1000.0-b as f32));
	if  utils::is_integer(a) {break;};
	b += 1;
    };

    let a = a as u32;
    let c = 1000 - a - b;
    a*b*c
}
