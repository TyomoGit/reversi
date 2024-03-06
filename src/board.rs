use std::fmt::{Debug, Display};

use crate::{
    game::{Result, ReversiGameError},
    player::PlayerKind,
};

pub const DEFAULT_BOARD_SIZE: usize = 8;

pub type Board = Vec<Vec<Option<PlayerKind>>>;

pub trait ReversiBoard {
    fn new() -> Self
    where
        Self: Sized;
    fn with_size(size: usize) -> Self
    where
        Self: Sized;
    fn size(&self) -> usize;
    fn init_four_central_squares(&mut self);
    fn board(&self) -> &Board;
    fn board_mut(&mut self) -> &mut Board;
    fn get_at(&self, x: usize, y: usize) -> Option<PlayerKind>;
    fn in_range(&self, x: usize, y: usize) -> bool;
    fn place_stone(&mut self, x: usize, y: usize, player: PlayerKind) -> Result<()>;
    fn flip(&mut self, x: usize, y: usize) -> Result<()>;
    fn set_turn(&mut self, player: PlayerKind);
    fn turn(&self) -> PlayerKind;
    fn count(&self, player: PlayerKind) -> usize;
    fn is_game_over(&self) -> bool;
}

impl Debug for dyn ReversiBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board = self.board();
        for row in board {
            for cell in row {
                match cell {
                    Some(PlayerKind::Black) => write!(f, "B")?,
                    Some(PlayerKind::White) => write!(f, "W")?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Display for dyn ReversiBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board = self.board();
        for row in board {
            for cell in row {
                match cell {
                    Some(PlayerKind::Black) => write!(f, "⚫︎")?,
                    Some(PlayerKind::White) => write!(f, "⚪︎")?,
                    None => write!(f, "[]")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct ArrayBasedBoard {
    board: Board,
    turn: PlayerKind,
}

impl Default for ArrayBasedBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl ReversiBoard for ArrayBasedBoard {
    fn new() -> Self {
        Self::with_size(DEFAULT_BOARD_SIZE)
    }

    fn with_size(size: usize) -> Self {
        if size & 1 != 0 || size < 4 {
            panic!("Board size must be even");
        }

        Self {
            board: vec![vec![None; size]; size],
            turn: PlayerKind::Black,
        }
    }

    #[inline]
    fn size(&self) -> usize {
        self.board.len()
    }

    fn init_four_central_squares(&mut self) {
        let size = self.board.len();
        let half = size / 2;

        self.board[half - 1][half - 1] = Some(PlayerKind::White);
        self.board[half - 1][half] = Some(PlayerKind::Black);
        self.board[half][half - 1] = Some(PlayerKind::Black);
        self.board[half][half] = Some(PlayerKind::White);
    }

    #[inline]
    fn board(&self) -> &Board {
        &self.board
    }

    #[inline]
    fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    #[inline]
    fn get_at(&self, x: usize, y: usize) -> Option<PlayerKind> {
        self.board
            .get(y)
            .and_then(|row| row.get(x).copied())
            .flatten()
    }

    #[inline]
    fn in_range(&self, x: usize, y: usize) -> bool {
        (0..self.size()).contains(&x) && (0..self.size()).contains(&y)
    }

    fn place_stone(&mut self, x: usize, y: usize, player: PlayerKind) -> Result<()> {
        let size = self.board.len();
        if x >= size || y >= size {
            return Err(ReversiGameError::IndexOutOfBound);
        } else if self.board[y][x].is_some() {
            return Err(ReversiGameError::StoneAlreadyPlaced);
        }

        self.board[y][x] = Some(player);

        Ok(())
    }

    #[inline]
    fn set_turn(&mut self, player: PlayerKind) {
        self.turn = player
    }

    #[inline]
    fn turn(&self) -> PlayerKind {
        self.turn
    }

    fn count(&self, player: PlayerKind) -> usize {
        self.board
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter(|&&x| if let Some(x) = x { player == x } else { false })
            })
            .count()
    }

    fn flip(&mut self, x: usize, y: usize) -> Result<()> {
        if !self.in_range(x, y) {
            return Err(ReversiGameError::IndexOutOfBound);
        }

        let Some(player) = self.get_at(x, y) else {
            return Err(ReversiGameError::NoStoneToFlip);
        };

        self.board[y][x] = Some(player.opposite());

        Ok(())
    }

    fn is_game_over(&self) -> bool {
        let cells_count = self.size() * self.size();
        self.count(PlayerKind::Black) + self.count(PlayerKind::White) == cells_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count() {
        let mut board = ArrayBasedBoard::new();
        board.init_four_central_squares();
        assert_eq!(board.count(PlayerKind::Black), 2);
        assert_eq!(board.count(PlayerKind::White), 2);
    }

    #[test]
    fn init_board() {
        let mut board = ArrayBasedBoard::new();
        board.init_four_central_squares();

        assert_eq!(
            format!("{:?}", &board as &dyn ReversiBoard),
            "........\n........\n........\n...WB...\n...BW...\n........\n........\n........\n"
        );
    }
}
