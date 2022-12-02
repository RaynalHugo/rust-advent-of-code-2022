#[allow(dead_code)]

pub fn solve(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .map(|value_as_string| value_as_string.parse::<u32>().expect("couldn't parse"))
                .sum()
        })
        .max()
        .expect("Couldn't find max")
}

#[cfg(test)]
mod tests {
    use super::solve;
    use std::fs;

    #[test]
    fn example() {
        let input = fs::read_to_string("./inputs/day_01_example.txt")
            .expect("couldn't find file ./inputs/day_01_example.txt");

        assert_eq!(solve(&input), 24000);
    }

    #[test]
    fn given_input() {
        let input = fs::read_to_string("./inputs/day_01_input.txt")
            .expect("couldn't find file ./inputs/day_01_input.txt");

        assert_eq!(solve(&input), 69836);
    }
}
