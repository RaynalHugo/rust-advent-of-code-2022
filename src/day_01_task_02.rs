#[allow(dead_code)]

pub fn solve(input: &str) -> u32 {
    let elf_calories: Vec<u32> = input
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .map(|value_as_string| value_as_string.parse::<u32>().expect("couldn't parse"))
                .sum()
        })
        .collect();

    elf_calories
        .iter()
        .fold([0, 0, 0], |acc, &value| {
            if value > acc[0] {
                return [value, acc[0], acc[1]];
            }
            if value > acc[1] {
                return [acc[0], value, acc[1]];
            }

            if value > acc[2] {
                return [acc[0], acc[1], value];
            }

            acc
        })
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_01_task_02::solve;
    use std::fs;

    #[test]
    fn example() {
        let input = fs::read_to_string("./inputs/day_01_example.txt")
            .expect("couldn't find file ./inputs/day_01_example.txt");

        assert_eq!(solve(&input), 45000);
    }

    #[test]
    fn given_input() {
        let input = fs::read_to_string("./inputs/day_01_input.txt")
            .expect("couldn't find file ./inputs/day_01_input.txt");

        assert_eq!(solve(&input), 207968);
    }
}
