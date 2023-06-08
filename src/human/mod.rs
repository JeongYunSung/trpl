pub mod human_crate;

use crate::coin::coin_crate::get_coin;
use crate::human::human_crate::get_human;

pub struct Human {
    pub name: String,
}

impl Human {
    pub fn new(name: String) -> Human {
        Human { name }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_name(&self) -> &str {
        get_human();
        get_coin();

        self.name.as_str()
    }
}