use heapless::String;

pub trait Solver {
    fn solve(input: &mut String<30000>) -> String<100>;
}