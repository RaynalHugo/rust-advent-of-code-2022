#[allow(dead_code)]

pub fn solve(input: &str) -> u32 {
    input
        .split('\n')
        .map(|rucksack| {
            let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);
            let mut first_compartment = first_compartment.chars().collect::<Vec<_>>();
            first_compartment.dedup();

            let mut second_compartment = second_compartment.chars().collect::<Vec<_>>();
            second_compartment.dedup();

            let duplicate = second_compartment
                .into_iter()
                .find(|item| first_compartment.contains(item))
                .expect("Couldn't find duplicate");

            if duplicate.is_ascii_uppercase() {
                duplicate as u32 - 64 + 26
            } else {
                duplicate as u32 - 96
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

        assert_eq!(solve(&input), 157);
    }

    #[test]
    fn given_input() {
        let input = fs::read_to_string("./inputs/day_03_input.txt")
            .expect("couldn't find file ./inputs/day_03_input.txt");

        assert_eq!(solve(&input), 7701);
    }
}
