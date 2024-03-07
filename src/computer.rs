use crate::{board::ReversiBoard, point::Point, stone::Stone};

pub enum PlayerType {
    Human,
    Computer(Box<dyn Computer>),
}

pub trait Computer {
    fn decide(&self, board: &dyn ReversiBoard) -> Point;
}

pub struct RandomComputer {
    color: Stone,
}

impl RandomComputer {
    pub fn new(color: Stone) -> Self {
        Self { color }
    }
}

impl Computer for RandomComputer {
    fn decide(&self, board: &dyn ReversiBoard) -> Point {
        let can_put_stones = board.get_can_put_stones(self.color);
        let index = rand::random::<usize>() % can_put_stones.len();
        can_put_stones[index]
    }
}

pub struct SimpleComputer {
    color: Stone,
}

impl SimpleComputer {
    pub fn new(color: Stone) -> Self {
        Self { color }
    }
}

impl Computer for SimpleComputer {
    fn decide(&self, board: &dyn ReversiBoard) -> Point {
        let can_put_stones = board.get_can_put_stones(self.color);

        let mut max_count: usize = 0;
        let mut max_index: usize = 0;

        for (i, p) in can_put_stones.iter().enumerate() {
            let count = board.count_flippable(p.x, p.y);
            if count > max_count {
                max_count = count;
                max_index = i;
            }
        }

        can_put_stones[max_index]
    }
}
