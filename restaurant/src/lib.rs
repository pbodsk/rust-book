mod front_of_house;
pub use crate::front_of_house::hosting;

pub mod back_of_house;

pub fn eat_at_restaurant() {
    let toast = back_of_house::Toast::Rye;
    let breakfast = back_of_house::Breakfast::summer(toast);

    hosting::add_to_waitlist();

}