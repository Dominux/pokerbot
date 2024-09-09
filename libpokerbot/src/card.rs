pub struct Card {
    rank: CardRank,
    suit: CardSuit,
}

pub enum CardRank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

pub enum CardSuit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
