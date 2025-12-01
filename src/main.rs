/* Libary Imports */
use clap::Parser;
use enum_dispatch::enum_dispatch;

/* Local Imports */
mod day_impl;
use day_impl::*;

/* Macro Definitions */
macro_rules! get_runners {
    ($(($runner_name: ident, $day_num: expr, $enum_name: ident),)+) => {

        #[enum_dispatch]
        enum Runner {
            $($enum_name($runner_name),)+
        }

        fn get_runner(day: u32) -> Result<Runner, DayError> {
            match day {
                $($day_num => Ok(Runner::$enum_name($runner_name::new())),)+
                _ => Err(DayError)
            }
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

get_runners!(
    (DayRunner1, 1, DAY1), 
    (DayRunner2, 2, DAY2),
    (DayRunner3, 3, DAY3),
    (DayRunner4, 4, DAY4),
    (DayRunner5, 5, DAY5),
    (DayRunner6, 6, DAY6),
    (DayRunner7, 7, DAY7),
    (DayRunner8, 8, DAY8),
    (DayRunner9, 9, DAY9),
    (DayRunner10, 10, DAY10),
    (DayRunner11, 11, DAY11),
    (DayRunner12, 12, DAY12),
    (DayRunner13, 13, DAY13),
    (DayRunner14, 14, DAY14),
    (DayRunner15, 15, DAY15),
    (DayRunner16, 16, DAY16),
    (DayRunner17, 17, DAY17),
    (DayRunner18, 18, DAY18),
    (DayRunner19, 19, DAY19),
    (DayRunner20, 20, DAY20),
    (DayRunner21, 21, DAY21),
    (DayRunner22, 22, DAY22),
    (DayRunner23, 23, DAY23),
    (DayRunner24, 24, DAY24),
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
#[enum_dispatch(Runner)]
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
    let day_num = match day {
        None => cli.day.expect("Day number must be provided."),
        Some(d) => d,
    };
    
    let runner: Runner = get_runner(day_num).expect("Invalid day number given"); 
    let result: (Outcome, Outcome) = run_day(day_num, runner);
    println!("DAY {}: PART1 - {:?}, PART2 - {:?}", day_num, result.0, result.1)
}

fn run_day(day: u32, runner: Runner) -> (Outcome, Outcome) {
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





