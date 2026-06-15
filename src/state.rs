#[derive(Debug, Clone, Copy)]
pub enum PlayerTurn {
    PlayerOne,
    PlayerTwo,
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerPiece {
    PlayerOnePiece,
    PlayerTwoPiece,
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    pub fields: [Option<PlayerPiece>; 64],
    pub current_turn: PlayerTurn,
    pub current_selected_piece: Option<u8>,
}

impl Default for State {
    fn default() -> Self {
        let mut fields = [const { Option::None }; 64];

        for row in 0..8 {
            for col in 0..8 {
                // Checkers are only placed on dark squares
                // Dark always lies on:
                // even x + uneven y or
                // uneven y + even x
                if (row + col) % 2 == 1 {
                    let index = row * 8 + col;

                    if row < 3 {
                        // Top 3 rows belong to Player Two
                        fields[index] = Option::Some(PlayerPiece::PlayerTwoPiece);
                    } else if row > 4 {
                        // Bottom 3 rows belong to Player One
                        fields[index] = Option::Some(PlayerPiece::PlayerOnePiece);
                    }
                }
            }
        }

        State {
            fields,
            current_turn: PlayerTurn::PlayerOne,
            current_selected_piece: Option::None,
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }
}
