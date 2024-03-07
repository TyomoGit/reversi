use reversi::{
    board::ReversiError,
    computer::{PlayerType, SimpleComputer, WeightedComputer},
    game::SimpleReversiGame,
    stone::Stone,
};
use std::io::stdin;

fn main() {
    let mut game = SimpleReversiGame::new(
        PlayerType::Human,
        PlayerType::Computer(Box::new(WeightedComputer::new(Stone::White))),
    );

    loop {
        println!("{}", &game);
        println!(
            "{}'s turn",
            if game.turn() == Stone::Black {
                "⚫︎"
            } else {
                "⚪︎"
            }
        );

        let mut buff = String::new();
        stdin().read_line(&mut buff).unwrap();
        let mut split = buff.trim().split(' ');

        let Some(x) = split.next() else {
            continue;
        };
        let Some(y) = split.next() else {
            continue;
        };

        let Some(x) = x.chars().next() else {
            continue;
        };
        let Ok(y) = y.parse::<usize>() else {
            continue;
        };
        let y = y - 1;

        let x = if x.is_ascii_uppercase() {
            x as usize - 'A' as usize
        } else {
            x as usize - 'a' as usize
        };

        let Err(error) = game.put_stone(x, y) else {
            continue;
        };

        match error {
            ReversiError::StoneAlreadyPlaced
            | ReversiError::IndexOutOfBound
            | ReversiError::InvalidMove
            | ReversiError::NoStoneToFlip => {
                println!("{:?}", error);
            }

            ReversiError::NextPlayerCantPutStone => {
                game.take_turn().unwrap();
                println!("{:?}", error);
            }

            ReversiError::GameOverWithWinner(winner) => {
                println!("{} wins!", winner);
                println!("{}", &game);
                break;
            }

            ReversiError::GameOverWithDraw => {
                println!("Draw!");
                println!("{}", &game);
                break;
            }
        }
    }
}
