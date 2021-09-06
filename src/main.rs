pub mod euler;
use num_bigint::{BigUint, ToBigUint};

pub use crate::euler::euler_problems;

fn main() {
    report_euler_answers();
}

fn report_euler_answers() -> () {

    let answer = euler_problems::problem_29_solver(100,100);
    println!("The answer to problem 29 is: {:?}", answer);

}
 
 
