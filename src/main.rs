mod direction;
mod ship;
mod battle_grid_cell;
mod battle_grid;
mod battleship;

use battleship::Battleship;

fn main() {
    let mut b = Battleship::create();
    b.print();
    
    b.position_ships();
    
    b.play();
}

