const MARKER_LENGTH: usize = 14;

#[allow(dead_code)]
pub fn solve(input: &str) -> usize {
    let chars: Vec<_> = input.chars().collect();
    let first_match = chars
        .windows(MARKER_LENGTH)
        .enumerate()
        .find(|(_index, window)| {
            let mut next_for_chars = window.to_vec();
            next_for_chars.sort();
            next_for_chars.dedup();

            next_for_chars.len() == MARKER_LENGTH
        })
        .unwrap();

    first_match.0 + MARKER_LENGTH
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

        assert_eq!(solve(examples.next().unwrap()), 19);
        assert_eq!(solve(examples.next().unwrap()), 23);
        assert_eq!(solve(examples.next().unwrap()), 23);
        assert_eq!(solve(examples.next().unwrap()), 29);
        assert_eq!(solve(examples.next().unwrap()), 26);
    }

    #[test]
    fn given_input() {
        let input = fs::read_to_string("./inputs/day_06_input.txt")
            .expect("couldn't find file ./inputs/day_06_input.txt");

        assert_eq!(solve(&input), 2447);
    }
}
