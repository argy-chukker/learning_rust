use std::convert::TryInto;

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
