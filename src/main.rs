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

fn print_test_possible_plays1() {
    use CardSuit::*;

    let hand = CardSet::from_vec(&make_cards(vec![
        (1, Clubs),
        (2, Clubs),
        (1, Spades),
        (1, Diamonds),
        (0, Diamonds),
        (0, Clubs),
        (5, Hearts),
        (12, Hearts),
        (12, Diamonds),
    ]));

    let discard_pile = make_cards(vec![
        (12, Spades),
        (6, Clubs),
        (10, Hearts),
        (0, Spades),
        (12, Clubs),
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

fn print_test_possible_plays2() {
    use CardSuit::*;

    let hand = CardSet::from_vec(&make_cards(vec![
        // Should NOT generate a move from this!!
        (2, Spades),

        // This should though
        (10, Spades),

        // Only 012
        (0, Hearts),
        (1, Hearts), 
        (2, Hearts),

        // 5 and 45 
        (5, Clubs),
        (4, Clubs),

        // Generates single
        (9, Clubs),

        // Should generate:
        // 4
        // 45
        // 456
        // 4567
        // 567
        (4, Diamonds),
        (5, Diamonds),
        (6, Diamonds),
        (7, Diamonds),

        //(11, Diamonds),
        //(12, Diamonds),
        //(0, Diamonds),
    ]));

    let discard_pile = make_cards(vec![
    ]);

    let played_cards = score::PlayedCards {
        straight_flush_played: CardSet::from_vec(&make_cards(vec![
            (11, Spades),
            (12, Spades),
            (0, Spades),

            (6, Clubs),
            (7, Clubs),
            (8, Clubs),

            (10, Clubs),
            (11, Clubs),
            (12, Clubs),

            (1, Diamonds),
            (2, Diamonds),
            (3, Diamonds),
        ])),
        multiple_played: CardSet::new(),
    };
    
    let plays = score::all_possible_plays(&hand, &discard_pile, &played_cards);

    println!("PLAYS:");
    for play in plays {
        print_play(play);
    }
}

fn print_test_possible_plays3() {
    use CardSuit::*;

    let hand = CardSet::from_vec(&make_cards(vec![
        (0, Hearts),
        (0, Diamonds),

        (0, Spades),
        (1, Spades),
        (2, Spades),
        (3, Spades),
        (4, Spades),
        (8, Spades),
        (9, Spades),
        (10, Spades),
        (11, Spades),
        (12, Spades),
    ]));

    let discard_pile = make_cards(vec![
        (6, Clubs),
    ]);

    let played_cards = score::PlayedCards {
        straight_flush_played: CardSet::from_vec(&make_cards(vec![
            (3, Clubs),
            (4, Clubs),
            (5, Clubs),

            (7, Clubs),
            (8, Clubs),
            (9, Clubs),

            (9, Hearts),
            (10, Hearts),
            (11, Hearts),
            (12, Hearts),

            (1, Diamonds),
            (2, Diamonds),
            (3, Diamonds),

            (5, Spades),
            (6, Spades),
            (7, Spades),
        ])),
        multiple_played: CardSet::new(),
    };
    
    let plays = score::all_possible_plays(&hand, &discard_pile, &played_cards);

    println!("PLAYS:");
    for play in plays {
        print_play(play);
    }
}



fn main() {
    // TODO
    // i think the ideal way to approximate unit testing this 
    // would be to use Turnt with the output from the testing function above
    // 
    // So i would use clap args parser (which we would want here anyway) and then
    // make a flag to do the output from the function above and any others like it,
    // then use turnt to test. yippeeee
    // (actually turnt might be too strong for this LOL cuz there's no file input. but we
    // will see)
    // (could also make a file input thing for testing maybe that could be neat)
    //print_test_possible_plays1();
    //print_test_possible_plays2();
    print_test_possible_plays3();
}
