#[allow(dead_code)]

pub fn solve(input: &str) -> u32 {
    let chars: Vec<_> = input.chars().collect();
    let first_match = chars
        .windows(4)
        .enumerate()
        .find(|(_index, window)| {
            let mut next_for_chars = window.to_vec();
            next_for_chars.sort();
            next_for_chars.dedup();

            next_for_chars.len() == 4
        })
        .unwrap();

    (first_match.0 as u32) + 4
}

#[cfg(test)]
mod tests {
    use super::solve;
    use std::fs;

    #[test]
    fn example() {
        let input = fs::read_to_string("./inputs/day_06_examples.txt")
            .expect("couldn't find file ./inputs/day_06_examples.txt");
        let mut examples = input.lines();

        assert_eq!(solve(examples.next().unwrap()), 7);
        assert_eq!(solve(examples.next().unwrap()), 5);
        assert_eq!(solve(examples.next().unwrap()), 6);
        assert_eq!(solve(examples.next().unwrap()), 10);
        assert_eq!(solve(examples.next().unwrap()), 11);
    }

    #[test]
    fn given_input() {
        let input = fs::read_to_string("./inputs/day_06_input.txt")
            .expect("couldn't find file ./inputs/day_06_input.txt");

        assert_eq!(solve(&input), 1647);
    }
}
