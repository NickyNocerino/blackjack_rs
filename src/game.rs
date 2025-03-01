use game_pieces_rs::card::{Suit, Rank, Card};
use game_pieces_rs::deck::Deck;

pub struct BlackJackGame {
    hand: Vec<Card>,
    dealer: Vec<Card>,
    stay: bool,
    deck: Deck,
}

impl BlackJackGame {
    pub fn new_empty() -> Self {
        Self{
            hand:Vec::<Card>::new(),
            dealer:Vec::<Card>::new(),
            stay:false,
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
            stay:false,
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
            stay:false,
            deck:updated_deck,
        }
    }

    pub fn hit(&self) -> Self {
        if self.deck.count < 1 {
            panic!("hitting from deck without enough cards")
        }
        let mut hand = self.hand.clone();
        let dealer = self.dealer.clone();
        let (card, updated_deck) = self.deck.draw();
        hand.push(card);
        Self {
            hand:hand,
            dealer:dealer,
            stay:false,
            deck:updated_deck,
        }
    }

    pub fn dealer_hit(&self) -> Self {
        if self.deck.count < 1 {
            panic!("dealer hitting from deck without enough cards")
        }
        let hand = self.hand.clone();
        let mut dealer = self.dealer.clone();
        let (card, updated_deck) = self.deck.draw();
        dealer.push(card);
        Self {
            hand:hand,
            dealer:dealer,
            stay:false,
            deck:updated_deck,
        }
    }

    pub fn stay(&self) -> Self{
        Self {
            hand:self.hand.clone(),
            dealer:self.dealer.clone(),
            stay:true,
            deck:self.deck.clone()
        }
    }

    pub fn get_hand_value(&self) -> usize {
        let mut value:usize = 0;
        let mut aces_count:usize = 0;
        for card in self.hand.iter(){
            if card.get_blackjack_value_index() == 0 {
                aces_count += 1;
            }
            value += card.get_blackjack_value_index() + 1
        }
        for _i in 0..aces_count {
            if value + 10 <= 21 {
                value += 10;
            }
        }
        value
    }

    pub fn get_dealer_value(&self) -> usize {
        let mut value:usize = 0;
        let mut aces_count:usize = 0;
        for card in self.dealer.iter(){
            if card.get_blackjack_value_index() == 0 {
                aces_count += 1;
            }
            value += card.get_blackjack_value_index() + 1
        }
        for _i in 0..aces_count {
            if value + 10 <= 21 {
                value += 10;
            }
        }
        value
    }

    pub fn is_hand_bust(&self) -> bool {
        self.get_hand_value() > 21
    }

    pub fn is_dealer_bust(&self) -> bool {
        self.get_dealer_value() > 21
    }

    pub fn is_hand_blackjack(&self) -> bool {
        self.get_hand_value() == 21 && self.hand.len() == 2
    }

    pub fn is_dealer_blackjack(&self) -> bool {
        self.get_dealer_value() == 21 && self.hand.len() == 2
    }

    pub fn get_hit_expected_value(&self) -> f64 {
        let mut expected_value:f64 = 0.0;
        let draw_probs = self.deck.draw_probs_by_blackjack_value();
        let mut hand_clone:Vec<Card>;                
        let mut drawn_card:Card;
        let mut drawn_deck:Deck;
        let mut drawn_game:Self;
        for i in 0usize..10usize {
            //println!("Hit blackjack index {:?} draw probability = {:?}", i, draw_probs[i] );
            if draw_probs[i] > 0.0 {
                (drawn_card, drawn_deck) = self.deck.draw_blackjack_value_index(i);
                hand_clone = self.hand.clone();
                hand_clone.push(drawn_card);
                drawn_game = Self {
                    hand:hand_clone,
                    dealer:self.dealer.clone(),
                    stay:false,
                    deck:drawn_deck,
                };
                
                expected_value += draw_probs[i] * drawn_game.get_expected_value();
            }
        }
        return expected_value;
    }

    pub fn get_stay_expected_value(&self) -> f64 {
        let stay_game = Self{
            hand:self.hand.clone(),
            dealer:self.dealer.clone(),
            stay: true,
            deck: self.deck.clone()
        };
        stay_game.get_expected_value()
    }

    pub fn get_expected_value(&self) -> f64 {
        // if has not been dealt, calc for each different possible deal
        if self.hand.len() == 0 && self.dealer.len() == 0 {
            return -2.0;
        }

        // if dealer has blackjack, you cannot win
        if self.is_dealer_blackjack() {
            // you can push though
            if self.is_hand_blackjack() {
                return 0.0;
            }
            return -1.0;
        }

        if self.is_hand_bust() {
            return -1.0
        }

        if self.is_dealer_bust() {
            if self.is_hand_blackjack() {
                return 1.5;
            }
            return 1.0
        }

        // if stay, check for payouts
        if self.stay {
            if self.is_hand_blackjack() {
                return 1.5;
            }

            if self.get_dealer_value() > self.get_hand_value() {
                return -1.0
            }

            // dealer stays on >=17
            if self.get_dealer_value() > 16 {
                if self.get_dealer_value() == self.get_hand_value() {
                    return 0.0;
                }
                else if self.get_dealer_value() > self.get_hand_value() {
                    return -1.0;
                }
                else if self.get_dealer_value() < self.get_hand_value() {
                    return 1.0;
                }
            }
            else {
                let mut expected_value:f64 = 0.0;
                let draw_probs = self.deck.draw_probs_by_blackjack_value();
                let mut dealer_clone:Vec<Card>;                
                let mut drawn_card:Card;
                let mut drawn_deck:Deck;
                let mut drawn_game:Self;
                for i in 0usize..10usize {
                    if draw_probs[i] > 0.0 {
                        (drawn_card, drawn_deck) = self.deck.draw_blackjack_value_index(i);
                        dealer_clone = self.dealer.clone();
                        dealer_clone.push(drawn_card);
                        drawn_game = Self {
                            hand:self.hand.clone(),
                            dealer:dealer_clone,
                            stay:true,
                            deck:drawn_deck,
                        };
                        expected_value += draw_probs[i] * drawn_game.get_expected_value();
                    }
                }
                return expected_value;
            }
        }
        else {
            //if not stay, determine expected value for hit or stay, return > value
            let stay_ev = self.get_stay_expected_value();
            let hit_ev =self.get_hit_expected_value();
            return f64::max(stay_ev, hit_ev);
        }
        panic!("THIS SHOULD NEVER BE REACHED")

    }

    pub fn print_contents(&self) {
        println!("Hand: {:?}, Value: {:?}, Bust: {:?}", self.hand, self.get_hand_value(), self.is_hand_bust());
        println!("Dealer: {:?}, Value: {:?}, Bust: {:?}", self.dealer, self.get_dealer_value(), self.is_dealer_bust());
        println!("Stay: {:?}", self.stay);
        // println!("Deck suit odds: {:?}", self.deck.draw_probs_by_suit());
        // println!("Deck rank odds: {:?}", self.deck.draw_probs_by_rank());
        // println!("Deck blackjack odds: {:?}", self.deck.draw_probs_by_blackjack_value() );
        println!("Position EV: {:?}", self.get_expected_value());
        if !self.stay && self.hand.len() != 0 && self.dealer.len() != 0 {
            println!("Hit EV: {:?}", self.get_hit_expected_value());
            println!("Stay EV: {:?}", self.get_stay_expected_value());
        }

    }
}