pub mod euler;
pub mod abbey;

pub use crate::abbey::abbey_problems;
pub use crate::euler::euler_problems;
pub use crate::euler::euler_benchmark;
pub use crate::euler::utils;
use num_bigint::{BigUint, ToBigUint};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match &args[1][..] {
	"b" => euler_benchmark::benchmark(),
	"a" => {abbey_problems::abbey_problem_1(args[2..].to_vec()).unwrap();},
	_ => report_euler_answers(),
    };
}

fn report_euler_answers() -> () {

    let answer = euler_problems::problem_1_solver(vec![3,5], 1000);
    println!("The answer to problem 1 is: {:?}", answer);
    
}
 
 
