/* Libary Imports */
use clap::Parser;

/* Local Imports */
mod day_impl;
use day_impl::*;

/* Macro Definitions */
macro_rules! get_runners2 {
    ($(($runner_name: ident, $day_num: expr),)+) => {
        fn get_runner(day: u32) -> Result<Box<dyn AocDay>, DayError> {
            let runner_impl: Result<Box<dyn AocDay>, DayError> = match day {
                $($day_num => Ok(Box::new($runner_name::new())),)+
                _ => Err(DayError)
            };
            runner_impl
        }
    };
}

/* Structs and Global Data */
const MAX_DAYS: u32 = 25;

#[derive(Debug)]
struct DayError;

#[derive(Debug)]
enum Outcome {
    Passed,
    Failed,
}

get_runners2!(
    (DayRunner1, 1), 
    (DayRunner2, 2),
    (DayRunner3, 3),
    (DayRunner4, 4),
    (DayRunner5, 5),
    (DayRunner6, 6),
    (DayRunner7, 7),
    (DayRunner8, 8),
    (DayRunner9, 9),
    (DayRunner10, 10),
    (DayRunner11, 11),
    (DayRunner12, 12),
    (DayRunner13, 13),
    (DayRunner14, 14),
    (DayRunner15, 15),
    (DayRunner16, 16),
    (DayRunner17, 17),
    (DayRunner18, 18),
    (DayRunner19, 19),
    (DayRunner20, 20),
    (DayRunner21, 21),
    (DayRunner22, 22),
    (DayRunner23, 23),
    (DayRunner24, 24),
);

#[derive(Parser)]
#[command(name="run_aoc", author, version, about)]
struct Cli {
    #[arg(long = "day", short = 'd')]
    day: Option<u32>,
    #[arg(long, short)]
    run_all_days: bool,
}

/* Traits */
trait AocDay {
    fn part1(&self, input: &Vec<Vec<String>>) -> u32 {
        unimplemented!("Part 1 not implemented");
    }
    fn part2(&self, input: &Vec<Vec<String>>) -> u32 {
        unimplemented!("Part 2 not implemented");
    }
} 

/* Functions */
fn main() {
    let cli = Cli::parse();

    if cli.run_all_days {
        for day_num in 1..MAX_DAYS {
            start_day(Some(day_num), &cli);
        }
    } else {
        start_day(None, &cli);
    }
}

fn start_day(day: Option<u32>, cli: &Cli) {
    let day_num = day.unwrap_or(cli.day.expect("Day number must be provided."));
    let runner = get_runner(day_num).expect("Invalid day number given"); 
    let result: (Outcome, Outcome) = run_day(day_num, runner);
    println!("DAY {}: PART1 - {:?}, PART2 - {:?}", day_num, result.0, result.1)
}

fn run_day(day: u32, runner: Box<dyn AocDay>) -> (Outcome, Outcome) {
    let input: Vec<Vec<String>> = get_input(day);
    let p1 = submit_solution(day, runner.part1(&input));
    let p2 = submit_solution(day, runner.part2(&input));
    (p1, p2)
}

fn get_input(day: u32) -> Vec<Vec<String>> {
    todo!();
}

fn submit_solution(day: u32, solution: u32) -> Outcome {
    todo!();
}





