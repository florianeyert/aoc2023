use std::env;
use std::time::Instant;

mod solutions;

fn compute_answer(day: usize, part: usize, solution: &(&fn() -> i64, Option<i64>)) {
    let now: Instant = Instant::now();
    let result: i64 = (solution.0)();
    if (solution.1).is_some() {
        assert_eq!(result, (solution.1).unwrap())
    }
    println!("[Day {}][Part {}] Puzzle answer is {} [{}ms]", day, part, result, now.elapsed().as_millis());
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let solutions: Vec<((usize, usize), (&fn() -> i64, Option<i64>))>  = Vec::from([
        ((1, 1), (&(solutions::aoc23_1_1 as fn() -> i64), Some(54940))),
        ((1, 2), (&(solutions::aoc23_1_2 as fn() -> i64), Some(54208))),
        ((2, 1), (&(solutions::aoc23_2_1 as fn() -> i64), Some(3035))),
        ((2, 2), (&(solutions::aoc23_2_2 as fn() -> i64), Some(66027))),
        ((3, 1), (&(solutions::aoc23_3_1 as fn() -> i64), Some(521601))),
        ((3, 2), (&(solutions::aoc23_3_2 as fn() -> i64), Some(80694070))),
        ((4, 1), (&(solutions::aoc23_4_1 as fn() -> i64), Some(25004))),
        ((4, 2), (&(solutions::aoc23_4_2 as fn() -> i64), Some(14427616))),
        ((5, 1), (&(solutions::aoc23_5_1 as fn() -> i64), Some(226172555))),
        ((5, 2), (&(solutions::aoc23_5_2 as fn() -> i64), Some(47909639)))
    ]); 

    if !(args.len() == 3 || (args.len() == 2 && args[1].as_str() == "all")) {
        println!("Enter exactly two arguments or 'all' as first argument.");
        std::process::exit(0);
    }
    match args[1].as_str() {
        "all" => {
            solutions.into_iter().for_each(|solution| {
                compute_answer(solution.0.0, solution.0.1, &(solution.1.0, solution.1.1))
            })
        },
        _ =>  match args[1].parse::<usize>() {
            Ok(day @ 1..=5)  => match args[2].parse::<usize>() {
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
