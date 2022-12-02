#[allow(dead_code)]

pub fn solve(input: &str) -> u32 {
    input
        .split('\n')
        .map(|round| {
            let res = round.split(' ').collect::<Vec<&str>>();
            let opponent_move = res.first().unwrap().chars().next().unwrap();
            let player_move = res.get(1).unwrap().chars().next().unwrap();
            process_match(opponent_move, player_move)
        })
        .sum()
}

const LOSE: u32 = 0;
const DRAW: u32 = 3;
const WIN: u32 = 6;

fn process_match(opponent_move: char, player_move: char) -> u32 {
    let point_for_player_move = if player_move == 'X' {
        1
    } else if player_move == 'Y' {
        2
    } else if player_move == 'Z' {
        3
    } else {
        panic!("Unknown player_move: {player_move}")
    };

    let match_result = if player_move == 'X' {
        // Rock + Rock
        if opponent_move == 'A' {
            DRAW

        // Rock + Paper
        } else if opponent_move == 'B' {
            LOSE

        // Rock + Scissors
        } else if opponent_move == 'C' {
            WIN
        } else {
            panic!("Unknown opponent_move")
        }
    } else if player_move == 'Y' {
        // Paper + Rock
        if opponent_move == 'A' {
            WIN

        // Paper + Paper
        } else if opponent_move == 'B' {
            DRAW

        // Paper + Scissors
        } else if opponent_move == 'C' {
            LOSE
        } else {
            panic!("Unknown opponent_move")
        }
    } else if player_move == 'Z' {
        // Scissors + Rock
        if opponent_move == 'A' {
            LOSE

        // Scissors + Paper
        } else if opponent_move == 'B' {
            WIN

        // Scissors + Scissors
        } else if opponent_move == 'C' {
            DRAW
        } else {
            panic!("Unknown opponent_move")
        }
    } else {
        panic!("Unknown player_move")
    };

    point_for_player_move + match_result
}

#[cfg(test)]
mod tests {
    use super::solve;
    use std::fs;

    #[test]
    fn example() {
        let input = fs::read_to_string("./inputs/day_02_example.txt")
            .expect("couldn't find file ./inputs/day_02_example.txt");

        assert_eq!(solve(&input), 15);
    }

    #[test]
    fn given_input() {
        let input = fs::read_to_string("./inputs/day_02_input.txt")
            .expect("couldn't find file ./inputs/day_02_input.txt");

        assert_eq!(solve(&input), 12458);
    }
}
