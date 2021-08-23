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
