use heapless::String;

pub trait Solver {
    fn solve(input: &mut String<30000>) -> String<100>;
}

pub trait SolverOwned {
    fn solve(input: String<30000>) -> (String<30000>, String<100>);
}