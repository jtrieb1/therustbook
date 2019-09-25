mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        
        fn seat_at_table() {}
    }
    
    pub mod serving {
        fn take_order() {}
        
        pub fn serve_order() {}
        
        fn take_payment() {}
    }

}

mod back_of_house {
    fn cook_order() {}
    
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
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
}

pub fn eat_at_restaurant() {
    // Order breakfast in the summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("rye");
    // Change our mind about the kind of toast we want
    meal.toast = String::from("wheat");
    println!("I'd like {} toast, please.", meal.toast);
    

}
