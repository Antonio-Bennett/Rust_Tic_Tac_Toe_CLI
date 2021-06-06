use tic_tac_toe::*;

fn main() {
    let (player1, player2) = menu_input();
    println!(
        "Player 1 is {} and Player 2 is {}",
        player1.name, player2.name
    );
}
