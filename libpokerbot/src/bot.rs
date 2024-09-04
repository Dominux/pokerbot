use crate::{card::Card, types::ChipsType};

pub struct Bot {
    hand: Option<BotHand>,
    chips: ChipsType,
}

struct BotHand {
    card1: Card,
    card2: Card,
}
