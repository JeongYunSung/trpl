pub mod coin_crate;

use crate::coin::coin_crate::*;
use crate::coin::coin_crate::get_coin;
use crate::human::human_crate::get_human;

#[derive(Debug)] // so we can inspect the state in a minute
pub enum UsState {
    Alabama,
    Alaska,
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    JJM(i32, i64, f32)
}

pub fn value_in_cents(coin: Coin) -> u8 {
    get_coin();
    get_human();

    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
        Coin::JJM(x, y, z) => {
            println!("JJM {} {} {}", x, y, z);
            10
        }
    }
}