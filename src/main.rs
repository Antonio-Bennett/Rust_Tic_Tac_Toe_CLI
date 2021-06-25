use tic_tac_toe::*;

fn main() {
    let (player1, player2) = game_setup();
    println!(
        "Player 1 is {} and Player 2 is {}",
        player1.name, player2.name
    );

    let mut board = Board::new();

    let mut game_over = false;
    let mut curr_player = &player1;

    //Initial display afterwards board printed in modification so that it will always be in latest updated state
    println!("{}", board);

    loop {
        game_input(&mut board, &curr_player, &mut game_over);
        if game_over {
            break;
        }
        if curr_player.order == 1 {
            curr_player = &player2;
        } else {
            curr_player = &player1;
        }
    }
}
