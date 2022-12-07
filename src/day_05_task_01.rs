use std::collections::VecDeque;

#[allow(dead_code)]

pub fn solve(input: &str) -> String {
    let parts: Vec<_> = input.split("\n\n").collect();
    let schema_part = parts.first().expect("Couldn't find first part");
    let moves_part = parts.get(1).expect("Couldn't find second part");

    let mut stacks = parse_schema(schema_part);

    let moves = parse_moves(moves_part);

    // perform moves
    moves.iter().for_each(|(repeat, from, to)| {
        (0..(*repeat)).for_each(|_index| {
            let res = stacks.get_mut(*from - 1).unwrap().pop_front().unwrap();

            stacks.get_mut(*to - 1).unwrap().push_front(res);
        })
    });

    stacks.iter().filter_map(|stack| stack.get(0)).collect()
}

fn get_index_of_stack(n: u32) -> usize {
    if n == 1 {
        return 1;
    }

    (n as usize - 1) * 4 + 1
}

fn parse_schema(schema_input: &&str) -> Vec<VecDeque<char>> {
    let number_of_stacks: u32 = schema_input
        .lines()
        .last()
        .unwrap()
        .split(' ')
        .filter(|value| !value.is_empty())
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let stacks: Vec<VecDeque<_>> = (1..=number_of_stacks)
        .map(|stack_number| {
            let column_index = get_index_of_stack(stack_number);
            (0..(schema_input.lines().count() - 1))
                .filter_map(|row_index| {
                    let row = schema_input.lines().nth(row_index).unwrap();

                    let character = row.chars().nth(column_index).unwrap();

                    if character.is_ascii_whitespace() {
                        return None;
                    }

                    Some(character)
                })
                .collect()
        })
        .collect();

    stacks
}

fn parse_moves(moves_part: &&str) -> Vec<(usize, usize, usize)> {
    moves_part
        .lines()
        .map(|single_move| {
            let mut chars = single_move.split(' ');

            let repeat: usize = chars.nth(1).unwrap().to_string().parse().unwrap();
            let from: usize = chars.nth(1).unwrap().to_string().parse().unwrap();
            let to: usize = chars.nth(1).unwrap().to_string().parse().unwrap();

            (repeat, from, to)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::solve;
    use std::fs;

    #[test]
    fn example() {
        let input = fs::read_to_string("./inputs/day_05_example.txt")
            .expect("couldn't find file ./inputs/day_05_example.txt");

        assert_eq!(solve(&input), "CMZ");
    }

    #[test]
    fn given_input() {
        let input = fs::read_to_string("./inputs/day_05_input.txt")
            .expect("couldn't find file ./inputs/day_05_input.txt");

        assert_eq!(solve(&input), "CFFHVVHNC");
    }
}
