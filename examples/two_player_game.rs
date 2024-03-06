use reversi::{
    game::{ReversiGameError, SimpleReversiGame},
    player::PlayerKind,
};
use std::io::stdin;

fn main() {
    let mut game = SimpleReversiGame::new();

    loop {
        println!("{}", &game);
        println!(
            "{}'s turn",
            if game.board().turn() == PlayerKind::Black {
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
            ReversiGameError::StoneAlreadyPlaced
            | ReversiGameError::IndexOutOfBound
            | ReversiGameError::InvalidMove
            | ReversiGameError::NoStoneToFlip => {
                println!("{:?}", error);
            }

            ReversiGameError::GameOverWithWinner(winner) => {
                println!("{} wins!", winner);
                println!("{}", &game);
                break;
            }

            ReversiGameError::GameOverWithDraw => {
                println!("Draw!");
                println!("{}", &game);
                break;
            }
        }
    }
}
