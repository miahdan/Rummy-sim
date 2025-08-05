use rummy_sim::card::{CardSet, Card, CardSuit, CardValue};
use rummy_sim::score;

fn make_card(value_index: usize, suit: CardSuit) -> Card {
    Card {
        value: CardValue::from_index(value_index),
        suit,
    }
}

fn make_cards(v: Vec<(usize, CardSuit)>) -> Vec<Card> {
    v.into_iter().map(|pair| make_card(pair.0, pair.1)).collect()
}

fn print_play(play: score::Play) {
    println!("({:?}, {}, {})", play.kind, play.cards_used, play.cards_acquired);
}

fn print_test_possible_plays() {
    use CardSuit::*;

    let hand = CardSet::from_vec(&make_cards(vec![
        (1, Clubs),
        (2, Clubs),
        (1, Spades),
        (1, Diamonds),
        (0, Diamonds),
        (0, Clubs),
        (5, Hearts),
    ]));

    let discard_pile = make_cards(vec![
        (6, Clubs),
        (10, Hearts),
        (0, Spades),
        (4, Spades),
    ]);

    let played_cards = score::PlayedCards {
        straight_flush_played: CardSet::new(),
        multiple_played: CardSet::from_vec(&make_cards(vec![
            (5, Clubs),
            (5, Spades),
            (5, Diamonds),
            (10, Clubs),
            (10, Spades),
            (10, Diamonds),
        ])),
    };
    
    let plays = score::all_possible_plays(&hand, &discard_pile, &played_cards);

    println!("PLAYS:");
    for play in plays {
        print_play(play);
    }
}

fn main() {
    print_test_possible_plays();
}
