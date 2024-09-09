use crate::{
    card::Card,
    types::{ChipsType, PlayersAmountType},
};

pub fn make_move(
    hand: Option<(Card, Card)>,
    chips: ChipsType,
    players_amount: PlayersAmountType,
    stage: Stage,
    pot: ChipsType,
    table: Table,
    bet: ChipsType,
    already_betted: ChipsType,
) -> MoveResult {
    todo!()
}

pub enum Stage {
    PreFlop,
    Flop,
    Turn,
    River,
}

pub struct Table(Option<(Card, Card, Card)>, Option<Card>, Option<Card>);

pub enum MoveResult {
    Fold,
    Call,
    Raise(ChipsType),
}
