#![allow(dead_code)]
enum Legend {
  North,
  NEast,
  East,
  SEast,
  South,
  SWest,
  West,
  NWest
}

enum SquareState {
  EMPTY, X, O, BORDER
}

struct Square<'a> {
  xpos: usize,
  ypos: usize,
  state: SquareState,
  compass: &'a mut [Square<'a>; 8]
}

impl<'a> Square<'a> {

    fn new() -> Self {
    
        Square{
            xpos: 0,
            ypos: 0,
            state: EMPTY,
            compass: &[]
        }
    }
}


pub struct Board<'a> {
  rows: usize,
  cols: usize,
  win_condition: u8,
  grid: Vec<Square<'a>>
}

impl<'a> Board<'a> {

    pub fn new( rows: usize, cols: usize ) -> Self {
    
        let gridv = Vec::<Square<'a>>::new();
        
        for x in 0 .. rows {
            for y in 0 .. cols {
            
                let index = (x * rows) + y;
                gridv[index] = Square::new()
            }
        }
        
        Board{
            rows: rows, 
            cols: cols, 
            win_condition: 3, 
            grid: gridv}
    }
}