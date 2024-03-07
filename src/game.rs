use std::fmt::{Display, Write};

use crate::{
    board::{ArrayBasedBoard, ReversiBoard, ReversiError},
    computer::PlayerType,
    point::Point,
    stone::Stone,
};

pub type Result<T> = std::result::Result<T, ReversiError>;

pub struct SimpleReversiGame {
    board: Box<dyn ReversiBoard>,
    turn: Stone,

    black: PlayerType,
    white: PlayerType,
}

impl SimpleReversiGame {
    pub fn new(black: PlayerType, white: PlayerType) -> Self {
        let mut board: Box<dyn ReversiBoard> = Box::new(ArrayBasedBoard::new());
        board.init_four_central_squares();

        Self {
            board,
            turn: Stone::Black,
            black,
            white,
        }
    }

    pub fn put_stone(&mut self, x: usize, y: usize) -> Result<()> {
        self.board.put_stone(x, y, self.turn)?;
        self.take_turn();
        Ok(())
    }

    pub fn winner(&self) -> Result<()> {
        self.board.winner()
    }

    pub fn check_can_put(&self, x: usize, y: usize) -> bool {
        self.board.check_can_put(x, y, self.turn)
    }

    pub fn get_can_put_stones(&self) -> Vec<Point> {
        self.board.get_can_put_stones(self.turn)
    }

    pub fn board(&self) -> &dyn ReversiBoard {
        self.board.as_ref()
    }

    #[inline]
    pub fn take_turn(&mut self) {
        self.turn = self.turn.opposite();

        self.on_player_turn();
    }

    fn on_player_turn(&mut self) {
        let player = match self.turn {
            Stone::Black => &self.black,
            Stone::White => &self.white,
        };

        let PlayerType::Computer(computer) = player else {
            return;
        };

        let point = computer.decide(self.board());
        self.put_stone(point.x, point.y).unwrap();
    }

    #[inline]
    pub fn set_turn(&mut self, turn: Stone) {
        self.turn = turn;
    }

    #[inline]
    pub fn turn(&self) -> Stone {
        self.turn
    }

    #[inline]
    pub fn black(&self) -> &PlayerType {
        &self.black
    }

    #[inline]
    pub fn white(&self) -> &PlayerType {
        &self.white
    }
}

impl Default for SimpleReversiGame {
    fn default() -> Self {
        Self::new(PlayerType::Human, PlayerType::Human)
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
        let mut game = SimpleReversiGame::default();
        assert_eq!(game.turn(), Stone::Black);
        game.put_stone(3, 2).unwrap();
        assert_eq!(game.turn(), Stone::White);
        game.put_stone(2, 4).unwrap();
    }

    #[test]
    fn finish() {
        let mut game = SimpleReversiGame::default();
        let size = game.board().size();

        *game.board.board_mut() = vec![vec![Some(Stone::White); size]; size];
        game.board.board_mut()[0][0] = None;
        game.board.board_mut()[0][7] = Some(Stone::Black);

        let result = game.put_stone(0, 0);
        assert_eq!(result, Err(ReversiError::GameOverWithWinner(Stone::White)));
        assert_eq!(game.board().count(Stone::Black), 8);
        assert_eq!(game.board().count(Stone::White), size * size - 8);
    }

    #[test]
    fn cant_put() {
        let mut game = SimpleReversiGame::default();
        *game.board.board_mut() = vec![vec![None; 8]; 8];
        game.board.board_mut()[0][0] = Some(Stone::Black);
        game.board.board_mut()[0][1] = Some(Stone::White);

        game.board.board_mut()[7][7] = Some(Stone::Black);
        game.board.board_mut()[7][6] = Some(Stone::White);

        assert_eq!(
            game.put_stone(2, 0),
            Err(ReversiError::NextPlayerCantPutStone)
        );
    }
}
