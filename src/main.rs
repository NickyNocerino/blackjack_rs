use game_pieces_rs::card::Card;
use blackjack_rs::game::BlackJackGame;

fn main() {
    let mut bj_game = BlackJackGame::new_standard(8);
    bj_game.print_contents();
    bj_game = bj_game.deal();
    //bj_game = bj_game.hit();
    //bj_game = bj_game.stay();
    bj_game.print_contents();

}
