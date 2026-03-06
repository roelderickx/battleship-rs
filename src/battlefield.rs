use crate::direction::Direction;
use crate::ship::Ship;
use crate::battlefield_position::BattlefieldPosition;

pub struct Battlefield {
    grid: [[BattlefieldPosition; 10]; 10]
}

impl Battlefield {
    pub fn create_player() -> Self {
        Self {
            grid: [[BattlefieldPosition::create_player(); 10]; 10]
        }
    }
    
    pub fn create_opponent() -> Self {
        Self {
            grid: [[BattlefieldPosition::create_opponent(); 10]; 10]
        }
    }
    
    /// Calculates if a given ship can be placed on x,y in the given direction
    /// The ship should not touch any other ship and should be completely inside the grid
    /// x and y must be in the range 0..=9
    fn can_position_ship(&self, ship: Ship, x: u8, y: u8, direction: Direction) -> bool {
        // ship must be inside battlefield
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
    /// Returns true if succeeded, false if the ship cannot be positioned as desired
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
                    self.save_position_information(x_pos, y_pos, ship, false);
                }
            }
            
            true
        }
        else {
            false
        }
    }
    
    pub fn reveal_position_information(&mut self, x: u8, y: u8) -> Ship {
        self.grid[x as usize][y as usize].set_targeted();
        self.grid[x as usize][y as usize].get_ship()
    }
    
    pub fn save_position_information(&mut self, x: u8, y: u8, ship: Ship, is_targeted: bool) {
        self.grid[x as usize][y as usize].save_position_information(ship, is_targeted);
    }
    
    pub fn all_ships_destroyed(&self) -> bool {
        let mut all_ships_length = 0;
        
        for ship in Ship::get_ship_list().into_iter() {
            all_ships_length += ship.get_length();
        }

        for y in 0..10 {
            for x in 0..10 {
                if self.grid[x][y].is_ship() {
                    all_ships_length -= 1;
                }
            }
        }

        all_ships_length == 0
    }

    pub fn print_line(&self, line_number: u8) {
        if line_number == 0 {
            print!("    1  2  3  4  5  6  7  8  9  10");
        }
        else {
            print!("{:2} ", line_number);
            for x in 0..10 {
                self.grid[x][(line_number as usize)-1].print_position();
            }
        }
    }
}

