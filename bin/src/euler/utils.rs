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
