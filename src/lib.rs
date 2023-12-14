mod front_of_house {
    pub mod hosting {
        pub(crate) fn add_to_waitlist() {}

        fn _seat_at_table() {}
    }

    mod serving {
        fn _take_order() {}

        fn _serve_order() {}

        fn _take_payment() {}
    }
}

pub use self::front_of_house::hosting;
// use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Use keyword
    hosting::add_to_waitlist();
}
