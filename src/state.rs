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
    pub selected_index: Option<u8>,
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
            selected_index: Option::None,
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_click(&mut self, index: u8) {
        let index = index as usize;

        // If the clicked cell has a friendly piece, select it.
        if let Some(piece) = self.fields[index]
            && self.is_friendly_piece(piece)
        {
            self.selected_index = Some(index as u8);
            return;
        }

        // Otherwise, try to move the currently selected piece to the clicked cell.
        if let Some(selected) = self.selected_index {
            let selected = selected as usize;
            if self.is_valid_move(selected, index) {
                self.make_move(selected, index);
                self.selected_index = None;
                self.switch_turn();
            }
        }
    }

    fn is_friendly_piece(&self, piece: PlayerPiece) -> bool {
        matches!(
            (self.current_turn, piece),
            (PlayerTurn::PlayerOne, PlayerPiece::PlayerOnePiece)
                | (PlayerTurn::PlayerTwo, PlayerPiece::PlayerTwoPiece)
        )
    }

    fn is_valid_move(&self, from: usize, to: usize) -> bool {
        // Target must be empty and on a playable (black) cell.
        if self.fields[to].is_some() {
            return false;
        }

        let from_row = from / COL_COUNT;
        let from_col = from % COL_COUNT;
        let to_row = to / COL_COUNT;
        let to_col = to % COL_COUNT;

        if (to_row + to_col) % 2 != 1 {
            return false;
        }

        let row_diff = to_row as isize - from_row as isize;
        let col_diff = to_col as isize - from_col as isize;

        let forward_dir: isize = match self.current_turn {
            PlayerTurn::PlayerOne => -1,
            PlayerTurn::PlayerTwo => 1,
        };

        // Regular diagonal move.
        if row_diff == forward_dir && col_diff.abs() == 1 {
            return true;
        }

        // Capture jump: two diagonal steps forward over an opponent piece.
        if row_diff == 2 * forward_dir && col_diff.abs() == 2 {
            let jumped_row = (from_row as isize + forward_dir) as usize;
            let jumped_col = (from_col as isize + col_diff / 2) as usize;
            let jumped_index = jumped_row * COL_COUNT + jumped_col;

            if let Some(piece) = self.fields[jumped_index] {
                return !self.is_friendly_piece(piece);
            }
        }

        false
    }

    fn make_move(&mut self, from: usize, to: usize) {
        self.fields[to] = self.fields[from];
        self.fields[from] = None;

        // Remove the jumped piece if this was a capture.
        let from_row = from / COL_COUNT;
        let from_col = from % COL_COUNT;
        let to_row = to / COL_COUNT;
        let to_col = to % COL_COUNT;

        let row_diff = to_row as isize - from_row as isize;
        let col_diff = to_col as isize - from_col as isize;

        if row_diff.abs() == 2 && col_diff.abs() == 2 {
            let jumped_row = (from_row as isize + row_diff / 2) as usize;
            let jumped_col = (from_col as isize + col_diff / 2) as usize;
            let jumped_index = jumped_row * COL_COUNT + jumped_col;
            self.fields[jumped_index] = None;
        }
    }

    fn switch_turn(&mut self) {
        self.current_turn = match self.current_turn {
            PlayerTurn::PlayerOne => PlayerTurn::PlayerTwo,
            PlayerTurn::PlayerTwo => PlayerTurn::PlayerOne,
        };
    }
}
