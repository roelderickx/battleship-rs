mod direction;
mod ship;
mod battlefield_position;
mod battlefield;
mod player;

use player::HumanPlayer;
use player::ComputerPlayer;

fn main() {
    let mut h = HumanPlayer::create("Player");
    let mut c = ComputerPlayer::create("Computer");
    h.print();

    h.position_ships();
    c.position_ships();

    loop {
        // player
        if h.attack(c.get_player()) {
            h.print();
            println!("Player wins!");
            break;
        }

        // opponent
        if c.attack(h.get_player()) {
            c.print();
            println!("Computer wins!");
            break;
        }

        h.print();
    }
}

