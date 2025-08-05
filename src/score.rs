use crate::card::{Card, CardSet, CardSuit, CardValue, NUM_CARD_VALUES, NUM_SUITS};

// THINGS IN THIS MODULE:
//
// 1. actual score value of cards i think
//
// 2. structure representing like a "move" (we want to output an "all possible moves"
// for a given hand)
//
// 3. structure of the entire area of "played cards"..? and perhaps with this comes some
// way to check the legality of "move"s
//
// actually ya the hand's "all possible moves" function is gonna need as input the whole played
// cards area
// omfg wait its also gonna need the discard pile!!!! which is fine it can just be a list of
// cards....... but this will mess with the fun calculation stuff i wanted 2 do with the hand.
// o well

pub struct Play {
    pub kind: PlayKind,
    pub cards_used: CardSet,

    // Cards that player will take on as part of doing this play
    // if they use a card past the top of the discard stack
    pub cards_acquired: CardSet,
}

#[derive(Debug)]
pub enum PlayKind {
    StraightFlush,
    Multiple,
}

pub struct PlayedCards {
    pub straight_flush_played: CardSet,
    pub multiple_played: CardSet,
}

pub struct PlayMetadata {
    pub player_index: usize,
}

pub fn all_possible_plays(
    hand: &CardSet,
    discard_pile: &Vec<Card>,
    played_cards: &PlayedCards,
) -> Vec<Play> {
    let playable_cards = playable_cards(hand, discard_pile);
    let mut plays: Vec<Play> = vec![];

    // 1. Multiples
    for value in CardValue::iter() {
        // for each V
        // check number of cards we have with value V
        // if 1: check if theres a V in played multiple cat 
        // if 2: cant play anything
        // if 3: can play!! this exact combo of 3
        // if 4: can play every combo of suits:
        //      012
        //      013
        //      023
        //      123
        //      0123
        //
        // will need func (prob just like a "new" for Play)
        // that takes a vec of cards + discard reference + kind
        // and then generates the move based on if any cards in the vec
        // are in the discard!

        let matching_suits = all_suits_with_value(&playable_cards, value);
        let num_suits = matching_suits.len();

        match num_suits {
            0 | 2 => (),
            1 => {
                if played_cards.value_was_played_as_multiple(value) {
                    let suit = matching_suits.last().unwrap().clone();
                    let card = Card { value, suit };

                    let mut cards_used = CardSet::new();
                    cards_used.add(&card);

                    let play = Play::make(cards_used, PlayKind::Multiple, discard_pile);
                    plays.push(play);
                }
            },
            3 => {
                let mut cards_used = CardSet::new();
                for suit in matching_suits.iter() {
                    let card = Card { value, suit: suit.clone() };
                    cards_used.add(&card);
                }
                let play = Play::make(cards_used, PlayKind::Multiple, discard_pile);
                plays.push(play);
            },
            4 => {
                todo!()
            },
            _ => panic!("Somehow more than 4 suits represented within hand: {:?}", matching_suits),
        }
    }

    // 2. Straights TODO

    plays
}

impl Play {
    fn make(cards_used: CardSet, kind: PlayKind, discard_pile: &Vec<Card>) -> Play {
        let mut cards_acquired = CardSet::new();

        let mut in_adding_mode = false;
        for discarded_card in discard_pile.iter() {
            if !in_adding_mode && cards_used.contains(discarded_card) {
                in_adding_mode = true;
            }

            if in_adding_mode {
                cards_acquired.add(discarded_card);
            }
        }

        Play {
            cards_used,
            cards_acquired,
            kind,
        }
    }
}


fn all_suits_with_value(s: &CardSet, value: CardValue) -> Vec<CardSuit> {
    let mut found_suits: Vec<CardSuit> = vec![];
    for suit in CardSuit::iter() {
        let card = Card { value, suit };
        if s.contains(&card) {
            found_suits.push(suit);
        }
    }
    found_suits
}

impl PlayedCards {
    fn value_was_played_as_multiple(&self, value: CardValue) -> bool {
        let mut num_times = 0;
        for suit in CardSuit::iter() {
            let card = Card { value, suit };
            if self.multiple_played.contains(&card) {
                num_times += 1;
            }
        }

        match num_times {
            0 => false,
            3 | 4 => true,
            _ => panic!(
                "Invalid game state: {} cards of value {:?} were played as multiples", 
                num_times,
                value, 
            ),
        }
    }
}

fn playable_cards(hand: &CardSet, discard_pile: &Vec<Card>) -> CardSet {
    let mut s = hand.clone();
    for card in discard_pile.iter() {
        s.add(card);
    }
    s
}

