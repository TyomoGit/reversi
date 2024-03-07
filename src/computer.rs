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
            let count = board.count_flippable(p.x, p.y, self.color);
            if count > max_count {
                max_count = count;
                max_index = i;
            }
        }

        can_put_stones[max_index]
    }
}

pub struct WeightedComputer {
    color: Stone,
}

impl WeightedComputer {
    pub fn new(color: Stone) -> Self {
        Self { color }
    }

    const WEIGHTS: [[i32; 8]; 8] = [
        [150, -50, 20, 10, 10, 20, -50, 150],
        [-50, -70, -3, -3, -3, -3, -70, -50],
        [20, -3, 3, 3, 3, 3, -3, 20],
        [10, -3, 3, 1, 1, 3, -3, 10],
        [10, -3, 3, 1, 1, 3, -3, 10],
        [20, -3, 3, 3, 3, 3, -3, 20],
        [-50, -70, -3, -3, -3, -3, -70, -50],
        [150, -50, 20, 10, 10, 20, -50, 150],
    ];
}

impl Computer for WeightedComputer {
    fn decide(&self, board: &dyn ReversiBoard) -> Point {
        let can_put_stones = board.get_can_put_stones(self.color);

        let mut max_count: i32 = i32::MIN;
        let mut max_index: usize = 0;

        for (i, p) in can_put_stones.iter().enumerate() {
            let mut cloned_board = dyn_clone::clone_box(board);
            let _ = cloned_board.put_stone(p.x, p.y, self.color);
            
            let (mut me, mut enemy): (i32, i32) = (0, 0);

            for (y, row) in cloned_board.board().iter().enumerate() {
                for (x, &stone) in row.iter().enumerate() {
                    if let Some(s) = stone {
                        if s == self.color {
                            me += Self::WEIGHTS[y][x];
                        } else if s == self.color.opposite() {
                            enemy += Self::WEIGHTS[y][x];
                        }
                    }
                }
            }

            let diff = me - enemy;
            dbg!(diff, can_put_stones[i]);
            if diff > max_count {
                max_count = diff;
                max_index = i;
            }
        }

        can_put_stones[max_index]
    }
}
