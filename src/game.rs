use game_pieces_rs::card::Card;
use game_pieces_rs::deck::Deck;

pub struct BlackJackGame {
    hand: Vec<Card>,
    dealer: Vec<Card>,
    deck: Deck,
}