use std::fmt::Display;

use crate::{
    board::{ArrayBasedBoard, ReversiBoard}, computer::PlayerType, error::ReversiError, point::Point, stone::Stone
};

pub type Result<T> = std::result::Result<T, ReversiError>;

pub struct PlayerManager {
    black: PlayerType,
    white: PlayerType,
}

impl PlayerManager {
    pub fn new(black: PlayerType, white: PlayerType) -> Self {
        Self { black, white }
    }

    pub fn player(&self, stone: Stone) -> &PlayerType {
        match stone {
            Stone::Black => &self.black,
            Stone::White => &self.white,
        }
    }

    pub fn player_mut(&mut self, stone: Stone) -> &mut PlayerType {
        match stone {
            Stone::Black => &mut self.black,
            Stone::White => &mut self.white,
        }
    }

    pub fn decide(&self, board: &dyn ReversiBoard, turn: Stone) -> Option<Point> {
        let player = self.player(turn);

        match player {
            PlayerType::Human => None,
            PlayerType::Computer(computer) => Some(computer.decide(board)),
        }
    }
}

pub struct SimpleReversiGame {
    board: Box<dyn ReversiBoard>,
    turn: Stone,
}

impl SimpleReversiGame {
    pub fn new() -> Self {
        let mut board: Box<dyn ReversiBoard> = Box::new(ArrayBasedBoard::new());
        board.init_four_central_squares();

        Self {
            board,
            turn: Stone::Black,
        }
    }

    pub fn put_stone(&mut self, x: usize, y: usize) -> Result<()> {
        let result = self.board.put_stone(x, y, self.turn);

         let Err(ReversiError::NextPlayerCantPutStone(_)) = result else {
             self.take_turn();
             return result;
         };

         result
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

    #[inline]
    pub fn board(&self) -> &dyn ReversiBoard {
        self.board.as_ref()
    }

    #[inline]
    pub fn board_mut(&mut self) -> &mut dyn ReversiBoard {
        self.board.as_mut()
    }

    #[inline]
    pub fn take_turn(&mut self) {
        self.turn = self.turn.opposite();
    }


    #[inline]
    pub fn set_turn(&mut self, turn: Stone) {
        self.turn = turn;
    }

    #[inline]
    pub fn turn(&self) -> Stone {
        self.turn
    }
}

impl Default for SimpleReversiGame {
    fn default() -> Self {
        Self::new()
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

    use crate::computer::WeightedComputer;

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
            Err(ReversiError::NextPlayerCantPutStone(Stone::White))
        );
    }

    #[test]
    fn cant_put_2() {
        let mut game = SimpleReversiGame::new();
        let mut player_mgr = PlayerManager::new(
            PlayerType::Human,
            PlayerType::Computer(Box::new(WeightedComputer::new(Stone::White)))
        );

        *game.board.board_mut() = vec![vec![None; 8]; 8];
        game.board.board_mut()[0][0] = Some(Stone::Black);
        game.board.board_mut()[0][1] = Some(Stone::White);

        game.board.board_mut()[7][0] = Some(Stone::Black);
        game.board.board_mut()[7][1] = Some(Stone::White);

        dbg!(&game.board);
        let result = game.put_stone(2, 0);
        assert_eq!(result, Err(ReversiError::NextPlayerCantPutStone(Stone::White)));
        assert_eq!(game.turn(), Stone::Black);
    }
}
