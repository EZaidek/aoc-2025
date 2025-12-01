macro_rules! get_runners {
    ($(($runner_name: ident, $day_num: expr)),+) => {
        fn get_runner(day: u32) -> impl AocDay {
            match day {
                $($day_num => {day_impl::$runner_name::new()})+
                _ => {panic!("Runner for day {} does not exist", day);}
            }
        }
    };
}