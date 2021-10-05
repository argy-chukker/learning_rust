use crate::euler::euler_problems;
use num_bigint::{BigUint, ToBigUint};
use std::time::{Duration, SystemTime};


pub fn benchmark() -> () {

    let timer = SystemTime::now();

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_1_solver(vec![3,5], 1000);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 1 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_2_solver(4000000);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 2 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_3_solver(600851475143_f64);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 3 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_4_solver(2, 3);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 4 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

}

