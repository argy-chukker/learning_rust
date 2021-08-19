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
