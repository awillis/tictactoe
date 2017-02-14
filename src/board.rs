#![allow(dead_code)]
enum Legend {
    North,
    NEast,
    East,
    SEast,
    South,
    SWest,
    West,
    NWest,
}

enum SquareState {
    EMPTY,
    X,
    O,
    BORDER,
}

type Compass = Option<[Square; 8]>;

struct Square {
    xpos: usize,
    ypos: usize,
    state: SquareState,
    compass: Box<Compass>,
}

impl Square {
    fn new() -> Self {

        Square {
            xpos: 0,
            ypos: 0,
            state: SquareState::EMPTY,
            compass: Box::new(None),
        }
    }
}

pub struct Board {
    rows: usize,
    cols: usize,
    win_condition: u8,
    grid: Vec<Square>,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Self {

        let mut gridv = Vec::<Square>::new();

        for x in 0..rows {
            for y in 0..cols {

                let index = (x * rows) + y;

                gridv[index] = Square::new()
            }
        }

        Board {
            rows: rows,
            cols: cols,
            win_condition: 3,
            grid: gridv,
        }
    }
}
