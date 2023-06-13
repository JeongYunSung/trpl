extern crate object;

use object::*;

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
            }),
            Box::new(Button {
                width: 50,
                height: 10,
            }),
        ],
    };

    screen.run();
}