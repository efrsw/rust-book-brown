mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() { }
    }
}

pub use front_of_house::hosting;

fn eat_at_restaurant() {
    // absolute -> /front_of_house/hosting/add_to_waitlist
    crate::front_of_house::hosting::add_to_waitlist();

    // relative -> ./hosting/add_to_waitlist
    front_of_house::hosting::add_to_waitlist();
}
