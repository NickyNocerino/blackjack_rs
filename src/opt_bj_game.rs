use game_pieces_rs::blackjack_deck::BlackjackDeck;
use game_pieces_rs::card::{Suit, Rank, Card};

use std::time::{Instant};
use std::fs;
use std::path::Path;


pub struct OptimizedBlackJackGame {
    hand: Vec<usize>,
    dealer: Vec<usize>,
    stay: bool,
    deck: BlackjackDeck,
}

impl OptimizedBlackJackGame {
    pub fn new_empty() -> Self {
        Self{
            hand:Vec::<usize>::new(),
            dealer:Vec::<usize>::new(),
            stay:false,
            deck:BlackjackDeck::new_empty(),
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
        let deck = BlackjackDeck::from_vec(&deck_list);

        Self {
            hand:Vec::<usize>::new(),
            dealer:Vec::<usize>::new(),
            stay:false,
            deck:deck,
        }
    }

    pub fn unique_key(&self) -> String {
        let count = self.deck.count;
        let mut count_by_blackjack_value = [0usize;10];
        count_by_blackjack_value.clone_from_slice(&self.deck.count_by_blackjack_value);
        let bjvc_string = count_by_blackjack_value.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("-");
        let mut hand = self.hand.clone();
        hand.sort();
        let hand_index_string = hand.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("-");
        let mut dealer = self.dealer.clone();
        dealer.sort();
        let dealer_index_string = dealer.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("-");
        let stay_string =self.stay.to_string();
        format!("[c={count}cbji=[{bjvc_string}]h=[{hand_index_string}]d=[{dealer_index_string}]s={stay_string}]")
    }
    
    pub fn write_bin_file_cache(&self, key:String, ev:f64) {
        let filepath = format!("bin/{key}.data");
        fs::write(filepath, ev.to_be_bytes()).expect("cannot write to file");
    }

    pub fn read_bin_file_cache(&self, key:String) -> Option<f64>{
        let filepath = format!("bin/{key}.data");
        if Path::new(&filepath).exists() {
            let bytes = fs::read(&filepath).expect("could not read file");
            let bytes_array = bytes.clone().try_into().unwrap_or_else(|v: Vec<u8>| panic!("Expected a Vec of length {} but it was {}", 8, v.len()));
            let ev:f64 = f64::from_be_bytes(bytes_array);
            return Some(ev);
        }
        return None;
    }

    pub fn deal(&self) -> Self {
        if self.deck.count < 3 {
            panic!("dealing from deck without enough cards")
        }

        if self.hand.len() > 0 || self.dealer.len() > 0 {
            panic!("dealing to an already dealt game")
        }
        let mut hand = Vec::<usize>::new();
        let mut dealer = Vec::<usize>::new();
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
            if *card == 0 {
                aces_count += 1;
            }
            value += *card + 1;
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
            if *card == 0 {
                aces_count += 1;
            }
            value += *card + 1;
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
        let mut hand_clone:Vec<usize>;                
        let mut drawn_card:usize;
        let mut drawn_deck:BlackjackDeck;
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

    pub fn get_deck_expected_value(&self) -> f64 {
        let mut expected_value:f64 = 0.0;
        let draw_probs = self.deck.draw_probs_by_blackjack_value();
        let mut dealer_clone:Vec<usize>;                
        let mut drawn_card:usize;
        let mut drawn_deck:BlackjackDeck;
        let mut drawn_game:Self;
        for i in 0usize..10usize {
            if draw_probs[i] > 0.0 {
                (drawn_card, drawn_deck) = self.deck.draw_blackjack_value_index(i);
                dealer_clone = self.dealer.clone();
                dealer_clone.push(drawn_card);
                drawn_game = Self {
                    hand:self.hand.clone(),
                    dealer:dealer_clone,
                    stay:false,
                    deck:drawn_deck,
                };
                expected_value += draw_probs[i] * drawn_game.get_hit_expected_value();
            }
        }
        return expected_value;
    }

    pub fn get_expected_value(&self) -> f64 {
        let unique_key = self.unique_key();
        let cached_ev = self.read_bin_file_cache(unique_key.clone());
        match cached_ev{
            Some(x) => {
                println!("got cached ev for {} = {} ", unique_key.clone(), x);
                return x;
            }
            None => {}
        }
        let now = Instant::now();

        // if has not been dealt, calc for each different possible deal
        if self.hand.len() == 0 && self.dealer.len() == 0 {
            return self.get_deck_expected_value();
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
                let mut dealer_clone:Vec<usize>;                
                let mut drawn_card:usize;
                let mut drawn_deck:BlackjackDeck;
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
                if now.elapsed().as_secs() > 5 {
                    println!("caching {} = {}, it took {} seconds to complete", unique_key.clone(), expected_value,  now.elapsed().as_secs());
                    self.write_bin_file_cache(unique_key, expected_value);
                }
                return expected_value;
            }
        }
        else {
            //if not stay, determine expected value for hit or stay, return > value
            let stay_ev = self.get_stay_expected_value();
            let hit_ev =self.get_hit_expected_value();
            let expected_value = f64::max(stay_ev, hit_ev);
            if now.elapsed().as_secs() > 5 {
                println!("caching {} = {}, it took {} seconds to complete", unique_key.clone(), expected_value, now.elapsed().as_secs());
                self.write_bin_file_cache(unique_key.clone(), expected_value);
            }
            return expected_value;
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
        // if !self.stay && self.hand.len() != 0 && self.dealer.len() != 0 {
        //     println!("Hit EV: {:?}", self.get_hit_expected_value());
        //     println!("Stay EV: {:?}", self.get_stay_expected_value());
        // }
        println!("Position EV: {:?}", self.get_expected_value());

    }
}