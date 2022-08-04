// use rand::Rng;
// use rand::ErrorKind::Transient;
// use rand::CryptoRng;

use rand::{CryptoRng, ErrorKind::Transient, Rng};

// use std::io::{self, Write};
use std::io::*;

mod front_of_house;

pub use front_of_house::hosting;

pub fn eat_at_restaurant() {
    let secret_number = rand::thread_rng().gen_range(1, 100);
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
