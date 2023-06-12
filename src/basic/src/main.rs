extern crate math;
extern crate trpl;

use math::add;
use trpl::lib_print;
use std::string::ToString;
use crate::human::Human;
use self::{coin::*, water::*, cup::*};

pub mod coin;
pub mod human;
pub mod cup;

fn main() {
    lib_print();

    let mut human = Human {
        name: "John".to_string(),
    };

    println!("Hello, {}!", human.get_name());

    human.set_name("Jane");

    println!("Hello, {}!", human.get_name());

    let mut newHuman = Human::new("JYS".to_string());

    println!("Hello, {}!", newHuman.get_name());

    value_in_cents(Coin::Quarter(UsState::Alaska));
    value_in_cents(Coin::JJM(1, 2, 3.5));

    drink();

    println!("Hello, {}!", add(1, 2));

    put_hand_in_cup();
    put_off();
}

mod water {
    pub fn drink() {
        println!("잘 마셨습니다");
    }
}