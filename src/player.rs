use crate::direction::Direction;
use crate::ship::Ship;
use crate::battlefield::Battlefield;

use std::io;
use rand::Rng;

// FIXME Use traits?

pub struct Player {
    name: String,
    player_field: Battlefield,
    opponent_field: Battlefield
}

impl Player {
    fn create(name: &str) -> Self {
        Self {
            name: name.to_string(),
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
        let spaces = " ".repeat(34-self.name.chars().count());
        println!("{}{}| Opponent", self.name, spaces);

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
    pub fn create(name: &str) -> Self {
        Self {
            base_player: Player::create(name)
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
    base_player: Player,
    first_shot_x: u8,
    first_shot_y: u8,
    current_ship_attacked: Ship,
    current_ship_destroyed: u8,
    first_ship_x: u8,
    first_ship_y: u8
}

impl ComputerPlayer {
    pub fn create(name: &str) -> Self {
        Self {
            base_player: Player::create(name),
            first_shot_x: 255,
            first_shot_y: 255,
            current_ship_attacked: Ship::None,
            current_ship_destroyed: 0,
            first_ship_x: 255,
            first_ship_y: 255
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

    fn is_valid_attack_direction(&self, direction: Direction) -> bool {
        let mut start_coord = self.first_ship_x;
        if direction.is_vertical() {
            start_coord = self.first_ship_y;
        }

        // get valid range
        let mut min_range = start_coord;
        if min_range >= self.current_ship_attacked.get_length() {
            min_range -= self.current_ship_attacked.get_length() - 1;
        }
        else {
            min_range = 0;
        }
        let mut max_range = start_coord;
        if max_range <= 10-self.current_ship_attacked.get_length() {
            max_range += self.current_ship_attacked.get_length();
        }
        else {
            max_range = 10;
        }

        // verify the complete range
        let mut max_length = 0;
        for coord_test in min_range..max_range {
            if (direction.is_horizontal() &&
                self.base_player.opponent_field.is_targeted(coord_test, self.first_ship_y) &&
                self.base_player.opponent_field.get_ship(coord_test, self.first_ship_y)
                        != self.current_ship_attacked) ||
               (direction.is_vertical() &&
                self.base_player.opponent_field.is_targeted(self.first_ship_x, coord_test) &&
                self.base_player.opponent_field.get_ship(self.first_ship_x, coord_test)
                        != self.current_ship_attacked)
            {
                max_length = 0;
            }
            else {
                max_length += 1;
            }
            if max_length == self.current_ship_attacked.get_length() {
                return true;
            }
        }

        false
    }

    /// Calculates coordinates to attack the given opponent
    /// Returns true if all opponent's ships are destroyed
    pub fn attack(&mut self, opponent: &mut Player) -> bool {
        let mut x = 255;
        let mut y = 255;

        if self.first_shot_x == 255 && self.first_shot_y == 255 {
            // The very first shot, pick a random location
            x = rand::rng().random_range(0..10);
            y = rand::rng().random_range(0..10);

            self.first_shot_x = x;
            self.first_shot_y = y;
        }
        else if self.current_ship_attacked.get_length() == 0 {
            // The next shot, no ship has been hit before
            // Pick a location which is not targeted before
            // and a multiple of 2 away from the first shot in both directions
            // and not next to an already found ship
            let mut valid_targets: Vec<(u8, u8)> = Vec::new();
            for y_loop in 0..10 {
                for x_loop in 0..5 {
                    let mut coord_x = x_loop * 2;
                    if self.first_shot_x % 2 == 0 {
                        coord_x += y_loop % 2;
                    }
                    else {
                        coord_x += self.first_shot_x % 2 - y_loop % 2;
                    }
                    let coord_y = y_loop;
                    // if not targeted and not next to a ship
                    if !self.base_player.opponent_field.is_targeted(coord_x, coord_y) &&
                       self.base_player.opponent_field.can_position_ship(1, coord_x, coord_y,
                                                                         Direction::Horizontal)
                    {
                        valid_targets.push((coord_x, coord_y));
                    }
                }
            }

            let index = rand::rng().random_range(0..valid_targets.len());
            let coord = valid_targets.get(index);
            match coord {
                Some(coord) => (x, y) = *coord,
                None => println!("*** DEBUG: coordinate not found"),
            }
        }
        else if self.is_valid_attack_direction(Direction::Horizontal) {
            // A ship had been hit in the previous step and it may be lying horizontally
            // Try left
            x = self.first_ship_x;
            loop {
                if x == 0 {
                    x = 255;
                    break;
                }
                x -= 1;
                if self.first_ship_x >= self.current_ship_attacked.get_length() &&
                   x == self.first_ship_x - self.current_ship_attacked.get_length()
                {
                    x = 255;
                    break;
                }
                if !self.base_player.opponent_field.is_targeted(x, self.first_ship_y) {
                    break;
                }
                else if self.base_player.opponent_field.get_ship(x, self.first_ship_y)
                                == Ship::None
                {
                    x = 255;
                    break;
                }
            }

            // Try right
            if x == 255 {
                x = self.first_ship_x;
                loop {
                    x += 1;
                    if x == 10 ||
                       x == self.first_ship_x + self.current_ship_attacked.get_length()
                    {
                        x = 255;
                        break;
                    }
                    if !self.base_player.opponent_field.is_targeted(x, self.first_ship_y) {
                        break;
                    }
                    else if self.base_player.opponent_field.get_ship(x, self.first_ship_y)
                                    == Ship::None
                    {
                        x = 255;
                        break;
                    }
                }
            }
            if x == 255 {
                println!("*** DEBUG: Computer tought horizontal direction is possible but it aint");
            }
            y = self.first_ship_y;
        }
        else if self.is_valid_attack_direction(Direction::Vertical) {
            // A ship had been hit in the previous step and it may be lying vertically
            // Try up
            y = self.first_ship_y;
            loop {
                if y == 0 {
                    y = 255;
                    break;
                }
                y -= 1;
                if self.first_ship_y >= self.current_ship_attacked.get_length() &&
                   y == self.first_ship_y - self.current_ship_attacked.get_length()
                {
                    y = 255;
                    break;
                }
                if !self.base_player.opponent_field.is_targeted(self.first_ship_x, y) {
                    break;
                }
                else if self.base_player.opponent_field.get_ship(self.first_ship_x, y)
                                == Ship::None
                {
                    y = 255;
                    break;
                }
            }

            // Try down
            if y == 255 {
                y = self.first_ship_y;
                loop {
                    y += 1;
                    if y == 10 ||
                       y == self.first_ship_y + self.current_ship_attacked.get_length()
                    {
                        y = 255;
                        break;
                    }
                    if !self.base_player.opponent_field.is_targeted(self.first_ship_x, y) {
                        break;
                    }
                    else if self.base_player.opponent_field.get_ship(self.first_ship_x, y)
                                    == Ship::None
                    {
                        y = 255;
                        break;
                    }
                }
            }
            if y == 255 {
                println!("*** DEBUG: Computer tought vertical direction is possible but it aint");
            }
            x = self.first_ship_x;
        }

        let ship = opponent.attack_coordinate(x, y);

        if self.current_ship_attacked.get_length() == 0 && ship.get_length() > 0 {
            // opponent ship found
            self.current_ship_attacked = ship;
            self.current_ship_destroyed = 1;
            self.first_ship_x = x;
            self.first_ship_y = y;
        }
        else if self.current_ship_attacked == ship {
            self.current_ship_destroyed += 1;
            if self.current_ship_attacked.get_length() == self.current_ship_destroyed {
                self.current_ship_attacked = Ship::None;
                self.current_ship_destroyed = 0;
                self.first_ship_x = 255;
                self.first_ship_y = 255;
            }
        }

        self.base_player.opponent_field.save_position_information(x, y, ship, true);
        println!("{} shot at {},{}", self.base_player.name, x+1, y+1);

        return self.base_player.all_opponent_ships_destroyed();
    }

    pub fn print(&self) {
        self.base_player.print();
    }
}

