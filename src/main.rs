use std::env;
use std::time::Instant;

mod solutions;

fn compute_answer(day: usize, part: usize, solution: &(&fn() -> i64, Option<i64>)) {
    let now: Instant = Instant::now();
    let result: i64 = (solution.0)();
    if (solution.1).is_some() {
        assert_eq!(result, (solution.1).unwrap())
    }
    println!("[Day {: >2}][Part {}] Solution: {: >15} [{: >4}.{:0>3}ms]", day, part, result,
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
        ((6, 2), (&(solutions::d6p2 as fn() -> i64), Some(39132886))),
        ((7, 1), (&(solutions::d7p1 as fn() -> i64), Some(251058093))),
        ((7, 2), (&(solutions::d7p2 as fn() -> i64), Some(249781879))),
        ((8, 1), (&(solutions::d8p1 as fn() -> i64), Some(13301))),
        ((8, 2), (&(solutions::d8p2 as fn() -> i64), Some(7309459565207))),
        ((9, 1), (&(solutions::d9p1 as fn() -> i64), Some(2043677056))),
        ((9, 2), (&(solutions::d9p2 as fn() -> i64), Some(1062))),
        ((10, 1), (&(solutions::d10p1 as fn() -> i64), Some(6717))),
        ((10, 2), (&(solutions::d10p2 as fn() -> i64), Some(381))),
        ((11, 1), (&(solutions::d11p1 as fn() -> i64), Some(9545480))),
        ((11, 2), (&(solutions::d11p2 as fn() -> i64), Some(406725732046))),
        ((12, 1), (&(solutions::d12p1 as fn() -> i64), Some(7307))),
        ((12, 2), (&(solutions::d12p2 as fn() -> i64), None)),
        ((13, 1), (&(solutions::d13p1 as fn() -> i64), Some(35521))),
        ((13, 2), (&(solutions::d13p2 as fn() -> i64), Some(34795))),
        ((14, 1), (&(solutions::d14p1 as fn() -> i64), Some(110128))),
        ((14, 2), (&(solutions::d14p2 as fn() -> i64), Some(103861))),
        ((15, 1), (&(solutions::d15p1 as fn() -> i64), Some(506269))),
        ((15, 2), (&(solutions::d15p2 as fn() -> i64), Some(264021))),
        ((16, 1), (&(solutions::d16p1 as fn() -> i64), Some(7951))),
        ((16, 2), (&(solutions::d16p2 as fn() -> i64), Some(8148))),
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
            println!("\nFound all solutions in {}.{} seconds ({}ms per task on average).",
                elapsed.as_secs(), elapsed.as_millis() % 1000, elapsed.as_millis() / num_solutions as u128);
        },
        _ =>  match args[1].parse::<usize>() {
            Ok(day @ 1..=16)  => match args[2].parse::<usize>() {
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
