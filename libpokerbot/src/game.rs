use crate::types::{ChipsType, PlayersAmountType};

pub(crate) struct Game {
    players_amount: PlayersAmountType,
    stage: GameStage,
    bank: ChipsType,
}

pub(crate) enum GameStage {
    NoGame,
    
}
