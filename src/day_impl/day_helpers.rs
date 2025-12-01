macro_rules! create_day_runner {
    ($runner_name: ident) => {
        pub struct $runner_name {}

        impl $runner_name {
            pub fn new() -> Self {
                Self {}
            }
        }
    };
}