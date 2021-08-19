pub mod euler;

pub use crate::euler::euler_problems;

fn main() {
    let answer_1 = euler_problems::problem_1_solver(vec![3,5], 1000);
    println!("The answer to problem 1 is: {}", answer_1);
}

