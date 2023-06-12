use std::fs;

#[derive(Clone, Copy)]
enum HandState {
    Rock,
    Paper,
    Scissors,
}


impl HandState {
    fn compare(&self, other_state: &HandState) -> TurnsOutcome {
        match (self, other_state) {
            (HandState::Rock, HandState::Rock)
            | (HandState::Paper, HandState::Paper)
            | (HandState::Scissors, HandState::Scissors) => TurnsOutcome::Draw,

            (HandState::Rock, HandState::Paper)
            | (HandState::Paper, HandState::Scissors)
            | (HandState::Scissors, HandState::Rock)  => TurnsOutcome::Lost,

            (HandState::Rock, HandState::Scissors)
            | (HandState::Paper, HandState::Rock) 
            | (HandState::Scissors, HandState::Paper)  => TurnsOutcome::Win,
        }
    }

    fn get_hand_state_for_outcome(&self, outcome: &TurnsOutcome) -> HandState{
        match outcome{
            TurnsOutcome::Win => {
                match self{
                            HandState::Rock => HandState::Paper,
                            HandState::Paper => HandState::Scissors,
                            HandState::Scissors => HandState::Rock,
                        }
            },
            TurnsOutcome::Lost => {
                match self{
                            HandState::Rock => HandState::Scissors,
                            HandState::Paper => HandState::Rock,
                            HandState::Scissors => HandState::Paper,
                        }
            },
            TurnsOutcome::Draw => self.clone()
            
        }
    }

    fn build_from_str(input_char: &char) -> Self{
        let deref_input_char = *input_char;
        if deref_input_char == 'A' {
            return Self::Rock;
        }else if deref_input_char == 'B' {
            return Self::Paper;
        }else if deref_input_char == 'C' {
            return Self::Scissors;
        }
        panic!("Can't a valid input letter")
    }
}

enum TurnsOutcome {
    Win,
    Lost,
    Draw,
}

impl TurnsOutcome{
    fn build_from_str(input_char: &char) -> Self{
        let deref_input_char = *input_char;
        if deref_input_char == 'Z'{
            return Self::Win
        }
        else if deref_input_char == 'Y' {
            return Self::Draw
        }
        else if deref_input_char == 'X' {
            return Self::Lost
        }
        panic!("Can't parse a valid turn out come ");
    }
}

struct SingleTurn {
    opponents_choice: HandState,
    expected_outcome: TurnsOutcome,
}

impl SingleTurn {
    fn caluculate_turn_score(&self) -> u16 {
        let mut score = 0;
        let players_choice = self.opponents_choice.get_hand_state_for_outcome(&self.expected_outcome);
        match players_choice {
            HandState::Rock => score += 1,
            HandState::Paper => score += 2,
            HandState::Scissors => score += 3,
        }

        match self.expected_outcome {
            TurnsOutcome::Lost => score += 0,
            TurnsOutcome::Draw => score += 3,
            TurnsOutcome::Win => score += 6,
        }

        score
    }

    fn build_turn_from_str(line: &str) -> Self {
        let trimed_line = line.trim().replace(" ", "");
        let mut trimed_line_chars = trimed_line.chars();
        let opponents_choice_str = &trimed_line_chars.nth(0).unwrap(); // reading first char and remove it 
        let expected_outcome_str = &trimed_line_chars.nth(0).unwrap(); // read 2nd char and remove it 
        let opponents_choice = HandState::build_from_str(opponents_choice_str);
        let expected_outcome = TurnsOutcome::build_from_str(expected_outcome_str);
        Self{
            opponents_choice,
            expected_outcoame
        }
    }
}

fn parse_file(contents: &str) -> Vec<SingleTurn> {
    let mut turns = Vec::new();
    for line in contents.lines() {
        let turn = SingleTurn::build_turn_from_str(&line);
        turns.push(turn);
    }
    turns
}

pub fn run() {
    let contents = fs::read_to_string("./src/advent_of_code/inputs/input_2.txt").unwrap();
    let player_game = parse_file(&contents);

    let score = player_game.iter().fold(0, |previous_value, current_turn|{
        let score = current_turn.caluculate_turn_score();
        previous_value + score
    });

    print!("{}", score);
}
