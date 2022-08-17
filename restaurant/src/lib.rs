mod front_of_house;

fn deliver_order() {}

mod back_of_the_house {

    pub enum Appetizer {
        Soup,
        Salad,
    }
    
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
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}


mod customer {
    use crate::front_of_house::hosting;
    use crate::back_of_the_house::*;

    pub fn eat_at_restaurant() {

        hosting::add_to_waitlist();

        let mut meal = Breakfast::summer("Rye");

        meal.toast = String::from("Wheat");

        println!("I'd like {} toast please", meal.toast);

        let order1 = Appetizer::Soup;
        let order2 = Appetizer::Salad;
        
    }
}
