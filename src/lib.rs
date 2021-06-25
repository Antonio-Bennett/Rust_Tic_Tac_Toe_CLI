use std::{char, fmt, io};

pub struct Player {
    pub name: String,
    pub order: u8,
}

impl Player {
    fn new(name: String, order: u8) -> Self {
        Self {
            name: name.to_string(),
            order,
        }
    }
}

pub struct Board {
    board: [char; 9],
    winning_states: [[usize; 3]; 8],
    moves_played: u8,
}

impl Board {
    pub fn new() -> Self {
        let mut board = ['\0'; 9];
        for num in 1..=9 {
            board[num - 1] = char::from_digit(num as u32, 10).unwrap();
        }

        let winning_states = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        let moves_played: u8 = 0;

        Self {
            board,
            winning_states,
            moves_played,
        }
    }

    pub fn check_game(&self, order: u8) -> Option<bool> {
        if self.moves_played == 9 {
            return None;
        }

        let ch;

        if order == 1 {
            ch = 'X';
        } else {
            ch = 'O';
        }

        for i in self.winning_states.iter() {
            if self.board[i[0]] == ch && self.board[i[1]] == ch && self.board[i[2]] == ch {
                return Some(true);
            }
        }

        Some(false)
    }

    pub fn modify_board(&mut self, pos: usize, character: char) {
        self.moves_played += 1;
        self.board[pos - 1] = character;
        println!("{}", self);
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "  {}  |  {}  |  {}  \n  {}  |  {}  |  {}  \n  {}  |  {}  |  {}  \n",
            self.board[0],
            self.board[1],
            self.board[2],
            self.board[3],
            self.board[4],
            self.board[5],
            self.board[6],
            self.board[7],
            self.board[8]
        )
    }
}

pub fn game_setup() -> (Player, Player) {
    let mut player1 = String::new();
    let mut player2 = String::new();

    println!("Enter Player 1 name: ");
    io::stdin()
        .read_line(&mut player1)
        .expect("Could not read player 1 name");

    println!("Enter Player 2 name: ");
    io::stdin()
        .read_line(&mut player2)
        .expect("Could not read player 2 name");

    let player1 = player1.trim().to_owned();
    let player2 = player2.trim().to_owned();

    let player1 = Player::new(player1, 1);
    let player2 = Player::new(player2, 2);

    (player1, player2)
}

pub fn game_input(board: &mut Board, player: &Player, game_over: &mut bool) {
    /* match board.check_game(player.order) {
    Some(false) => { */
    println!("{} choose your move: [1-9]", player.name);
    let mut position = String::new();
    io::stdin()
        .read_line(&mut position)
        .expect("Couldn't read position choice");

    let position = position.trim().parse::<usize>().unwrap();
    let character: char;

    if player.order == 1 {
        character = 'X';
    } else {
        character = 'O';
    }

    board.modify_board(position, character);
    if let Some(result) = board.check_game(player.order) {
        if result {
            *game_over = true;
            println!("Congrats! {} won the game!", player.name);
        }
    } else {
        *game_over = true;
        println!("It was a draw!");
    }
    // }
    // Some(true) => {
    // }
    // None => println!("It was a draw!"),
    // }
}
