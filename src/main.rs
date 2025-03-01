use blackjack_rs::game::BlackJackGame;

use std::fs;

fn main() {
    // clean up old cache values
    fs::create_dir_all("bin/").expect("");
    fs::remove_dir_all("bin/").expect("");
    fs::create_dir_all("bin/").expect("");

    let mut bj_game = BlackJackGame::new_standard(1);
    bj_game.print_contents();
    bj_game = bj_game.deal();
    //bj_game = bj_game.hit();
    //bj_game = bj_game.stay();
    bj_game.print_contents();

}
