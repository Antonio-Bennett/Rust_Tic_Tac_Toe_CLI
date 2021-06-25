use tic_tac_toe::*;

fn main() {
    let (mut player1, mut player2, mut board) = game_setup();

    let mut game_over = false;
    let mut curr_player = &player1;

    //Initial display afterwards board printed in modification so that it will always be in latest updated state
    println!("\n{}", board);

    loop {
        if game_over {
            if restart() {
                //Destructuring assignments are unstable currently
                let (temp, temp1, temp2) = game_setup();
                player1 = temp;
                player2 = temp1;
                board = temp2;
                game_over = false;
                curr_player = &player1;
                println!("\n{}", board);
            } else {
                println!("\nThanks for playing!!!");
                break;
            }
        }

        if game_input(&mut board, &curr_player, &mut game_over) {
            if curr_player.order == 1 {
                curr_player = &player2;
            } else {
                curr_player = &player1;
            }
        }
    }
}
