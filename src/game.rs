use game_pieces_rs::card::{Suit, Rank, Card};
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
            deck:Deck::new_empty(),
        }
    }

    pub fn new_standard(num_decks:usize) -> Self {
        let mut deck_list = Vec::<Card>::new();
        let suit_list = vec![Suit::Spades, Suit::Clubs, Suit::Diamonds, Suit::Hearts];
        let rank_list = vec![Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King];
        for _i in 0..num_decks {
            for rank in rank_list.iter(){
                for suit in suit_list.iter() {
                    deck_list.push(Card::new(*rank, *suit));
                }
            }
        }
        let deck = Deck::from_vec(&deck_list);

        Self {
            hand:Vec::<Card>::new(),
            dealer:Vec::<Card>::new(),
            deck:deck,
        }
    }

    pub fn deal(&self) -> Self {
        if self.deck.count < 3 {
            panic!("dealing from deck without enough cards")
        }
        let mut hand = Vec::<Card>::new();
        let mut dealer = Vec::<Card>::new();
        let (mut card, mut updated_deck) = self.deck.draw();
        hand.push(card);
        (card, updated_deck) = updated_deck.draw();
        hand.push(card);
        (card, updated_deck) = updated_deck.draw();
        dealer.push(card);
        // uncomment for 2 dealer cards revealed
        // (card, updated_deck) = updated_deck.draw();
        // dealer.push(card);
        Self {
            hand:hand,
            dealer:dealer,
            deck:updated_deck,
        }
    }

    pub fn hit(&self) -> Self {
        if self.deck.count < 1 {
            panic!("hitting from deck without enough cards")
        }
        let mut hand = self.hand.clone();
        let mut dealer = self.dealer.clone();
        let (mut card, mut updated_deck) = self.deck.draw();
        hand.push(card);
        Self {
            hand:hand,
            dealer:dealer,
            deck:updated_deck,
        }
    }

    pub fn print_contents(&self) {
        println!("Hand: {:?}", self.hand );
        println!("Dealer: {:?}", self.dealer );
        println!("Deck suit odds: {:?}", self.deck.draw_probs_by_suit() );
        println!("Deck rank odds: {:?}", self.deck.draw_probs_by_rank() );
        println!("Deck blackjack odds: {:?}", self.deck.draw_probs_by_blackjack_value() );

    }
}