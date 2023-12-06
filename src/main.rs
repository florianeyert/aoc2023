use std::env;
use std::time::Instant;

mod solutions;

fn compute_answer(day: usize, part: usize, solution: &(&fn() -> i64, Option<i64>)) {
    let now: Instant = Instant::now();
    let result: i64 = (solution.0)();
    if (solution.1).is_some() {
        assert_eq!(result, (solution.1).unwrap())
    }
    println!("[Day {}][Part {}] Solution: {} [{}.{:0>3}ms]", day, part, result,
    now.elapsed().as_millis(), now.elapsed().as_micros() % 1000);
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let solutions: Vec<((usize, usize), (&fn() -> i64, Option<i64>))>  = Vec::from([
        ((1, 1), (&(solutions::d1p1 as fn() -> i64), Some(54940))),
        ((1, 2), (&(solutions::d1p2 as fn() -> i64), Some(54208))),
        ((2, 1), (&(solutions::d2p1 as fn() -> i64), Some(3035))),
        ((2, 2), (&(solutions::d2p2 as fn() -> i64), Some(66027))),
        ((3, 1), (&(solutions::d3p1 as fn() -> i64), Some(521601))),
        ((3, 2), (&(solutions::d3p2 as fn() -> i64), Some(80694070))),
        ((4, 1), (&(solutions::d4p1 as fn() -> i64), Some(25004))),
        ((4, 2), (&(solutions::d4p2 as fn() -> i64), Some(14427616))),
        ((5, 1), (&(solutions::d5p1 as fn() -> i64), Some(226172555))),
        ((5, 2), (&(solutions::d5p2 as fn() -> i64), Some(47909639))),
        ((6, 1), (&(solutions::d6p1 as fn() -> i64), Some(2374848))),
        ((6, 2), (&(solutions::d6p2 as fn() -> i64), Some(39132886)))
    ]); 

    if !(args.len() == 3 || (args.len() == 2 && args[1].as_str() == "all")) {
        println!("Enter exactly two arguments or 'all' as first argument.");
        std::process::exit(0);
    }
    match args[1].as_str() {
        "all" => {
            let now: Instant = Instant::now();
            let num_solutions = solutions.len();
            solutions.into_iter().for_each(|solution| {
                compute_answer(solution.0.0, solution.0.1, &(solution.1.0, solution.1.1))
            });
            let elapsed = now.elapsed();
            println!("\nRan everything in {}.{} seconds (on average {}ms per task).",
                elapsed.as_secs(), elapsed.as_millis() % 1000, elapsed.as_millis() / num_solutions as u128);
        },
        _ =>  match args[1].parse::<usize>() {
            Ok(day @ 1..=6)  => match args[2].parse::<usize>() {
                Ok(part @ 1..=2) => {compute_answer(day, part, 
                    &solutions.iter().find(|s| s.0.0 == day && s.0.1 == part).unwrap().1)}
                Ok(part ) => {
                    println!("{} is an invalid part (must be 1 or 2).", part);
                },
                Err(_) => {
                    println!("Second argument needs to be a number.");
                }
            },
            Ok(day @ 1..=25) => {
                println!("Day {} not solved yet.", day)
            }
            Ok(day) => {
                println!("{} is an invalid day (must be a day from 1 to 25).", day)
            }
            Err(_) => {
                println!("First argument needs to be a number.")
            }
        }
    }

}
