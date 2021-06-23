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
}

impl Board {
    pub fn new() -> Self {
        let mut board = ['\0'; 9];
        for num in 1..=9 {
            board[num - 1] = char::from_digit(num as u32, 10).unwrap();
        }
        Self { board }
    }

    pub fn check_game(&self) -> bool {
        todo!()
    }

    pub fn modify_board(&mut self, pos: usize, character: char) {
        self.board[pos - 1] = character;
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
    match board.check_game() {
        false => {
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
        }
        true => {
            *game_over = true;
        }
    }
}
