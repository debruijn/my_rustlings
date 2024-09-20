// Booleans (`bool`)

use std::ops::Not;

fn main() {
    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    let is_evening = is_morning.not();
    if is_evening {
        println!("Good evening!");
    }
}
