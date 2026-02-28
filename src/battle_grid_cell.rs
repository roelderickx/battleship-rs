use crate::ship::Ship;

#[derive(Copy,Clone)]
pub enum BattleGridCell {
    Ocean(bool),
    Battleship(bool, Ship)
}

impl BattleGridCell {
    pub fn get_symbol(&self, is_player: bool) -> String {
        match self {
            BattleGridCell::Ocean(is_targeted) => {
                if *is_targeted {
                    String::from(" * ")
                }
                else {
                    String::from(" . ")
                }
            },
            BattleGridCell::Battleship(is_targeted, ship) => {
                if *is_targeted {
                    format!("*{}*", ship.get_symbol())
                }
                else if is_player {
                    format!("-{}-", ship.get_symbol())
                }
                else {
                    String::from(" . ")
                }
            }
        }
    }
    
    pub fn is_ship(&self) -> bool {
        match self {
            BattleGridCell::Ocean(_is_targeted) => false,
            BattleGridCell::Battleship(_is_targeted, _ship) => true
        }
    }
    
    pub fn is_targeted(&self) -> bool {
        match self {
            BattleGridCell::Ocean(is_targeted) => *is_targeted,
            BattleGridCell::Battleship(is_targeted, _ship) => *is_targeted
        }
    }
}

