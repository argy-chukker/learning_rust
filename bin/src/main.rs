pub mod euler;

pub use crate::euler::euler_problems;

fn main() {
    report_euler_answers();
}

fn report_euler_answers() -> () {
    let answer_1 = euler_problems::problem_1_solver(vec![3,5], 1000);
    println!("The answer to problem 1 is: {}", answer_1);

    let answer_2 = euler_problems::problem_2_solver(4000000);
    println!("The answer to problem 2 is: {}", answer_2);    

    // By brute force I know 1471 is a prime divisor.
    let answer_3 = euler_problems::problem_3_solver(600851475143_f64);
    println!("The answer to problem 3 is: {:?}", answer_3) ;    
}
 
 
