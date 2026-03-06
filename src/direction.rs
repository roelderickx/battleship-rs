#[derive(Copy,Clone)]
pub enum Direction {
    Horizontal,
    Vertical
}

impl Direction {
    pub fn is_horizontal(&self) -> bool {
        match self {
            Direction::Horizontal => true,
            Direction::Vertical => false
        }
    }
    
    pub fn is_vertical(&self) -> bool {
        !self.is_horizontal()
    }
}

