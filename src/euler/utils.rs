use num_bigint::{BigUint, ToBigUint};
use num::{NumCast, traits::Unsigned};
use itertools::Itertools;
use std::collections::HashMap;

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
    last_two : [u128; 2],
}

impl FibonacciSequence {

    pub fn new_fibonacci() -> FibonacciSequence {
	FibonacciSequence{last_two : [0,1]}
    }
    
    pub fn next(&mut self) -> u128 {
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
    if n == 1 {
	vec![1]
    } else if n == 2 {
	vec![1,2]
    } else {
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
}


pub fn primes_below<T : Unsigned + NumCast + Copy + std::cmp::PartialOrd>(n : T) -> Vec<T> {

    let top : f64 = NumCast::from(n).unwrap();
    let top : usize = top.sqrt() as usize + 1;

    let max_candidate : u128 = NumCast::from(n).unwrap();
    let candidates_u128 : Vec<u128> = (2_u128..max_candidate+1_u128).collect();
    let mut candidates : Vec<T> = Vec::new();

    for c in candidates_u128 {
	candidates.push(NumCast::from(c).unwrap())
    }
    
    for i in 2..top {
	if candidates[i-2] > NumCast::from(0).unwrap() {
	    let mut max_j : usize = NumCast::from(n).unwrap();
	    max_j /= i;
	    max_j += 1 ;
	    for j in i..max_j {
	 	candidates[i*j-2] = NumCast::from(0).unwrap();
	     }
	 }
    }

    let mut result : Vec<T> = Vec::new();
    for c in candidates {
	if c > NumCast::from(0).unwrap() {result.push(c)}
    }
    result
}

pub fn factorize<T : Unsigned + NumCast + Copy + std::cmp::PartialOrd>(n : T) -> Vec<T> {
    let candidates = primes_below(n);
    let mut result : Vec<T> = Vec::new();

    let mut rest = n;
    
    for c in &candidates {
	if rest % *c == NumCast::from(0).unwrap()
	{
	    result.push(*c);
	    while rest % *c == NumCast::from(0).unwrap() {rest = rest /  *c;};
	};
    };
    if result == [] {result.push(rest);};
    result
}

pub fn is_prime<T : Unsigned + NumCast + Copy + std::cmp::PartialOrd>(n : T) -> bool {
    n == factorize(n)[0]
}

pub fn simplify_fraction<T : Unsigned + NumCast + Copy + std::cmp::PartialOrd + std::fmt::Debug>(num : T, dem : T) -> (T, T) {

    let d_factors : Vec<T> = factorize(NumCast::from(dem).unwrap());
    let n_factors : Vec<T> = factorize(NumCast::from(num).unwrap());


    let res_num : T;
    let  res_den : T;
    
    if d_factors == n_factors 	{
	if num > dem {
	    res_num = num / dem;
	    res_den = NumCast::from(1).unwrap();
	} else {
	    res_den = dem / num;
	    res_num = NumCast::from(1).unwrap();
	}; 


    } else {
        let mut new_den = d_factors.clone();
        let mut new_num = n_factors.clone();
	new_den.retain(|f| ! n_factors.contains(&f));
	res_den = Iterator::reduce(new_den.into_iter(), |x,y| x * y).unwrap();

	new_num.retain(|f| ! d_factors.contains(&f));
	res_num = Iterator::reduce(new_num.into_iter(), |x,y| x * y).unwrap();
    };

    (res_num, res_den)
}

pub fn is_palindrome(n : u128) -> bool {
    let ciphres = (n as f64).log10() as u128 + 1;

    let mut  result = true;
    for i in 0..(ciphres/2) {
	let l_mod = 10_u128.pow((i + 1) as u32);
	let u_mod = 10_u128.pow((ciphres - i) as u32);
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

pub fn divisors_n(n : u64) -> u64 {
    let primes_below = primes_below(n as u32);

    let mut rest = n;
    let mut i = 0;
    let mut count = 1;
    
    loop {
	let current_prime = primes_below[i] as u64;	
        let mut inner_count = 1;
	while rest % current_prime == 0 {
	    rest /= current_prime;
	    inner_count += 1;
	};
	i += 1;
	count *= inner_count;
	if rest == 1 {break;};
    }
    count
}

pub struct CollatzSequence {
    pub known_sequences : HashMap<u64, Vec<u64>>,
}

impl CollatzSequence {
    pub fn get_seq(&mut self, n : u64) -> Vec<u64> {
	if n == 1 {return vec![1]};
	let next_step = if n % 2 == 0 {n / 2} else {3*n + 1};

	if !self.known_sequences.contains_key(&n) {
	    let mut new_sequence = vec![n];
	    new_sequence.append(&mut self.get_seq(next_step));
	    self.known_sequences.insert(
		n, new_sequence
	    );
	}

	self.known_sequences.get(&n).unwrap().to_vec()
    }

    pub fn new_seq() -> CollatzSequence {
	CollatzSequence{known_sequences : HashMap::new()}
    }
}

pub fn digits<T : std::fmt::Display>(n : T) -> Vec<u32> {
    n.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect()
}

pub fn digits_to_n<T: Unsigned +NumCast>(digits : Vec<u32>) -> T {
    let mut n : T = NumCast::from(0).unwrap();
    for j in  0..digits.len() {
	n = n + NumCast::from((digits[j] as u128) * 10_u128.pow((digits.len() - j - 1) as u32)).unwrap();
    };
    n
}

pub fn is_abundant(n : u64) -> bool {
    divisors(n).iter().sum::<u64>() - n > n
}

pub struct BigFibonacciSequence {
    last_two : [BigUint; 2],
}

impl BigFibonacciSequence {

    pub fn new_big_fibonacci() -> BigFibonacciSequence {
	BigFibonacciSequence{last_two : ["0".parse::<BigUint>().unwrap(),"1".parse::<BigUint>().unwrap()]}
    }
    
    pub fn next(&mut self) -> BigUint {
	let next : BigUint = self.last_two.iter().sum();

	self.last_two[1] = self.last_two[0].clone();
	self.last_two[0] = next.clone();
	
	next
    }
}

pub fn factorial<T : NumCast + Unsigned + std::cmp::PartialEq + Copy> (n : T) -> T {

    if n == NumCast::from(0).unwrap() {
	NumCast::from(1).unwrap()
    } else if n == NumCast::from(1).unwrap() {
	NumCast::from(1).unwrap()
    } else {
	n * factorial(n - NumCast::from(1).unwrap())
    }
}

pub fn binary_representation<T : Unsigned + Copy + NumCast + std::cmp::PartialOrd + ToString + std::fmt::Debug>(n : T) -> String {

    let mut digits = Vec::<u32>::new();
    let mut res : T = n;

    let t_0 = NumCast::from(0).unwrap();
    while res > t_0 {

	let t_2 = NumCast::from(2).unwrap();
	digits.push(NumCast::from(res % t_2).unwrap());
	res = res / t_2;
    };
    digits_to_n::<T>(digits.into_iter().rev().collect()).to_string()
}


pub fn is_pandigital<T: Unsigned + std::fmt::Display> (n : T) -> bool {

    let digs = digits(n);
    let mut is_pan = true;
    for i in 1..digs.len() + 1 {
	if !digs.contains(&(i as u32)) {
	    is_pan = false;
	    break;
	};
    };
    is_pan
}

pub struct ChampernowneConstant{
    current_n : u128,
    current_digits : Vec<u32>
}

pub fn is_permutation<T : Clone + std::cmp::PartialEq + std::cmp::Ord>(a : Vec<T>, b : Vec<T>) -> bool {

    let mut s_a = a.clone();
    s_a.sort();
    let mut s_b = b.clone();
    s_b.sort();

    s_a == s_b
}

impl ChampernowneConstant {

    pub fn new_champernowne() -> ChampernowneConstant {
	ChampernowneConstant{current_n : 1, current_digits : vec![1]}
    }

    pub fn next_digit(&mut self) -> u32 {
	let next_d = self.current_digits.pop().unwrap();

	if self.current_digits.is_empty() {
	    self.current_n += 1;
	    self.current_digits = digits(self.current_n).into_iter().rev().collect();
	};

	next_d
    }
}


pub enum GeometricNumber {Triangular, Pentagonal, Hexagonal}

pub struct GeometricalNumbers<T> {
    seen : HashMap<T,T>,
    shape : GeometricNumber
}


impl<T> GeometricalNumbers<T>
where T : Unsigned + num::NumCast + Eq + std::hash::Hash + Copy
{
    pub fn new_geometrical(shape : GeometricNumber) -> GeometricalNumbers<T> {
	GeometricalNumbers{seen: HashMap::<T, T>::new(), shape : shape}
    }

    pub fn get_number(&mut self, n : T) -> T {
	if self.seen.contains_key(&n) {
	    *self.seen.get(&n).unwrap()
	} else {
	    let t_1 : T = NumCast::from(1).unwrap();
	    let t_2 : T = NumCast::from(2).unwrap();
	    let t_3 : T = NumCast::from(3).unwrap();
	    let result =
		match self.shape {
		    GeometricNumber::Triangular => n*(n+t_1)/t_2,
		    GeometricNumber::Pentagonal => n*(t_3*n-t_1)/t_2,
		    GeometricNumber::Hexagonal => n*(t_2*n-t_1)
		};
	    self.seen.insert(n, result);
	    result
	}
    }
}

pub fn ascii_score(ascii : &char) -> u32 {
    *ascii as u32 - 64
}

pub fn word_score(word : String) -> u32 {
    word.chars().map(|c| ascii_score(&c)).sum()
}

pub fn continuous_fraction_approx<F>(level : u32, p_vals : F, base : BigUint) -> (BigUint, BigUint)
where F : Fn(u32) -> u32 {
    continuous_fraction_approx_rec(level, p_vals, (0.to_biguint().unwrap(),1.to_biguint().unwrap()), base)
}

fn continuous_fraction_approx_rec<F>(level : u32, p_vals : F, result :  (BigUint, BigUint), base : BigUint) -> (BigUint, BigUint)
where F : Fn(u32) -> u32 {
    if level > 1 {
	let den = result.1.clone();
	let num = result.1.clone() * p_vals(level) + result.0;
	continuous_fraction_approx_rec(level-1, p_vals, (den, num), base) }
    else {
	(result.0 + base*result.1.clone(), result.1) }
}

