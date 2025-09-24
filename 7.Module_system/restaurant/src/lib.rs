use rand::{Rng, CryptoRng};

mod front_of_house;

pub fn eat_at_restaurant() {
    // Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    front_of_house::hosting::add_to_waitlist();
}

fn server_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::server_order();
    }

    fn cook_order() {}
}

mod back_of_house_another {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
}

pub fn eat_at_restaurant_another() {
    let mut meal = back_of_house_another::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
}

mod back_of_house_new {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_new() {
    let order1 = back_of_house_new::Appetizer::Soup;
    let order2 = back_of_house_new::Appetizer::Salad;
}

mod front_of_house_new {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub use self::front_of_house_new::hosting;

pub fn eat_at_restaurant_2() {
    hosting::add_to_waitlist();
}