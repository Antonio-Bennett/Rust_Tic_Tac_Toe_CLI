use std::io;

enum PlayerOrder {
    One,
    Two,
}

pub struct Player {
    pub name: String,
    order: PlayerOrder,
}

impl Player {
    fn new(name: String, num: u8) -> Self {
        let order;
        if num == 1 {
            order = PlayerOrder::One;
        } else {
            order = PlayerOrder::Two;
        }
        Self {
            name: name.to_string(),
            order,
        }
    }
}

pub fn menu_input() -> (Player, Player) {
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

    let player1 = player1.trim();
    let player2 = player2.trim();

    let player1 = Player::new(player1.to_owned(), 1);
    let player2 = Player::new(player2.to_owned(), 2);

    (player1, player2)
}

pub fn input(_player: Player) -> Player {
    Player::new("test".to_owned(), 1)
}
