use crate::direction::Direction;
use crate::ship::Ship;
use crate::battle_grid_cell::BattleGridCell;

pub struct BattleGrid {
    grid: [[BattleGridCell; 10]; 10],
    is_player: bool
}

impl BattleGrid {
    pub fn create(is_player: bool) -> Self {
        Self {
            grid: [[BattleGridCell::Ocean(false); 10]; 10],
            is_player
        }
    }
    
    /// Calculates if a given ship can be placed on x,y in the given direction
    /// The ship should not touch any other ship and should be completely inside the grid
    /// x and y must be in the range 0..9
    fn can_position_ship(&self, ship: Ship, x: u8, y: u8, direction: Direction) -> bool {
        // ship must be inside battlegrid
        if (direction.is_horizontal() && x + ship.get_length() - 1 > 9) ||
           (direction.is_vertical() && y + ship.get_length() - 1 > 9) {
            return false;
        }

        // Ship may not touch any other ship
        // Calculate zone around given ship
        let mut x1 = x;
        if x1 > 0 {
            x1 -= 1;
        }
        let mut y1 = y;
        if y1 > 0 {
            y1 -= 1;
        }
        let mut x2 = x + 1;
        if direction.is_horizontal() {
            x2 += ship.get_length() - 1;
        }
        if x2 > 9 {
            x2 = 9;
        }
        let mut y2 = y + 1;
        if direction.is_vertical() {
            y2 += ship.get_length() - 1;
        }
        if y2 > 9 {
            y2 = 9;
        }
        // Check if no other ship is present in the calculated zone
        for y_test in y1..=y2 {
            for x_test in x1..=x2 {
                if self.grid[x_test as usize][y_test as usize].is_ship() {
                    return false;
                }
            }
        }

        true
    }
    
    /// Positions given ship at x,y in the given direction
    /// Returns true if succeeded
    pub fn position_ship(&mut self, ship: Ship, x: u8, y: u8, direction: Direction) -> bool {
        if self.can_position_ship(ship, x, y, direction) {
            // Calculate ship zone
            let mut x2 = x + 1;
            if direction.is_horizontal() {
                x2 += ship.get_length() - 1;
            }
            let mut y2 = y + 1;
            if direction.is_vertical() {
                y2 += ship.get_length() - 1;
            }
            for y_pos in y..y2 {
                for x_pos in x..x2 {
                    self.grid[x_pos as usize][y_pos as usize] =
                            BattleGridCell::Battleship(false, ship);
                }
            }
            
            true
        }
        else {
            if self.is_player {
                println!("Illegal ship position.");
                println!("Ships may not touch each other and must be placed inside the grid");
            }
            
            false
        }
    }
    
    /// Shoot at given coordinate
    pub fn shoot(&mut self, x: u8, y: u8) {
        match self.grid[x as usize][y as usize] {
            BattleGridCell::Ocean(_is_targeted) => {
                self.grid[x as usize][y as usize] = BattleGridCell::Ocean(true);
            },
            BattleGridCell::Battleship(_is_targeted, ship) => {
                self.grid[x as usize][y as usize] = BattleGridCell::Battleship(true, ship);
            }
        }
    }
    
    pub fn all_ships_destroyed(&self) -> bool {
        for y in 0..10 {
            for x in 0..10 {
                if self.grid[x][y].is_ship() && !self.grid[x][y].is_targeted() {
                    return false;
                }
            }
        }
        true
    }

    pub fn print_line(&self, line_number: u8) {
        if line_number == 0 {
            print!("    1  2  3  4  5  6  7  8  9  10");
        }
        else {
            print!("{:2} ", line_number);
            for x in 0..10 {
                print!("{}", self.grid[x][(line_number as usize)-1].get_symbol(self.is_player));
            }
        }
    }
}

