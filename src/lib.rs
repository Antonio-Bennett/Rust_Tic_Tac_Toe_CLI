use std::{char, fmt, io};

//Player struct that holds the name of the player and the order they play in
pub struct Player {
    pub name: String,
    pub order: u8,
}

impl Player {
    fn new(name: String, order: u8) -> Self {
        Self { name, order }
    }
}

//The board - important thing is winning states holds the possible combos of winning which gives the check O(n) time, moves played help to calculate a draw
pub struct Board {
    board: [char; 9],
    winning_states: [[usize; 3]; 8],
    moves_played: u8,
}

impl Board {
    pub fn new() -> Self {
        let mut board = ['\0'; 9];
        for num in 1..=9 {
            //Initializes positions as nums 1-9 to make it easier for players to understand locations
            board[num - 1] = char::from_digit(num as u32, 10).unwrap();
        }

        //This holds the posible indexes of winning moves
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

        //When this reaches 9 we know the game is drawn
        let moves_played: u8 = 0;

        Self {
            board,
            winning_states,
            moves_played,
        }
    }

    //Due to winning states field on struct this is an O(n) time check with n being number of winning positions
    pub fn check_game(&self, order: u8) -> Option<bool> {
        //Player order is passed so we know which character to check for a win
        let ch = if order == 1 { 'X' } else { 'O' };

        //Iterate through the possible winning combos and check if the state of the board has the character in
        //these positions
        for i in self.winning_states.iter() {
            if self.board[i[0]] == ch && self.board[i[1]] == ch && self.board[i[2]] == ch {
                return Some(true);
            }
        }

        //If 9 moves are played the board is filled and it is a draw
        //The player might win on the 9th move so we have to check for win first
        if self.moves_played == 9 {
            return None;
        }

        //If no winninh position is found then the game continues
        Some(false)
    }

    pub fn modify_board(&mut self, pos: usize, character: char) -> bool {
        //This is used in main game loop to know if current player should be switched since board changed
        let modified;

        //Checks if positions selected is already filled otherwise modify position and increment moves played
        if self.board[pos - 1] == 'X' {
            println!("\nX is already played at that position");
            modified = false;
        } else if self.board[pos - 1] == 'O' {
            println!("\nO is already played at that position");
            modified = false;
        } else {
            self.board[pos - 1] = character;
            self.moves_played += 1;
            modified = true;
        }

        //The board is printed after being modified
        println!("\n{}", self);

        modified
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

//Implented Display trait so board is easily printed
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

//Initializes game pretty self explanatory
pub fn game_setup() -> (Player, Player, Board) {
    let mut player1 = String::new();
    let mut player2 = String::new();

    println!("\nEnter Player 1 name: ");
    io::stdin()
        .read_line(&mut player1)
        .expect("Could not read player 1 name");

    println!("\nEnter Player 2 name: ");
    io::stdin()
        .read_line(&mut player2)
        .expect("Could not read player 2 name");

    let player1 = player1.trim().to_owned();
    let player2 = player2.trim().to_owned();

    let player1 = Player::new(player1, 1);
    let player2 = Player::new(player2, 2);

    println!(
        "\nPlayer 1 is {} and Player 2 is {}",
        player1.name, player2.name
    );

    (player1, player2, Board::default())
}

//Confirms if game should be reinitialized after ending
pub fn restart() -> bool {
    let mut ans = String::new();
    println!("\nWould you like to restart the game? Y or N");

    io::stdin()
        .read_line(&mut ans)
        .expect("Could not read restart decision");

    ans = ans.trim().to_uppercase();

    if ans == "N" {
        return false;
    }

    true
}

//This is the main function in the game loop
pub fn game_input(board: &mut Board, player: &Player, game_over: &mut bool) -> bool {
    //Take players chosen square to play move
    println!("{} choose your move: [1-9]", player.name);
    let mut position = String::new();
    io::stdin()
        .read_line(&mut position)
        .expect("Couldn't read position choice");

    //The position they chose and the coresponding character based on players order is defined
    let position = position.trim().parse::<usize>().unwrap();

    let character = if player.order == 1 { 'X' } else { 'O' };

    //If the board is modified then other player gets to play otherwise current player moves again
    let modified = board.modify_board(position, character);

    //Check if the game is won, drawn etc after the board is modified
    if let Some(result) = board.check_game(player.order) {
        if result {
            *game_over = true;
            println!("Congrats! {} won the game!", player.name);
        }
    } else {
        *game_over = true;
        println!("It was a draw!");
    }

    //This is used inside loop to see if current player should go again
    modified
}
