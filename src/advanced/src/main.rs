mod generic;
mod my_trait;
mod lifetime;

use generic::print as generic_print;
use my_trait::print as trait_print;
use lifetime::print as lifetime_print;

fn main() {
    generic_print();
    trait_print();
    lifetime_print();
}
