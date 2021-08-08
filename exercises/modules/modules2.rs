// modules2.rs
// Make me compile! Execute `rustlings hint modules2` for hints :)


mod delicious_snacks {

    pub mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

use self::delicious_snacks::veggies::CUCUMBER as veggie;
use self::delicious_snacks::fruits::PEAR as fruit;


fn main() {
    println!(
        "favorite snacks: {} and {}",
        fruit,
        veggie
    );
}
