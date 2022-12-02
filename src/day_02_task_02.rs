use core::panic;

#[allow(dead_code)]

pub fn solve(input: &str) -> u32 {
    input
        .split('\n')
        .map(|round| {
            let res = round.split(' ').collect::<Vec<&str>>();
            let opponent_move = res.first().unwrap().chars().next().unwrap();
            let expected_results = res.get(1).unwrap().chars().next().unwrap();
            process_match(opponent_move, make_guess(opponent_move, expected_results))
        })
        .sum()
}

const LOSE_POINTS: u32 = 0;
const DRAW_POINTS: u32 = 3;
const WIN_POINTS: u32 = 6;

const LOSE: char = 'X';
const DRAW: char = 'Y';
const WIN: char = 'Z';

const ROCK: char = 'A';
const PAPER: char = 'B';
const SCISSORS: char = 'C';

fn make_guess(opponent_move: char, expected_result: char) -> char {
    if expected_result == WIN {
        if opponent_move == ROCK {
            return PAPER;
        }
        if opponent_move == PAPER {
            return SCISSORS;
        }
        if opponent_move == SCISSORS {
            return ROCK;
        }

        panic!("Unexpected opponent_move {opponent_move}")
    }

    if expected_result == DRAW {
        if opponent_move == ROCK {
            return ROCK;
        }
        if opponent_move == PAPER {
            return PAPER;
        }
        if opponent_move == SCISSORS {
            return SCISSORS;
        }

        panic!("Unexpected opponent_move {opponent_move}")
    }

    if expected_result == LOSE {
        if opponent_move == ROCK {
            return SCISSORS;
        }
        if opponent_move == PAPER {
            return ROCK;
        }
        if opponent_move == SCISSORS {
            return PAPER;
        }

        panic!("Unexpected opponent_move {opponent_move}")
    }

    panic!("Unexpected expected_result")
}

fn process_match(opponent_move: char, player_move: char) -> u32 {
    let point_for_player_move = if player_move == ROCK {
        1
    } else if player_move == PAPER {
        2
    } else if player_move == SCISSORS {
        3
    } else {
        panic!("Unknown player_move: {player_move}")
    };

    let match_result = if player_move == ROCK {
        if opponent_move == ROCK {
            DRAW_POINTS
        } else if opponent_move == PAPER {
            LOSE_POINTS
        } else if opponent_move == SCISSORS {
            WIN_POINTS
        } else {
            panic!("Unknown opponent_move, {opponent_move}")
        }
    } else if player_move == PAPER {
        if opponent_move == ROCK {
            WIN_POINTS
        } else if opponent_move == PAPER {
            DRAW_POINTS
        } else if opponent_move == SCISSORS {
            LOSE_POINTS
        } else {
            panic!("Unknown opponent_move, {opponent_move}")
        }
    } else if player_move == SCISSORS {
        if opponent_move == ROCK {
            LOSE_POINTS
        } else if opponent_move == PAPER {
            WIN_POINTS
        } else if opponent_move == SCISSORS {
            DRAW_POINTS
        } else {
            panic!("Unknown opponent_move, {opponent_move}")
        }
    } else {
        panic!("Unknown player_move {player_move}")
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

        assert_eq!(solve(&input), 12);
    }

    #[test]
    fn given_input() {
        let input = fs::read_to_string("./inputs/day_02_input.txt")
            .expect("couldn't find file ./inputs/day_02_input.txt");

        assert_eq!(solve(&input), 12683);
    }
}
