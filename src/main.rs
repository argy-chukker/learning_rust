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

    let answer_3 = euler_problems::problem_3_solver(600851475143_f64);
    println!("The answer to problem 3 is: {:?}", answer_3);    

    let answer_4 = euler_problems::problem_4_solver(2, 3);
    println!("The answer to problem 4 is: {:?}", answer_4);    

    let answer_5 = euler_problems::problem_5_solver(20);
    println!("The answer to problem 5 is: {:?}", answer_5);    

    let answer_6 = euler_problems::problem_6_solver(100);
    println!("The answer to problem 6 is: {:?}", answer_6);    

    let answer_7 = euler_problems::problem_7_solver(10001);
    println!("The answer to problem 7 is: {:?}", answer_7);    

    let answer_8 = euler_problems::problem_8_solver(13, None);
    println!("The answer to problem 8 is: {:?}", answer_8);    

    let answer_9 = euler_problems::problem_9_solver();
    println!("The answer to problem 9 is: {:?}", answer_9);    

    let answer_10 = euler_problems::problem_10_solver(2000000);
    println!("The answer to problem 10 is: {:?}", answer_10);    

    let answer_11 = euler_problems::problem_11_solver(4, None);
    println!("The answer to problem 11 is: {:?}", answer_11);    

    let answer_12 = euler_problems::problem_12_solver(500);
    println!("The answer to problem 12 is: {:?}", answer_12);

    let answer_13 = euler_problems::problem_13_solver(None, 10);
    println!("The answer to problem 13 is: {:?}", answer_13);

    let answer_14 = euler_problems::problem_14_solver(1000000);
    println!("The answer to problem 14 is: {:?}", answer_14);

    let answer_15 = euler_problems::problem_15_solver(20,20);
    println!("The answer to problem 15 is: {:?}", answer_15);

    let answer_16 = euler_problems::problem_16_solver(1000);
    println!("The answer to problem 16 is: {:?}", answer_16);

    let answer_17 = euler_problems::problem_17_solver(1000);
    println!("The answer to problem 17 is: {:?}", answer_17);

    let answer_18 = euler_problems::problem_18_solver(None);
    println!("The answer to problem 18 is: {:?}", answer_18);

    let answer_67 = euler_problems::problem_67_solver(None);
    println!("The answer to problem 67 is: {:?}", answer_67);
}
 
 
