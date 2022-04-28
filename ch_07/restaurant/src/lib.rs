pub use crate::front_of_house::serving;

mod front_of_house;

mod back_of_house;

pub fn eat_breakfast() {
    let mut meal = back_of_house::Breakfast::summer("rye");
    meal.toast = String::from("wheat"); // Toast can be updated
    println!("I'd like {} toast please", meal.toast);
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist(); // Absolute path

    front_of_house::hosting::seat_at_table(); // Relative path

    serving::take_order(); // Using path brought into scope with use keyword
    serving::serve_order();
    serving::take_payment();
}

fn return_order() {} // Called by super
