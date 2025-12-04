use std::{ops::Div};

use crate::{AocDay, DayError};

create_day_runner!(DayRunner1);

const START_VALUE: i32 = 50;
const MAX_VALUE: i32 = 100;

impl AocDay for DayRunner1 {

    fn part1(&self, input: &Vec<Vec<String>>) -> Result<u32, DayError<'_>> {
        let mut curr = START_VALUE;
        let mut zeroed = 0;
        for rotation in input {
            let dir = &rotation[0];
            let value: i32 = rotation[1..rotation.len()].join("").parse::<i32>().expect("Failed to convert rotation into int");
            match dir.as_str() {
                "L" => curr = (curr - value) % MAX_VALUE,
                "R" => curr = (curr + value) % MAX_VALUE,
                _ => panic!("Unknown directions found")
            }
            zeroed += (curr == 0) as u32;
        }

        Ok(zeroed)
    }

    fn part2(&self, input: &Vec<Vec<String>>) -> Result<u32, DayError<'_>> {
        let mut curr = START_VALUE;
        let mut zeroed: u32 = 0;
        for rotation in input {
            let dir = &rotation[0];
            let value: i32 = rotation[1..rotation.len()].join("").parse::<i32>().expect("Failed to convert rotation into int");

            println!("{}, {}", curr, zeroed);
            let full_rotates: u32 = value.div(MAX_VALUE) as u32;
            match dir.as_str() {
                "L" => {
                    zeroed += full_rotates + ((curr - (value % MAX_VALUE) <= 0 && (curr != 0)) as u32);
                    curr = (MAX_VALUE + ((curr - value) % MAX_VALUE)) % MAX_VALUE;
                },
                "R" => {
                    zeroed += full_rotates + ((curr + (value % MAX_VALUE) >= MAX_VALUE) as u32);
                    curr = (curr + value) % MAX_VALUE;
                },
                _ => panic!("Unknown directions found")
            }
        }

        Ok(zeroed)
    }
}