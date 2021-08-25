use std::convert::TryInto;
use itertools::Itertools;

pub fn multiples_under(multiples : Vec<u32>, upto : u32) -> Vec<u32> {

    let mut answer : Vec<u32> = vec![];
    
    for candidate in 1..upto {
	for target in &multiples {
	    if is_multiple(candidate, *target) {
		answer.push(candidate);
		break;
	    }
	}
    }
    answer
}

fn is_multiple(number : u32, base : u32) -> bool {
    number % base == 0
}

pub fn is_even(number : u32) -> bool {
    is_multiple(number, 2)
}

pub struct FibonacciSequence {
    last_two : [u32; 2],
}

impl FibonacciSequence {

    pub fn new_fibonacci() -> FibonacciSequence {
	FibonacciSequence{last_two : [0,1]}
    }
    
    pub fn next(&mut self) -> u32 {
	let next = self.last_two.iter().sum();

	self.last_two[1] = self.last_two[0];
	self.last_two[0] = next;
	
	next
    }
}

pub struct TriangularSequence {
    last : u32,
    cumsum : u32,
}

impl TriangularSequence {

    pub fn new_triangular() -> TriangularSequence {
	TriangularSequence{last : 0, cumsum : 0}
    }
    
    pub fn next(&mut self) -> u32 {
	self.last += 1;
	self.cumsum += self.last;

	self.cumsum
    }
}

pub fn divisors(n : u64) -> Vec<u64> {
    let top = (n as f64 / 2.0).ceil() as u64;

    let mut divisors : Vec<u64> = Vec::new();

    for i in 1..top {
	if n % i == 0 {
	    divisors.push(i);
	    divisors.push(n / i);
	};
    };
    divisors.into_iter().unique().collect()
}


pub fn primes_below(n : u32) -> Vec<u32> {
    let top = (n as f64).sqrt() as usize + 1;
    
    let mut candidates : Vec<u32> = (2..n+1).collect();

    for i in 2..top {
	if candidates[i-2] > 0 {
	    let max_j = n as usize / i +1 ;
	    for j in i..max_j {
	 	candidates[i*j-2] = 0;
	     }
	 }
    }

    let mut result : Vec<u32> = Vec::new();
    for c in candidates {
	if c > 0 {result.push(c)}
    }
    result
}

pub fn factorize(n : u32) -> Vec<u32> {
    let candidates = primes_below(n);
    let mut result : Vec<u32> = Vec::new();

    let mut rest = n;
    
    for c in &candidates {
	if rest % c == 0
	{
	    result.push(*c);
	    while rest % c == 0 {rest /= c;};
	};
    };
    if result == [] {result.push(rest);};
    result
}

pub fn is_palindrome(n : u32) -> bool {
    let ciphres = (n as f64).log10() as u32 + 1;

    let mut  result = true;
    for i in 0..(ciphres/2) {
	let l_mod = 10_u32.pow(i + 1) as u32;
	let u_mod = 10_u32.pow(ciphres - i) as u32;
        let r_side = (n % l_mod) / (l_mod / 10);
  	let l_side =  (n % u_mod) / (u_mod / 10);
	if l_side != r_side {
	    result = false;
	    break;};
    };
    
    result
}

pub fn nth_prime(n: u32) -> u32 {
    if n <= 6 {
    return [2 , 3, 5, 7, 11, 13][n as usize-1];
};
    let n_float = n as f64;
    let bound = (n_float * (n_float.ln() + n_float.ln().ln())) as u32;

    let candidates = primes_below(bound);

    candidates[n as usize - 1]
}

pub fn is_integer(n : f32) -> bool {
    n.floor() == n
}
