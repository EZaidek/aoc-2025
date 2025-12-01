/* Standard Imports */
use std::{env, fmt::write};
use std::fmt::Display;

/* Libary Imports */
use clap::Parser;
use tokio::{self};
use reqwest::{self, Client, header::{HeaderMap, HeaderValue}};

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

/* Structs and Global Data */
const MAX_DAYS: u32 = 25;
const YEAR: u32 = 2025;

#[derive(Debug)]
struct DayError;

#[derive(Debug)]
enum Outcome {
    Passed(u32),
    FailedTooHigh(u32),
    FailedTooLow(u32),
    FailedOther(u32),
}

impl Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Passed(x) => write!(f, "Guess = {x} ✅"),
            Self::FailedTooHigh(x) => write!(f, "Guess = {x} ❌ Guess too high"),
            Self::FailedTooLow(x) => write!(f, "Guess = {x} ❌ Guess too low"),
            Self::FailedOther(x) => write!(f, "Guess = {x} ❌ Submission Failed"),
        }
    }
}

#[derive(Parser)]
#[command(name="run_aoc", author, version, about)]
struct Cli {
    #[arg(long = "day", short = 'd')]
    day: Option<u32>,
    #[arg(long, short)]
    run_all_days: bool,
}

#[derive(Debug)]
pub struct AocClient {
    http_client: Client,
    current_day: Option<u32>,
    run_all_days: bool,
}

impl AocClient {

    fn new(cli: &Cli) -> Self {
        Self {
            http_client: AocClient::build_client(),
            current_day: cli.day,
            run_all_days: cli.run_all_days,
        }
    }

    fn build_client() -> Client{
        reqwest::Client::builder()
            .default_headers({
                let mut headers = HeaderMap::new();
                headers.insert(
                    "Cookie",
                    HeaderValue::from_str(env::var("AOC_TOKEN").unwrap().as_str()).unwrap(),
                );
                headers
            })
            .user_agent("github/EZaidek/aoc-2025 by EZaidek")
            .build()
            .expect("Failed to build http client")
    }

    async fn start(&self) {
        if self.run_all_days {
            for i in 1..=MAX_DAYS {
                self.start_day(Some(i)).await;
            }
        }
        self.start_day(self.current_day).await;
    }

    async fn start_day(&self, day: Option<u32>) {
        let day_num = day.expect("A day number must be given. Usage: run_aoc --day [DAY]");
        let runner = get_runner(day_num).expect("Invalid day number given");
        let result: (Outcome, Outcome) = self.run_day(day_num, runner).await;
        println!("Day {day_num}:");
        println!("   P1: {}", result.0);
        println!("   P2: {}", result.1);
    }

    async fn run_day(&self, day: u32, runner: Box<dyn AocDay>) -> (Outcome, Outcome) {
        let input: Vec<Vec<String>> = self.get_input(day).await;
        let p1 = self.submit_solution(day, runner.part1(&input), 1).await.expect("Failed to submit part 1 solution");
        let p2 = self.submit_solution(day, runner.part2(&input), 2).await.expect("Failed to submit part 2 solution");
        (p1, p2)
    }

    async fn get_input(&self, day: u32) -> Vec<Vec<String>> {

        let path = format!("src/inputs/day{day}.txt");

        // if input has already been cached
        if std::path::Path::new(&path).exists() {
            let contents = std::fs::read_to_string(&path).expect("Failed to read file");
            return contents.split("\n").map(|s| s.chars().map(|c| c.to_string()).collect()).collect()
        }

        let resp = self.http_client 
            .get(format!("https://adventofcode.com/{YEAR}/day/{day}/input"))
            .send()
            .await
            .expect("Failed unwrapping GET response")
            .text()
            .await
            .expect("Failed unwrapping GET response text");

        std::fs::File::create(&path).expect("Failed to create file");
        std::fs::write(&path, &resp).expect("Failed to write to file");
        resp.lines().map(|s| s.chars().map(|c| c.to_string()).collect()).collect()
    }

    async fn submit_solution(&self, day: u32, solution: u32, part: u32) -> Result<Outcome, reqwest::Error> {
        let sumbission = [("level", part.to_string()), ("answer", solution.to_string())];
        let resp = self.http_client
            .post(format!("https://adventofcode.com/{YEAR}/day/{day}/answer"))
            .form(&sumbission)
            .send()
            .await
            .expect("Failed unwrapping POST response")
            .text()
            .await;

        match resp {
            Ok(s) if s.contains("too low") => Ok(Outcome::FailedTooLow(solution)),
            Ok(s) if s.contains("too high") => Ok(Outcome::FailedTooHigh(solution)),
            Ok(s) if s.contains("That's the right answer") => Ok(Outcome::Passed(solution)),
            Ok(_) => Ok(Outcome::FailedOther(solution)),
            Err(e) => Err(e), 
        }
    }
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
#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let aoc_client = AocClient::new(&cli);
    aoc_client.start().await;
}
