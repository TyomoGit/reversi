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
        let result = self.board.put_stone(x, y, self.turn);

        if let PlayerType::Computer(_) = self.current_player_type() {
            self.take_turn()?;
            result
        } else {
            result?;
            self.take_turn()
        }
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
    pub fn take_turn(&mut self) -> Result<()> {
        self.turn = self.turn.opposite();

        self.on_player_turn()
    }

    #[inline]
    fn current_player_type(&self) -> &PlayerType {
        match self.turn {
            Stone::Black => &self.black,
            Stone::White => &self.white,
        }
    }

    fn on_player_turn(&mut self) -> Result<()> {
        let player = self.current_player_type();

        let PlayerType::Computer(computer) = player else {
            return Ok(());
        };

        let point = computer.decide(self.board());

        let board_before_put = dyn_clone::clone_box(self.board.as_ref());

        self.put_stone(point.x, point.y)?;

        Err(ReversiError::ComputerTurnIsOk(board_before_put))
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
        write!(f, "{}", self.board)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::computer::{SimpleComputer, WeightedComputer};

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

    #[test]
    fn com_human_err() {
        let mut game = SimpleReversiGame::new(
            PlayerType::Human,
            PlayerType::Computer(Box::new(WeightedComputer::new(Stone::White))),
        );

        *game.board.board_mut() = vec![vec![None; 8]; 8];
        game.board.board_mut()[0][0] = Some(Stone::Black);
        game.board.board_mut()[0][1] = Some(Stone::White);

        game.board.board_mut()[7][0] = Some(Stone::Black);
        game.board.board_mut()[7][1] = Some(Stone::White);

        dbg!(&game.board);
        let result = game.put_stone(2, 0);
        assert_eq!(result, Err(ReversiError::NextPlayerCantPutStone));
        assert_eq!(game.turn(), Stone::Black);
    }

    #[test]
    fn com_com_err() {
        let mut game = SimpleReversiGame::new(
            PlayerType::Human,
            PlayerType::Computer(Box::new(WeightedComputer::new(Stone::White))),
        );

        *game.board.board_mut() = vec![vec![None; 8]; 8];
        game.board.board_mut()[0][0] = Some(Stone::White);
        game.board.board_mut()[0][1] = Some(Stone::Black);

        game.board.board_mut()[7][7] = Some(Stone::White);
        game.board.board_mut()[7][6] = Some(Stone::Black);

        let result = game.take_turn();

        assert_eq!(result, Err(ReversiError::NextPlayerCantPutStone));
        assert_eq!(game.turn(), Stone::Black);
    }
}
