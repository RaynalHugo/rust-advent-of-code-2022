#[allow(dead_code)]

pub fn solve(input: &str) -> u32 {
    let lines: Vec<_> = input.split('\n').collect();
    lines
        .chunks(3)
        .map(|chunk| {
            let first = *chunk.first().unwrap();
            let mut first = first.chars().collect::<Vec<_>>();
            first.dedup();

            let second = *chunk.get(1).unwrap();
            let second = second.chars().collect::<Vec<_>>();

            let third = *chunk.get(2).unwrap();
            let third = third.chars().collect::<Vec<_>>();

            let common_item = first
                .iter()
                .find(|item| second.contains(item) && third.contains(item))
                .expect("Couldn't find common object");

            println!("{:?}", chunk);

            if common_item.is_ascii_uppercase() {
                *common_item as u32 - 64 + 26
            } else {
                *common_item as u32 - 96
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::solve;
    use std::fs;

    #[test]
    fn example() {
        let input = fs::read_to_string("./inputs/day_03_example.txt")
            .expect("couldn't find file ./inputs/day_03_example.txt");

        assert_eq!(solve(&input), 70);
    }

    #[test]
    fn given_input() {
        let input = fs::read_to_string("./inputs/day_03_input.txt")
            .expect("couldn't find file ./inputs/day_03_input.txt");

        assert_eq!(solve(&input), 2644);
    }
}
