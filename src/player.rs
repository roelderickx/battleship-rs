use crate::direction::Direction;
use crate::ship::Ship;
use crate::battlefield::Battlefield;

use std::io;
use rand::RngExt;

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
        let spaces = " ".repeat(25-self.name.chars().count());
        println!("{}{}| Opponent", self.name, spaces);

        for y in 0..11 {
            self.player_field.print_line(y);
            print!("  | ");
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

    fn ask_manual_positioning(&self) -> bool {
        loop {
            println!("Ship positioning: (A)utomatically on random spots or (M)anually");

            let mut direction = String::new();

            io::stdin()
                .read_line(&mut direction)
                .expect("Failed to read line");

            let first_char = direction.chars().nth(0).unwrap();

            if first_char == 'A' || first_char == 'a' {
                return false;
            }
            else if first_char == 'M' || first_char == 'm' {
                return true;
            }
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

    fn position_ships_automated(&mut self) {
        for ship in Ship::get_ship_list().into_iter() {
            // player position
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

        self.print();
    }

    pub fn position_ships(&mut self) {
        // TODO: ask automated or manual
        if self.ask_manual_positioning() {
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
                        print!("Ships may not touch each other and must be placed entirely \
                                inside the battlefield");
                    }
                }

                self.print();
            }
        }
        else {
            self.position_ships_automated();
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
    attack_even_squares: u8,
    current_ship_attacked: Ship,
    current_ship_destroyed: u8,
    first_ship_x: u8,
    first_ship_y: u8,
    first_direction: usize
}

impl ComputerPlayer {
    pub fn create(name: &str) -> Self {
        Self {
            base_player: Player::create(name),
            attack_even_squares: rand::rng().random_range(0..2),
            current_ship_attacked: Ship::None,
            current_ship_destroyed: 0,
            first_ship_x: 255,
            first_ship_y: 255,
            first_direction: 0
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

    fn get_coord_value(&self, x: u8, y: u8, direction: Direction) -> u8 {
        if direction.is_horizontal() {
            x
        }
        else {
            y
        }
    }

    fn is_valid_attack_direction(&self, direction: Direction) -> bool {
        let start_coord = self.get_coord_value(self.first_ship_x, self.first_ship_y, direction);

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
            let x_t = self.get_coord_value(coord_test, self.first_ship_x, direction);
            let y_t = self.get_coord_value(self.first_ship_y, coord_test, direction);

            if self.base_player.opponent_field.is_targeted(x_t, y_t) &&
               self.base_player.opponent_field.get_ship(x_t, y_t) != self.current_ship_attacked
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

    fn get_coord_attack_axis(&self, first_x: u8, first_y: u8, direction: Direction) -> (u8, u8) {
        // Try left or up
        let first_coord = self.get_coord_value(first_x, first_y, direction);
        let mut coord = first_coord;

        loop {
            if coord == 0 {
                coord = 255;
                break;
            }
            coord -= 1;
            if first_coord >= self.current_ship_attacked.get_length() &&
               coord == first_coord - self.current_ship_attacked.get_length()
            {
                coord = 255;
                break;
            }

            let x_t = self.get_coord_value(coord, first_x, direction);
            let y_t = self.get_coord_value(first_y, coord, direction);
            if !self.base_player.opponent_field.is_targeted(x_t, y_t) {
                break;
            }
            else if self.base_player.opponent_field.get_ship(x_t, y_t) == Ship::None
            {
                coord = 255;
                break;
            }
        }

        // Try right or down
        if coord == 255 {
            coord = first_coord;

            loop {
                coord += 1;
                if coord == 10 ||
                   coord == first_coord + self.current_ship_attacked.get_length()
                {
                    coord = 255;
                    break;
                }

                let x_t = self.get_coord_value(coord, first_x, direction);
                let y_t = self.get_coord_value(first_y, coord, direction);
                if !self.base_player.opponent_field.is_targeted(x_t, y_t) {
                    break;
                }
                else if self.base_player.opponent_field.get_ship(x_t, y_t) == Ship::None
                {
                    coord = 255;
                    break;
                }
            }
        }

        if coord == 255 {
            print!("*** DEBUG: Computer validated ");
            if direction.is_horizontal() {
                print!("horizontal");
            }
            else {
                print!("vertical");
            }
            println!(" direction must be possible but no valid coordinate found");
        }

        let x = self.get_coord_value(coord, first_x, direction);
        let y = self.get_coord_value(first_y, coord, direction);
        (x, y)
    }

    /// Calculates coordinates to attack the given opponent
    /// Returns true if all opponent's ships are destroyed
    pub fn attack(&mut self, opponent: &mut Player) -> bool {
        let mut x = 255;
        let mut y = 255;

        if self.current_ship_attacked.get_length() == 0 {
            // The next shot, no ship has been hit before
            // Pick a location which is not targeted before
            // and a multiple of 2 away from the first shot in both directions
            // and not next to an already found ship
            let mut valid_targets: Vec<(u8, u8)> = Vec::new();
            for y_loop in 0..10 {
                for x_loop in 0..10 {
                    if (x_loop + y_loop) % 2 == self.attack_even_squares &&
                       !self.base_player.opponent_field.is_targeted(x_loop, y_loop) &&
                       self.base_player.opponent_field.can_position_ship(1, x_loop, y_loop,
                                                                         Direction::Horizontal)
                    {
                        valid_targets.push((x_loop, y_loop));
                    }
                }
            }

            let index = rand::rng().random_range(0..valid_targets.len());
            let coord = valid_targets.get(index);
            match coord {
                Some(coord) => (x, y) = *coord,
                None => println!("*** DEBUG: coordinate at index {} not found", index),
            }
        }
        else {
            let attack_direction = [ Direction::Horizontal, Direction::Vertical ];

            for _i in 0..2 {
                if self.is_valid_attack_direction(attack_direction[self.first_direction]) {
                    // A ship had been hit in the previous step
                    // and it may be lying in 1 direction
                    (x, y) = self.get_coord_attack_axis(self.first_ship_x, self.first_ship_y,
                                                        attack_direction[self.first_direction]);
                }
                else if self.is_valid_attack_direction(attack_direction[1 - self.first_direction]) {
                    // A ship had been hit in the previous step
                    // and it may be lying in the other direction
                    (x, y) = self.get_coord_attack_axis(self.first_ship_x, self.first_ship_y,
                                                        attack_direction[1 - self.first_direction]);
                }
            }
        }

        let ship = opponent.attack_coordinate(x, y);

        if self.current_ship_attacked.get_length() == 0 && ship.get_length() > 0 {
            // opponent ship found
            self.current_ship_attacked = ship;
            self.current_ship_destroyed = 1;
            self.first_ship_x = x;
            self.first_ship_y = y;
            self.first_direction = rand::rng().random_range(0..2);
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

