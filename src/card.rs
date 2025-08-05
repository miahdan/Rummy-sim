use std::fmt;

pub const NUM_CARD_VALUES: usize = 13;
pub const NUM_SUITS: usize = 4;
pub const NUM_POSSIBLE_CARDS: usize = NUM_CARD_VALUES * NUM_SUITS;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardValue {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl CardValue {
    pub fn index(&self) -> usize {
        use CardValue::*;
        match self {
            Ace => 0,
            Two => 1,
            Three => 2,
            Four => 3,
            Five => 4,
            Six => 5,
            Seven => 6,
            Eight => 7,
            Nine => 8,
            Ten => 9,
            Jack => 10,
            Queen => 11,
            King => 12,
        }
    }

    pub fn from_index(i: usize) -> Self {
        use CardValue::*;
        match i {
            0 => Ace,
            1 => Two,
            2 => Three,
            3 => Four,
            4 => Five,
            5 => Six,
            6 => Seven,
            7 => Eight,
            8 => Nine,
            9 => Ten,
            10 => Jack,
            11 => Queen,
            12 => King,
            _ => panic!("Invalid index for card value: {}", i),
        }
    }

    pub fn iter() -> std::vec::IntoIter<CardValue> {
        let mut v = vec![];
        for i in 0..NUM_CARD_VALUES {
            v.push(Self::from_index(i));
        }
        v.into_iter()
    }

    // TODO how to deal with Ace's varying value?? Unsure if that will be done here
    //
    // POSSIBLE REPRESENTATIONS OF "PLAYED CARDS" SECITON:
    // 1. list of "plays" each with a list of cards within them and who played them
    // 2. map of all 52 cards to a value containing info on whether or not it's been played,
    // and if so who played it and the type of play that occurred (same-kind, straight,
    // or low straight which would be for ace only)
    //
    // I think I am leaning 2
    //
    //
    fn rummy_value(&self) -> i32 {
        match self {
            CardValue::Ace => 3,
            CardValue::Ten | CardValue::Jack | CardValue::Queen | CardValue::King => 2,
            _ => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardSuit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

impl CardSuit {
    pub fn index(&self) -> usize {
        match self {
            CardSuit::Spades => 0,
            CardSuit::Hearts => 1,
            CardSuit::Clubs => 2,
            CardSuit::Diamonds => 3,
        }
    }

    pub fn from_index(i: usize) -> Self {
        match i {
            0 => CardSuit::Spades,
            1 => CardSuit::Hearts,
            2 => CardSuit::Clubs,
            3 => CardSuit::Diamonds,
            _ => panic!("Invalid index for card suit: {}", i),
        }
    }

    pub fn iter() -> std::vec::IntoIter<CardSuit> {
        let mut v = vec![];
        for i in 0..NUM_SUITS {
            v.push(Self::from_index(i));
        }
        v.into_iter()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub suit: CardSuit,
    pub value: CardValue,
}


impl Card {
    pub fn to_string(&self) -> String {
        let suit_str = match self.suit {
            CardSuit::Spades => "S",
            CardSuit::Hearts => "H",
            CardSuit::Clubs => "C",
            CardSuit::Diamonds => "D",
        };
        use CardValue::*;
        let value_str = match self.value {
            Ace => "A",
            Two => "2",
            Three => "3",
            Four => "4",
            Five => "5",
            Six => "6",
            Seven => "7",
            Eight => "8",
            Nine => "9",
            Ten => "10",
            Jack => "J",
            Queen => "Q",
            King => "K",
        };
        format!("{}:{}", value_str, suit_str)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Clone)]
pub struct CardSet {
    map: [[bool; NUM_POSSIBLE_CARDS]; NUM_SUITS],
}

impl CardSet {
    pub fn as_ordered_list(&self) -> Vec<Card> {
        let mut v = vec![];
        for suit in 0..NUM_SUITS {
            for value in 0..NUM_POSSIBLE_CARDS {
                if self.map[suit][value] {
                    let c = Card {
                        suit: CardSuit::from_index(suit),
                        value: CardValue::from_index(value),
                    };
                    v.push(c);
                }
            }
        }
        v
    }
}

impl fmt::Display for CardSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let card_list = self.as_ordered_list();
        let str_list: Vec<String> = card_list
            .into_iter()
            .map(|c| c.to_string())
            .collect();
        write!(f, "{:?}", str_list)
    }
}

impl CardSet {
    pub fn new() -> Self {
        let map = [[false; NUM_POSSIBLE_CARDS]; NUM_SUITS];
        CardSet { map }
    }

    fn set_value(&mut self, card: &Card, b: bool) -> Result<(), ()> {
        let suit_index = card.suit.index();
        let value_index = card.value.index();
        let old_value = self.map[suit_index][value_index];
        if old_value == b {
            Err(())
        } else {
            self.map[suit_index][value_index] = b;
            Ok(())
        }
    }

    pub fn add(&mut self, card: &Card) {
        let res = self.set_value(card, true);
        if let Err(()) = res {
            panic!("Adding card {} to hand that already contains it: {}", card, self);
        }
    }

    pub fn remove(&mut self, card: &Card) {
        let res = self.set_value(card, false);
        if let Err(()) = res {
            panic!("Removing card {} from a hand without it: {}", card, self);
        }
    }

    pub fn contains(&self, card: &Card) -> bool {
        let suit_index = card.suit.index();
        let value_index = card.value.index();
        self.map[suit_index][value_index]
    }

    pub fn inner_map(&self) -> &[[bool; NUM_POSSIBLE_CARDS]; NUM_SUITS] {
        &self.map
    }

    pub fn from_vec(v: &Vec<Card>) -> CardSet {
        let mut card_set = CardSet::new();
        for card in v.iter() {
            card_set.add(card);
        }
        card_set
    }
}


/*
impl Card {
    // OFFICIAL hashing rules for cards:
    // Spades, Hearts, Clubs, Diamonds is the suits order
    // Multiply 13 (number of values) times the index of the suit
    // Then add the value, where Ace is 0 and King is 12

    fn hash(&self) -> u8 {
        let suit_component = NUM_CARD_VALUES * self.suit.index();
        let value_component = self.value.index();
        (value_component + suit_component) as u8
    }

    fn unhash(h: u8) -> Card {
        let h = h as usize;
        if h < NUM_POSSIBLE_CARDS {
            panic!("Invalid card hash: {}", h);
        } 
        let suit_component = h / NUM_CARD_VALUES;
        let value_component = h % NUM_CARD_VALUES;
        Card {
            suit: CardSuit::from_index(suit_component),
            value: CardValue::from_index(value_component),
        }
    }
}
*/

/*
 
 I am wholly unsure if the hash-table representation of a stack of cards yields any
 real benefits other than seeming kinda cool. Obviously all operations are O(52), but this
 is true with vectors as well since the number of possible cards in play is always bounded. 
 I think the only benefit could be if there's a table referring to every card in play in 
 the whole game (as it might prevent bugs where cards would maybe be duplicated?), but the
 cost (being extremely cumbersome) of doing that is fairly high for little reward

 BUT i think for players' hands, this representation is pretty neat!! Because then calculating
 things like if there are straights in your hand is really easy!! This would just be like a 
 52-sized array of bools i guess
 So TODO make simpler hand representation struct 
 Wait what if it had
 4 slots one per suit
 13 slots in each suit, one per value
 to make combo-finding calculations as easy as possible

 MAYBE can also use something like this for the "played cards" area?
 

/// A CardStack represents a stack of cards,
/// but using a more efficient lookup-table 
/// representation as opposed to something 
/// like a vector. Can represent hands,
/// discard piles, the deck, etc. Has specialized
/// shuffling function.
pub struct CardStack {
    pub map: [Option<usize>; NUM_POSSIBLE_CARDS],
}

impl CardStack {
    pub fn shuffle(&mut self) {
    }
}

*/
