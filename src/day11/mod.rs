pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};

pub type Input = Vec<input::MonkeyItems>;

pub fn run(part: Part) -> Output {
    match part {
        Part::OneEx => {
            let mut exampleinput = input::readex();
            part1::solve(&mut exampleinput)
        }
        Part::One => {
            let mut input = input::read();
            part1::solve(&mut input)
        }
        Part::TwoEx => {
            let mut exampleinput = input::readex();
            part2::solve(&mut exampleinput)
        }
        Part::Two => {
            let mut input = input::read();
            part2::solve(&mut input)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_one_example() {
        let result = run(Part::OneEx);
        println!("{result}");
    }

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        println!("{result}");
    }

    #[test]
    fn check_answer_two_example() {
        let result = run(Part::TwoEx);
        println!("{result}");
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        println!("{result}");
    }
}
