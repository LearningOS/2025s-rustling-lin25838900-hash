mod delicious_snacks {
    // 把 fruits::PEAR 引进来，改名 fruit
    pub use self::fruits::PEAR as fruit;
    // 把 veggies::CUCUMBER 引进来，改名 veggie
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
