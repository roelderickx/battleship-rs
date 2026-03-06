use crate::direction::Direction;
use crate::ship::Ship;
use crate::battlefield::Battlefield;

use std::io;
use rand::Rng;

// FIXME Use traits?

pub struct Player {
    player_field: Battlefield,
    opponent_field: Battlefield
}

impl Player {
    fn create() -> Self {
        Self {
            player_field: Battlefield::create_player(),
            opponent_field: Battlefield::create_opponent()
        }
    }
    
    fn position_ship(&mut self, ship: Ship, x: u8, y: u8, direction: Direction) -> bool {
        self.player_field.position_ship(ship, x, y, direction)
    }
    
    fn attack_coordinate(&mut self, x: u8, y: u8) -> Ship {
        self.player_field.reveal_position_information(x, y)
    }
    
    fn all_opponent_ships_destroyed(&self) -> bool {
        return self.opponent_field.all_ships_destroyed();
    }

    fn print(&self) {
        println!("*** BATTLESHIP ***\n");
        let spaces = " ".repeat(28);
        println!("Player{}| Opponent", spaces);

        for y in 0..11 {
            self.player_field.print_line(y);
            print!(" | ");
            self.opponent_field.print_line(y);
            println!("");
        }
    }
}



pub struct HumanPlayer {
    base_player: Player
}

impl HumanPlayer {
    pub fn create() -> Self {
        Self {
            base_player: Player::create()
        }
    }
    
    pub fn get_player(&mut self) -> &mut Player {
        return &mut self.base_player;
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
        for ship in Ship::get_ship_list().into_iter() {
            // player position
            loop {
                let (x, y) = self.ask_coordinate(ship);
                let direction = self.ask_direction();
                if self.base_player.position_ship(ship, x, y, direction) {
                    break;
                }
                else {
                    println!("Illegal ship position.");
                    println!("Ships may not touch each other and must be placed inside the grid");
                }
            }

            self.print();
        }
    }
    
    /// Asks coordinates to attack the given opponent
    /// Returns true if all opponent's ships are destroyed
    pub fn attack(&mut self, opponent: &mut Player) -> bool {
        let (x, y) = self.ask_coordinate(Ship::None);
        let ship = opponent.attack_coordinate(x, y);

        self.base_player.opponent_field.save_position_information(x, y, ship, true);

        return self.base_player.all_opponent_ships_destroyed();
    }

    pub fn print(&self) {
        self.base_player.print();
    }
}



pub struct ComputerPlayer {
    base_player: Player
}

impl ComputerPlayer {
    pub fn create() -> Self {
        Self {
            base_player: Player::create()
        }
    }
    
    pub fn get_player(&mut self) -> &mut Player {
        return &mut self.base_player;
    }

    pub fn position_ships(&mut self) {
        for ship in Ship::get_ship_list().into_iter() {
            loop {
                let x = rand::rng().random_range(0..=(10 - ship.get_length()));
                let y = rand::rng().random_range(0..10);
                let direction = rand::rng().random_range(1..=2);

                if direction == 1 {
                    if self.base_player.position_ship(ship, x, y, Direction::Horizontal) {
                        break;
                    }
                }
                else {
                    if self.base_player.position_ship(ship, y, x, Direction::Vertical) {
                        break;
                    }
                }
            }
        }
    }
    
    /// Calculates coordinates to attack the given opponent
    /// Returns true if all opponent's ships are destroyed
    pub fn attack(&mut self, opponent: &mut Player) -> bool {
        // TODO make somewhat more intelligent
        // * A ship occupies at least 2 positions, do not shoot at two
        //   coordinates next to each other
        // * When the destroyer is sunk a ship occupies at least 3 positions
        // * When a ship is hit, the direction must be determined. In some
        //   cases one of both directions is impossible.
        let x = rand::rng().random_range(0..10);
        let y = rand::rng().random_range(0..10);
        let ship = opponent.attack_coordinate(x, y);

        self.base_player.opponent_field.save_position_information(x, y, ship, true);
        println!("Opponent shot at {},{}", x+1, y+1);

        return self.base_player.all_opponent_ships_destroyed();
    }

    pub fn print(&self) {
        self.base_player.print();
    }
}

