#[derive(Copy,Clone)]
pub enum Ship {
    None,
    AircraftCarrier,
    Battleship,
    Cruiser,
    Submarine,
    Destroyer
}

impl Ship {
    pub fn get_ship_list() -> [Ship;5] {
        [
            Ship::AircraftCarrier,
            Ship::Battleship,
            Ship::Cruiser,
            Ship::Submarine,
            Ship::Destroyer
        ]
    }

    pub fn get_length(&self) -> u8 {
        match self {
            Ship::None => 0,
            Ship::AircraftCarrier => 5,
            Ship::Battleship => 4,
            Ship::Cruiser => 3,
            Ship::Submarine => 3,
            Ship::Destroyer => 2
        }
    }
    
    pub fn get_symbol(&self) -> char {
        match self {
            Ship::None => ' ',
            Ship::AircraftCarrier => 'A',
            Ship::Battleship => 'B',
            Ship::Cruiser => 'C',
            Ship::Submarine => 'S',
            Ship::Destroyer => 'D'
        }
    }
    
    pub fn get_name(&self) -> String {
        match self {
            Ship::None => String::from(""),
            Ship::AircraftCarrier => String::from("aircraft carrier"),
            Ship::Battleship => String::from("battleship"),
            Ship::Cruiser => String::from("cruiser"),
            Ship::Submarine => String::from("submarine"),
            Ship::Destroyer => String::from("destroyer")
        }
    }
}

