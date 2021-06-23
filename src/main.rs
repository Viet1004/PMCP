mod front_of_house;
mod restaurant;

fn main() {
    front_of_house::hosting::threetime_print();
    front_of_house::hosting::add_to_waitlist();

    restaurant::eat_at_restaurant();
}
