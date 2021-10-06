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

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_5_solver(20);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 5 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_6_solver(100);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 6 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_7_solver(10001);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 7 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_8_solver(13, None);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 8 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_9_solver();
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 9 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_10_solver(2000000);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 10 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_11_solver(4, None);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 11 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_12_solver(500);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 12 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_13_solver(None, 10);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 13 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_14_solver(1000000);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 14 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_15_solver(20, 20);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 15 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_16_solver(1000);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 16 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_17_solver(1000);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 17 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_18_solver(None);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 18 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_19_solver(1901, 2000, None, None);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 19 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_20_solver(100);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 20 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_21_solver(10000);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 21 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_22_solver();
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 22 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer.unwrap());

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_23_solver();
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 23 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_24_solver(None, 1000000);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 24 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_25_solver(999);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 25 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_26_solver(1000_u32);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 26 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_27_solver(100_u32, 1000_u32, None);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 27 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_28_solver::<u32>(1001);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 28 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_29_solver(100, 100);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 29 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_30_solver(5);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 30 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_31_solver(200, None);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 31 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_32_solver();
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 32 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_33_solver();
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 33 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_34_solver();
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 34 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_35_solver(1000000);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 35 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_36_solver();
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 36 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_37_solver();
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 37 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_38_solver(None);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 38 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_39_solver();
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 39 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_40_solver(vec![1,10,100,1000,10000,100000,1000000]);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 40 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_41_solver();
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 41 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_42_solver();
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 42 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer.unwrap());

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_43_solver();
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 43 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_44_solver();
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 44 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_45_solver();
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 45 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_46_solver();
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 46 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_47_solver(4);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 47 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_48_solver(1000);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 48 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_49_solver();
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 49 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_50_solver(1000000);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 50 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_53_solver(100, 1000000);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 53 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_57_solver(1000);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 57 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_58_solver(10);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 58 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_65_solver(100);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 65 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

    let begin_t = timer.elapsed().unwrap().as_millis();
    let answer = euler_problems::problem_67_solver(None);
    let end_t = timer.elapsed().unwrap().as_millis();
    println!("Problem 67 took {} milli-seconds, and its answer is {}", end_t - begin_t, answer);

}

