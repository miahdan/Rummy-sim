use std::fmt;

pub const NUM_CARD_VALUES: usize = 13;
pub const NUM_SUITS: usize = 4;
pub const NUM_POSSIBLE_CARDS: usize = NUM_CARD_VALUES * NUM_SUITS;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardValue {
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
    Ace,
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

    pub fn next(&self) -> CardValue {
        if let CardValue::King = self {
            CardValue::Ace
        } else {
            let index = self.index();
            CardValue::from_index(index + 1)
        }
    }

    pub fn prev(&self) -> CardValue {
        if let CardValue::Ace = self {
            CardValue::King
        } else {
            let index = self.index();
            CardValue::from_index(index - 1)
        }
    }

    pub fn iter() -> std::vec::IntoIter<CardValue> {
        let mut v = vec![];
        for i in 0..NUM_CARD_VALUES {
            v.push(Self::from_index(i));
        }
        v.into_iter()
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
        let result = self.set_value(card, true);
        if let Err(()) = result {
            panic!("Adding card {} to set that already contains it: {}", card, self);
        }
    }

    pub fn remove(&mut self, card: &Card) {
        let result = self.set_value(card, false);
        if let Err(()) = result {
            panic!("Removing card {} from a set without it: {}", card, self);
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
