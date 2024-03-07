use std::{fmt::{Debug, Display}, vec};

use crate::{game::Result, stone::Stone, point::Point};

pub const DEFAULT_BOARD_SIZE: usize = 8;

pub type Board = Vec<Vec<Option<Stone>>>;

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReversiError {
    StoneAlreadyPlaced,
    InvalidMove,
    IndexOutOfBound,
    NoStoneToFlip,
    NextPlayerCantPutStone,

    GameOverWithWinner(Stone),
    GameOverWithDraw,
}

pub trait ReversiBoard {
    fn size(&self) -> usize;
    fn board(&self) -> &Board;
    fn board_mut(&mut self) -> &mut Board;

    fn get_at(&self, x: usize, y: usize) -> Option<Stone>;
    fn in_range(&self, x: usize, y: usize) -> bool;
    fn count(&self, player: Stone) -> usize;
    fn count_flippable(&self, x: usize, y: usize) -> usize;
    fn is_game_over(&self) -> bool;

    fn init_four_central_squares(&mut self);
    fn flip(&mut self, x: usize, y: usize) -> Result<()>;
    fn put_stone(&mut self, x: usize, y: usize, player: Stone) -> Result<()>;
    fn winner(&self) -> Result<()>;
    fn check_can_put(&self, x: usize, y: usize, player: Stone) -> bool;
    fn get_can_put_stones(&self, player: Stone) -> Vec<Point>;
}

impl Debug for dyn ReversiBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board = self.board();
        for row in board {
            for cell in row {
                match cell {
                    Some(Stone::Black) => write!(f, "B")?,
                    Some(Stone::White) => write!(f, "W")?,
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
                    Some(Stone::Black) => write!(f, "⚫︎")?,
                    Some(Stone::White) => write!(f, "⚪︎")?,
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
}

impl Default for ArrayBasedBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl ArrayBasedBoard {
    pub fn new() -> Self {
        Self::with_size(DEFAULT_BOARD_SIZE)
    }

    pub fn with_size(size: usize) -> Self {
        if size & 1 != 0 || size < 4 {
            panic!("Board size must be even");
        }

        Self {
            board: vec![vec![None; size]; size],
        }
    }
}

impl ReversiBoard for ArrayBasedBoard {
    #[inline]
    fn size(&self) -> usize {
        self.board.len()
    }

    fn init_four_central_squares(&mut self) {
        let size = self.board.len();
        let half = size / 2;

        self.board[half - 1][half - 1] = Some(Stone::White);
        self.board[half - 1][half] = Some(Stone::Black);
        self.board[half][half - 1] = Some(Stone::Black);
        self.board[half][half] = Some(Stone::White);
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
    fn get_at(&self, x: usize, y: usize) -> Option<Stone> {
        self.board
            .get(y)
            .and_then(|row| row.get(x).copied())
            .flatten()
    }

    #[inline]
    fn in_range(&self, x: usize, y: usize) -> bool {
        (0..self.size()).contains(&x) && (0..self.size()).contains(&y)
    }

    fn count(&self, player: Stone) -> usize {
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
            return Err(ReversiError::IndexOutOfBound);
        }

        let Some(player) = self.get_at(x, y) else {
            return Err(ReversiError::NoStoneToFlip);
        };

        self.board[y][x] = Some(player.opposite());

        Ok(())
    }

    fn is_game_over(&self) -> bool {
        let cells_count = self.size() * self.size();
        self.count(Stone::Black) + self.count(Stone::White) == cells_count
    }
    
    fn put_stone(&mut self, x: usize, y: usize, player: Stone) -> Result<()> {
        if !self.check_can_put(x, y, player) {
            return Err(ReversiError::InvalidMove);
        }

        let size = self.board.len();
        if x >= size || y >= size {
            return Err(ReversiError::IndexOutOfBound);
        } else if self.board[y][x].is_some() {
            return Err(ReversiError::StoneAlreadyPlaced);
        }
        self.board[y][x] = Some(player);

        get_flippable(self, x, y, player)
            .iter()
            .for_each(|&Point{x, y}| {
                self.flip(x, y).unwrap();
            });

        if self.is_game_over() {
            self.winner()?;
        }

        // self.set_turn(self.turn().opposite());

        if self.get_can_put_stones(player.opposite()).is_empty() {
            // Next player cannot place stones

            // self.set_turn(self.turn().opposite());

            if self.get_can_put_stones(player).is_empty() {
                // Both players cannot place stones
                return self.winner();
            }

            // Next next player(the player who called this function) can place stones
            return Err(ReversiError::NextPlayerCantPutStone);
        }

        if self.count(player.opposite()) == 0 {
            // There are no next player's stones
            return Err(ReversiError::GameOverWithWinner(player));
        }

        Ok(())
    }
    
    fn winner(&self) -> Result<()> {
        match (
            self.count(Stone::Black),
            self.count(Stone::White),
        ) {
            (black, white) if black > white => {
                Err(ReversiError::GameOverWithWinner(Stone::Black))
            }
            (black, white) if black < white => {
                Err(ReversiError::GameOverWithWinner(Stone::White))
            }
            _ => Err(ReversiError::GameOverWithDraw),
        }
    }
    
    fn check_can_put(&self, x: usize, y: usize, player: Stone) -> bool {
        if !self.in_range(x, y) {
            return false;
        }

        if self.get_at(x, y).is_some() {
            return false;
        }

        for d in DIRECTIONS {
            let mut stack: Vec<Point> = Vec::new();

            let mut x = x as i32 + d.0;
            let mut y = y as i32 + d.1;

            if !self.in_range(x as usize, y as usize) {
                continue;
            }

            while self.get_at(x as usize, y as usize) == Some(player.opposite()) {
                stack.push(Point::new(x as usize, y as usize));
                x += d.0;
                y += d.1;
            }

            if self.get_at(x as usize, y as usize) == Some(player) && !stack.is_empty() {
                return true;
            }
        }

        false
    }
    
    fn get_can_put_stones(&self, player: Stone) -> Vec<Point> {
        let mut result: Vec<Point> = Vec::new();

        for y in 0..self.size() {
            for x in 0..self.size() {
                if self.check_can_put(x, y, player) {
                    result.push(Point::new(x, y));
                }
            }
        }

        result
    }
    
    fn count_flippable(&self, x: usize, y: usize) -> usize {
        let Some(color) = self.get_at(x, y) else {
            return 0;
        };

        get_flippable(self, x, y, color).len()
    }
}

fn get_flippable(board: &dyn ReversiBoard, x: usize, y: usize, player: Stone) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();

    for d in DIRECTIONS {
        let mut stack: Vec<Point> = Vec::new();

        let mut x = x as i32 + d.0;
        let mut y = y as i32 + d.1;

        if !board.in_range(x as usize, y as usize) {
            continue;
        }

        while board.get_at(x as usize, y as usize) == Some(player.opposite()) {
            stack.push(Point::new(x as usize, y as usize));
            x += d.0;
            y += d.1;
        }

        if board.get_at(x as usize, y as usize) != Some(player) {
            continue;
        }

        result.extend(stack);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count() {
        let mut board = ArrayBasedBoard::new();
        board.init_four_central_squares();
        assert_eq!(board.count(Stone::Black), 2);
        assert_eq!(board.count(Stone::White), 2);
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
