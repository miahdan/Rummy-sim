use crate::card::{Card, CardSet, CardSuit, CardValue, NUM_CARD_VALUES, NUM_SUITS};

const HIGH_ACE_INDEX: usize = 13;
const LOW_ACE_INDEX: usize = 0;

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

#[derive(Clone, Debug)]
pub enum PlayKind {
    StraightFlush {
        ace_status: Option<AceStatus>,
    },
    Multiple,
}

#[derive(Clone, Debug)]
pub enum AceStatus {
    High,
    Low, 
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
                let mut all_three_card_move_sets: Vec<CardSet> = vec![CardSet::new(); 4];
                let mut four_card_move_set = CardSet::new();
                for i in 0..4 {
                    let suit = CardSuit::from_index(i);
                    let card = Card { value, suit };
                    for j in 0..4 {
                        if i != j {
                            all_three_card_move_sets[j].add(&card);
                        }
                    }
                    four_card_move_set.add(&card);
                }

                for three_card_set in all_three_card_move_sets.into_iter() {
                    let play = Play::make(three_card_set, PlayKind::Multiple, discard_pile);
                    plays.push(play);
                }

                let four_play = Play::make(four_card_move_set, PlayKind::Multiple, discard_pile);
                plays.push(four_play);
            },
            _ => panic!("Somehow more than 4 suits represented within hand: {:?}", matching_suits),
        }
    }

    // 2. Straights TODO
    // Straights can be 1-13 length 
    // OH i gotta remember that ace can be played before 2 or after K
    // (and if it's played, to set the ace status)
    //
    // Potentially good news related to aces: 2 and K literally cannot be played
    // off a played ace in any scenario :D
    // Actually wait this might be useful to think about for the algorithm of finding
    // cards that can be played off the already-played cards
    // 2 can only be played as a low, K can only be played as a high
    // Idk maybe not
    //
    //
    // length casework:
    // 1, 2 = play on top of another
    // 3+ = tons of combinations omg 
    //
    // unlike multis if there's one of length 3+ it can still be split up and played
    //
    // easiest way 2 go off the top of my head is to START with the already-played cards
    // and get some sort of list of things you can play off those

    // 2a. Play-off-of straights
    for suit in CardSuit::iter() {
        for value in CardValue::iter() {
            let card = Card { suit, value };
            if !playable_cards.contains(&card) {
                continue;
            }

            let next_card = Card { suit, value: value.next() };
            let prev_card = Card { suit, value: value.prev() };
            if played_cards.straight_flush_played.contains(&next_card) {
                generate_straight_extensions(
                    &playable_cards, 
                    &card, 
                    &prev_card, 
                    AceStatus::Low, 
                    &mut plays, 
                    discard_pile,
                );
            }
            else if played_cards.straight_flush_played.contains(&prev_card) {
                generate_straight_extensions(
                    &playable_cards, 
                    &card, 
                    &next_card, 
                    AceStatus::High, 
                    &mut plays, 
                    discard_pile,
                );
            }
        }
    }
    
    // 2b. Standalone straights
    for suit in CardSuit::iter() {
        for value_index in LOW_ACE_INDEX..HIGH_ACE_INDEX {
            let has_card = bounded_cardset_contains(&playable_cards, suit, value_index);
            let has_next = bounded_cardset_contains(&playable_cards, suit, value_index + 1);
            let next_has_next = bounded_cardset_contains(&playable_cards, suit, value_index + 2);
            if has_card && has_next && next_has_next {
                generate_standalone_plays(
                    &playable_cards, 
                    suit, 
                    value_index, 
                    &mut plays,
                    &discard_pile,
                );
            }
        }
    }

    plays
}

fn generate_straight_extensions(
    playable_cards: &CardSet, 
    card: &Card, 
    additional_card: &Card, 
    ace_status_value: AceStatus,
    plays: &mut Vec<Play>,
    discard_pile: &Vec<Card>,
) {

    let mut cards_used = CardSet::new();
    cards_used.add(&card);

    let ace_status = match card.value {
        CardValue::Ace => Some(ace_status_value),
        _ => None,
    };
    let value_is_not_ace = ace_status.is_none();
    let kind = PlayKind::StraightFlush { ace_status };

    if value_is_not_ace && playable_cards.contains(additional_card) {
        let mut other_cards_used = cards_used.clone();
        other_cards_used.add(additional_card);
        let play = Play::make(other_cards_used, kind.clone(), discard_pile);
        plays.push(play);
    }

    let play = Play::make(cards_used, kind, discard_pile);
    plays.push(play);
}

fn generate_standalone_plays(
    playable_cards: &CardSet, 
    suit: CardSuit, 
    value_index: usize, 
    plays: &mut Vec<Play>,
    discard_pile: &Vec<Card>,
) {
    let card_of_index = |i| {
        let mod_index = mod_value_index(i);
        let value = CardValue::from_index(mod_index);
        Card { suit, value }
    };

    let mut first_set = CardSet::new();
    let mut first_ace_status: Option<AceStatus> = None;
    for i in value_index..(value_index + 3) {
        let single_ace_status = ace_status_of_index(i);
        first_ace_status = first_ace_status.or(single_ace_status);

        let card = card_of_index(i);
        first_set.add(&card);
    }
    
    let mut play_infos = vec![(first_set, first_ace_status)];

    let mut extra_index = value_index + 3;
    while bounded_cardset_contains(playable_cards, suit, extra_index) {
        let (mut set, ace_status) = play_infos.last().unwrap().clone();

        let card = card_of_index(extra_index);
        set.add(&card);

        let single_ace_status = ace_status_of_index(extra_index);
        let ace_status = ace_status.or(single_ace_status);

        play_infos.push((set, ace_status));

        extra_index += 1;
    }

    for (cards_used, ace_status) in play_infos.into_iter() {
        let kind = PlayKind::StraightFlush { ace_status };
        let play = Play::make(cards_used, kind, discard_pile);
        plays.push(play);
    }
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

        Play { cards_used, cards_acquired, kind }
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
                num_times, value,
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

fn bounded_cardset_contains(set: &CardSet, suit: CardSuit, index: usize) -> bool {
    if index > HIGH_ACE_INDEX { 
        return false; 
    }

    let mod_index = mod_value_index(index);
    let card = Card { suit, value: CardValue::from_index(mod_index) };
    set.contains(&card)
}

fn mod_value_index(value_index: usize) -> usize {
    if value_index == HIGH_ACE_INDEX { LOW_ACE_INDEX } else { value_index }
}

fn ace_status_of_index(value_index: usize) -> Option<AceStatus> {
    match value_index {
        LOW_ACE_INDEX => Some(AceStatus::Low),
        HIGH_ACE_INDEX => Some(AceStatus::High),
        _ => None,
    }
}
