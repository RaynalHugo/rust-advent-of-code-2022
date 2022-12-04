#[allow(dead_code)]

pub fn solve(input: &str) -> usize {
    input
        .split('\n')
        .filter(|shift| {
            let assignments: Vec<_> = shift
                .split(',')
                .map(|assignment| {
                    assignment
                        .split('-')
                        .map(|period| period.parse::<u8>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect();

            let first_elf_assignment = assignments.first().unwrap();
            let first_elf_assignment_range =
                first_elf_assignment.first().unwrap()..=first_elf_assignment.get(1).unwrap();

            let second_elf_assignment = assignments.get(1).unwrap();
            let second_elf_assignment_range =
                second_elf_assignment.first().unwrap()..=second_elf_assignment.get(1).unwrap();

            if first_elf_assignment_range.contains(&second_elf_assignment.first().unwrap())
                && first_elf_assignment_range.contains(&second_elf_assignment.get(1).unwrap())
            {
                return true;
            }

            if second_elf_assignment_range.contains(&first_elf_assignment.first().unwrap())
                && second_elf_assignment_range.contains(&first_elf_assignment.get(1).unwrap())
            {
                return true;
            }

            false
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::solve;
    use std::fs;

    #[test]
    fn example() {
        let input = fs::read_to_string("./inputs/day_04_example.txt")
            .expect("couldn't find file ./inputs/day_04_example.txt");

        assert_eq!(solve(&input), 2);
    }

    #[test]
    fn given_input() {
        let input = fs::read_to_string("./inputs/day_04_input.txt")
            .expect("couldn't find file ./inputs/day_04_input.txt");

        assert_eq!(solve(&input), 466);
    }
}
