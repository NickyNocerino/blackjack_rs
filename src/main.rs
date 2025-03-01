use blackjack_rs::game::BlackJackGame;
use blackjack_rs::opt_bj_game::OptimizedBlackJackGame;

use std::fs;
use std::time::Instant;

fn main() {
    // clean up old cache values
    fs::create_dir_all("bin/").expect("");
    fs::remove_dir_all("bin/").expect("");
    fs::create_dir_all("bin/").expect("");

    let mut bj_game = OptimizedBlackJackGame::new_standard(1);
    //bj_game.print_contents();
    bj_game = bj_game.deal();
    //bj_game = bj_game.hit();
    //bj_game = bj_game.stay();
    let now = Instant::now();
    bj_game.print_contents();
    println!("computation to {} seconds", now.elapsed().as_secs_f64())
    

}
