pub trait Solver {
    fn solve(input: &heapless::String<20000>) -> heapless::String<100>;
}

pub mod day01;