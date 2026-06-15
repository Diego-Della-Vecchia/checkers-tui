pub enum PlayerTurn {
    PlayerOne,
    PlayerTwo,
}

pub enum PlayerPiece {
    PlayerOnePiece,
    PlayerTwoPiece,
}

pub struct State {
    pub fields: [Option<PlayerPiece>; 64],
    pub current_turn: PlayerTurn,
    pub current_selected_piece: u8,
}
