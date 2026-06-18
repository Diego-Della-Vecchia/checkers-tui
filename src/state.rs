pub const ROW_COUNT: usize = 8;
pub const COL_COUNT: usize = 8;

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

        for row in 0..ROW_COUNT {
            for col in 0..COL_COUNT {
                // black cells appear on even x + odd y or odd x + even y
                if (row + col) % 2 == 1 {
                    let index = row * COL_COUNT + col;

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
