pub mod euler;

pub use crate::euler::euler_problems;

fn main() {
    report_euler_answers();
}

fn report_euler_answers() -> () {

    let answer = euler_problems::problem_24_solver(None, 1000000);
    println!("The answer to problem 24 is: {:?}", answer);

}
 
 
