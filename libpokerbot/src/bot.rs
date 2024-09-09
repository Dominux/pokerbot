use crate::{
    card::Card,
    types::{ChipsType, PlayersAmountType},
};

pub struct Bot {
    hand: Option<(Card, Card)>,
    chips: ChipsType,
    players_amount: PlayersAmountType,
    stage: Stage,
    bank: ChipsType,
    table: Table,
}

impl Bot {
    pub fn new(
        hand: Option<(Card, Card)>,
        chips: ChipsType,
        players_amount: PlayersAmountType,
        stage: Stage,
        bank: ChipsType,
        table: Table,
    ) -> Self {
        Self {
            hand,
            chips,
            players_amount,
            stage,
            bank,
            table,
        }
    }

    pub fn make_move(&self) {}
}

pub(crate) enum Stage {
    PreFlop,
    Flop,
    Turn,
    River,
}

struct Table(Option<(Card, Card, Card)>, Option<Card>, Option<Card>);
