use reversi::{
    computer::{PlayerType, WeightedComputer}, error::ReversiError, game::{PlayerManager, SimpleReversiGame}, stone::Stone
};
use std::io::stdin;

fn main() {
    let mut game = SimpleReversiGame::new();
    let player_mgr = PlayerManager::new(
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

        let (x, y) = if let Some(point) = player_mgr.decide(game.board(), game.turn()) {
            (point.x, point.y)
        } else {
            let Some(xy) = read_input() else {
                continue;
            };

            xy
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

            ReversiError::NextPlayerCantPutStone(stone) => {
                println!("{:?}: {} cannot put stone.", error, stone);
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

fn read_input() -> Option<(usize, usize)> {
    let mut buff = String::new();
        stdin().read_line(&mut buff).unwrap();
        let mut split = buff.trim().split(' ');

        let Some(x) = split.next() else {
            return None;
        };
        let Some(y) = split.next() else {
            return None;
        };

        let Some(x) = x.chars().next() else {
            return None;
        };
        let Ok(y) = y.parse::<usize>() else {
            return None;
        };
        let y = y - 1;

        let x = if x.is_ascii_uppercase() {
            x as usize - 'A' as usize
        } else {
            x as usize - 'a' as usize
        };

        Some((x, y))
}
