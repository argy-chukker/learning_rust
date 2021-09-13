pub mod euler;

pub use crate::euler::euler_problems;

fn main() {
    report_euler_answers();
}

fn report_euler_answers() -> () {

    let answer = euler_problems::problem_37_solver();
    println!("The answer to problem 37 is: {:?}", answer);

}
 
 
