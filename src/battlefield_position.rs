use crate::ship::Ship;

use colored_text::Colorize;

#[derive(Copy,Clone)]
pub struct BattlefieldPosition {
    is_unknown: bool,
    is_targeted: bool,
    ship: Ship
}

impl BattlefieldPosition {
    pub fn create_player() -> Self {
        Self {
            is_unknown: false,
            is_targeted: false,
            ship: Ship::None
        }
    }

    pub fn create_opponent() -> Self {
        Self {
            is_unknown: true,
            is_targeted: false,
            ship: Ship::None
        }
    }

    pub fn is_ship(&self) -> bool {
        self.ship.get_length() > 0
    }

    pub fn get_ship(&self) -> Ship {
        self.ship
    }

    pub fn is_targeted(&self) -> bool {
        self.is_targeted
    }

    pub fn set_targeted(&mut self) {
        self.is_targeted = true;
    }

    pub fn save_position_information(&mut self, ship: Ship, is_targeted: bool) {
        self.is_unknown = false;
        self.is_targeted = is_targeted;
        self.ship = ship;
    }

    pub fn print_position(&self) {
        if self.is_unknown {
            print!(" {} ", ".".bright_white());
        }
        else if self.is_targeted {
            if self.is_ship() {
                print!(" {} ", self.ship.get_symbol().to_string().bright_red());
            }
            else {
                print!(" {} ", "*".bright_red());
            }
        }
        else {
            if self.is_ship() {
                print!(" {} ", self.ship.get_symbol().bright_green());
            }
            else {
                print!(" {} ", "~".bright_blue());
            }
        }
    }
}

