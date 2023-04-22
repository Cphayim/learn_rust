use crate::{back_of_house::Breakfast, front_of_house::hosting::add_to_wait_list};

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn seasonal_fruit(&self) -> &str {
            self.seasonal_fruit.as_str()
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn deliver_order() {}

pub fn eat_at_restaurant() {
    add_to_wait_list();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
