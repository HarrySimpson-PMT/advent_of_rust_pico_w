use heapless::String;

pub trait Solver {
        fn solve(input: String<30000>) -> String<100>;
}