use std::fmt::{Display, Write};

use crate::{
    board::{ArrayBasedBoard, ReversiBoard},
    player::PlayerKind,
    point::Point,
};

pub type Result<T> = std::result::Result<T, ReversiGameError>;

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

pub struct SimpleReversiGame {
    board: Box<dyn ReversiBoard>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReversiGameError {
    StoneAlreadyPlaced,
    InvalidMove,
    IndexOutOfBound,
    NoStoneToFlip,
    NextPlayerCantPutStone,

    GameOverWithWinner(PlayerKind),
    GameOverWithDraw,
}

impl SimpleReversiGame {
    pub fn new() -> Self {
        let mut board: Box<dyn ReversiBoard> = Box::new(ArrayBasedBoard::new());
        board.init_four_central_squares();
        board.set_turn(PlayerKind::Black);

        Self { board }
    }

    pub fn put_stone(&mut self, x: usize, y: usize) -> Result<()> {
        if !self.check_can_put(x, y) {
            return Err(ReversiGameError::InvalidMove);
        }

        self.board.place_stone(x, y, self.board.turn())?;

        for d in DIRECTIONS {
            let mut stack: Vec<Point> = Vec::new();

            let mut x = x as i32 + d.0;
            let mut y = y as i32 + d.1;

            if !self.board.in_range(x as usize, y as usize) {
                continue;
            }

            while self.board.get_at(x as usize, y as usize) == Some(self.board.turn().opposite()) {
                stack.push(Point::new(x as usize, y as usize));
                x += d.0;
                y += d.1;
            }

            if self.board.get_at(x as usize, y as usize) != Some(self.board.turn()) {
                continue;
            }

            for p in stack {
                self.board.flip(p.x, p.y)?;
            }
        }

        if self.board.is_game_over() {
            self.winner()?;
        }

        self.board.set_turn(self.board.turn().opposite());

        if self.get_can_put_stones().is_empty() {
            // Next player cannot place stones

            self.board.set_turn(self.board.turn().opposite());

            if self.get_can_put_stones().is_empty() {
                // Both players cannot place stones
                return self.winner();
            }

            // Next next player(the player who called this function) can place stones
            return Err(ReversiGameError::NextPlayerCantPutStone);
        }

        if self.board.count(self.board.turn()) == 0 {
            // There are no next player's stones
            return Err(ReversiGameError::GameOverWithWinner(self.board.turn().opposite()));
        }

        Ok(())
    }

    fn winner(&self) -> Result<()> {
        match (
            self.board.count(PlayerKind::Black),
            self.board.count(PlayerKind::White),
        ) {
            (black, white) if black > white => {
                Err(ReversiGameError::GameOverWithWinner(PlayerKind::Black))
            }
            (black, white) if black < white => {
                Err(ReversiGameError::GameOverWithWinner(PlayerKind::White))
            }
            _ => Err(ReversiGameError::GameOverWithDraw),
        }
    }

    pub fn check_can_put(&self, x: usize, y: usize) -> bool {
        if !self.board.in_range(x, y) {
            return false;
        }

        if self.board.get_at(x, y).is_some() {
            return false;
        }

        for d in DIRECTIONS {
            let mut stack: Vec<Point> = Vec::new();

            let mut x = x as i32 + d.0;
            let mut y = y as i32 + d.1;

            if !self.board.in_range(x as usize, y as usize) {
                continue;
            }

            while self.board.get_at(x as usize, y as usize) == Some(self.board.turn().opposite()) {
                stack.push(Point::new(x as usize, y as usize));
                x += d.0;
                y += d.1;
            }

            if self.board.get_at(x as usize, y as usize) == Some(self.board.turn()) && !stack.is_empty() {
                return true;
            }
        }

        false
    }

    pub fn get_can_put_stones(&self) -> Vec<Point> {
        let mut result: Vec<Point> = Vec::new();

        for y in 0..self.board.size() {
            for x in 0..self.board.size() {
                if self.check_can_put(x, y) {
                    result.push(Point::new(x, y));
                }
            }
        }

        result
    }

    pub fn board(&self) -> &dyn ReversiBoard {
        self.board.as_ref()
    }
}

impl Default for SimpleReversiGame {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for SimpleReversiGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board = self.board.to_string();
        let mut result = String::new();

        write!(result, "   | ")?;

        for i in 0..self.board().size() {
            let alpha = (b'A' + i as u8) as char;
            write!(result, "{: ^2}", alpha)?;
        }

        writeln!(result)?;

        writeln!(result, "---+-{}", "--".repeat(self.board().size()))?;

        for (i, row) in board.lines().enumerate() {
            writeln!(result, "{: ^2} | {}", i + 1, row)?;
        }

        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn t1() {
        let mut game = SimpleReversiGame::new();
        assert_eq!(game.board.turn(), PlayerKind::Black);
        game.put_stone(3, 2).unwrap();
        assert_eq!(game.board.turn(), PlayerKind::White);
        game.put_stone(2, 4).unwrap();
    }

    #[test]
    fn finish() {
        let mut game = SimpleReversiGame::new();
        let size = game.board().size();

        *game.board.board_mut() = vec![vec![Some(PlayerKind::White); size]; size];
        game.board.board_mut()[0][0] = None;
        game.board.board_mut()[0][7] = Some(PlayerKind::Black);

        let result = game.put_stone(0, 0);
        assert_eq!(
            result,
            Err(ReversiGameError::GameOverWithWinner(PlayerKind::White))
        );
        assert_eq!(game.board().count(PlayerKind::Black), 8);
        assert_eq!(game.board().count(PlayerKind::White), size * size - 8);
    }

    #[test]
    fn cant_put() {
        let mut game = SimpleReversiGame::new();
        *game.board.board_mut() = vec![vec![None; 8]; 8];
        game.board.board_mut()[0][0] = Some(PlayerKind::Black);
        game.board.board_mut()[0][1] = Some(PlayerKind::White);

        assert_eq!(game.put_stone(2, 0), Err(ReversiGameError::NextPlayerCantPutStone));
    }
}
