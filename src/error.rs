use crate::stone::Stone;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReversiError {
    StoneAlreadyPlaced,
    InvalidMove,
    IndexOutOfBound,
    NoStoneToFlip,
    NextPlayerCantPutStone(Stone),

    GameOverWithWinner(Stone),
    GameOverWithDraw,
}
