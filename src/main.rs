pub mod abbey;
pub mod euler;

pub use crate::abbey::abbey_problems;
pub use crate::euler::euler_benchmark;
pub use crate::euler::euler_problems;
pub use crate::euler::poker;
pub use crate::euler::utils;
use num_bigint::{BigUint, ToBigUint};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match &args[1][..] {
        "b" => euler_benchmark::benchmark(),
        "a" => {
            abbey_problems::abbey_problem_16(args[2..].to_vec()).unwrap();
        }
        _ => report_euler_answers(),
    };
}

fn report_euler_answers() -> () {
    //    let answer = euler_problems::problem_251_solver(110_000_000);
    //    let answer = euler_problems::problem_251_solver(100_000);
    //    let answer = euler_problems::problem_251_solver(10_000);

    let answer = euler_problems::problem_756_solver(4,3);
    println!("The answer to problem 251 is: {:?}", answer);
}
