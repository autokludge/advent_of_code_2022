pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};

pub type Input = u8;

pub fn run(part: Part) -> Output {
    let exampleinput = input::readex();
    let input = input::read();
    match part {
        Part::OneEx => part1::solve(&exampleinput),
        Part::One => part1::solve(&input),
        Part::TwoEx => part2::solve(&exampleinput),
        Part::Two => part2::solve(&input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        println!("{result}");
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        println!("{result}");
    }
}
