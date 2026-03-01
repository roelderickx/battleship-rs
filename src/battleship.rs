use crate::direction::Direction;
use crate::ship::Ship;
use crate::battle_grid::BattleGrid;

use std::io;
use rand::Rng;

pub struct Battleship {
    player_grid: BattleGrid,
    opponent_grid: BattleGrid
}

impl Battleship {
    pub fn create() -> Self {
        Self {
            player_grid: BattleGrid::create(true),
            opponent_grid: BattleGrid::create(false)
        }
    }

    /// Asks for a coordinate x,y where both x and y are in range 1..=10
    /// Returns (x, y) as range 0..=9
    fn ask_coordinate(&self, ship: Ship) -> (u8, u8) {
        loop {
            if ship.get_length() > 0 {
                println!("Enter x,y coordinate of {} (length {})",
                         ship.get_name(), ship.get_length());
            }
            else {
                println!("Enter x,y coordinate");
            }

            let mut coordinate = String::new();

            io::stdin()
                .read_line(&mut coordinate)
                .expect("Failed to read line");

            let mut coordinates = coordinate.trim().split(',');
            
            let x: u8 = match coordinates.next() {
                Some(val) => {
                    match val.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    }
                },
                None => continue,
            };
            
            let y: u8 = match coordinates.next() {
                Some(val) => {
                    match val.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    }
                },
                None => continue,
            };
            
            if x >= 1 && y <= 10 && x >= 1 && y <= 10 {
                return (x-1, y-1)
            }
        }
    }
    
    fn ask_direction(&self) -> Direction {
        loop {
            println!("Enter direction: (H)orizontally or (V)ertically");

            let mut direction = String::new();

            io::stdin()
                .read_line(&mut direction)
                .expect("Failed to read line");
            
            let first_char = direction.chars().nth(0).unwrap();
            
            if first_char == 'H' || first_char == 'h' {
                return Direction::Horizontal;
            }
            else if first_char == 'V' || first_char == 'v' {
                return Direction::Vertical;
            }
        }
    }

    pub fn position_ships(&mut self) {
        let ships = [
            Ship::AircraftCarrier,
            Ship::Battleship,
            Ship::Cruiser,
            Ship::Submarine,
            Ship::Destroyer
        ];

        for ship in ships.into_iter() {
            // player position
            loop {
                let (x, y) = self.ask_coordinate(ship);
                let direction = self.ask_direction();
                if self.player_grid.position_ship(ship, x, y, direction) {
                    break;
                }
            }

            // opponent position
            loop {
                let x = rand::rng().random_range(0..=(10 - ship.get_length()));
                let y = rand::rng().random_range(0..10);
                let direction = rand::rng().random_range(1..=2);

                if direction == 1 {
                    if self.opponent_grid.position_ship(ship, x, y, Direction::Horizontal) {
                        break;
                    }
                }
                else {
                    if self.opponent_grid.position_ship(ship, y, x, Direction::Vertical) {
                        break;
                    }
                }
            }

            self.print();
        }
    }
    
    pub fn play(&mut self) {
        loop {
            // player
            let (x, y) = self.ask_coordinate(Ship::None);
            self.opponent_grid.shoot(x, y);

            if self.opponent_grid.all_ships_destroyed() {
                self.print();
                println!("Player wins!");
                break;
            }
            
            // opponent
            // TODO make somewhat more intelligent
            let x = rand::rng().random_range(0..10);
            let y = rand::rng().random_range(0..10);
            self.player_grid.shoot(x, y);
            self.print();

            if self.player_grid.all_ships_destroyed() {
                println!("Opponent wins!");
                break;
            }
        }
    }

    pub fn print(&self) {
        println!("*** BATTLESHIP ***\n");
        let spaces = " ".repeat(28);
        println!("Player{}| Opponent", spaces);

        for y in 0..11 {
            self.player_grid.print_line(y);
            print!(" | ");
            self.opponent_grid.print_line(y);
            println!("");
        }
    }
}

