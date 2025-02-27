use game_pieces_rs::card::Card;
use game_pieces_rs::deck::Deck;

pub struct BlackJackGame {
    hand: Vec<Card>,
    dealer: Vec<Card>,
    deck: Deck,
}

impl BlackJackGame {
    pub fn new_empty() -> Self {
        Self{
            hand:Vec::<Card>::new(),
            dealer:Vec::<Card>::new(),
            deck:Deck:new_empty(),
        }
    }

    pub new_standard(num_decks:usize) -> Self {

        let mut deck_list = Vec::<Card>::new();
        let suit_list = vec![Suit::Spades, Suit::Clubs, Suit::Diamonds, Suit::Hearts];
        let rank_list = vec![Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King];
        for i in 0..num_decks {
            for rank in rank_list.iter(){
                for suit in suit_list.iter() {
                    deck_list.push(Card::new(*rank, *suit));
                }
            }
        }
        let mut deck = Deck::from_vec(&deck_list);

        Self {
            hand:Vec::<Card>::new(),
            dealer:Vec::<Card>::new(),
            deck:deck,
        }
    }

    pub fn deal(&self) -> Self {
        let mut deck = self.deck.clone();
        let mut hand = Vec::<Card>::new();
        let mut dealer = Vec::<Card>::new(),
        let mut (card, updated_deck) = deck.draw();
        hand.push(card);
        (card, updated_deck) = updated_deck.draw();
        hand.push(card);
        (card, updated_deck) = updated_deck.draw();
        dealer.push(card);
        (card, updated_deck) = updated_deck.draw();
        dealer.push(card);

        Self {
            hand:hand,
            dealer:dealer,
            deck:deck,
        }
    }
}