pub(crate) struct Card {
    rank: CardRank,
    suit: CardSuit,
}

pub(crate) enum CardRank {
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

pub(crate) enum CardSuit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
