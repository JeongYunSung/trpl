pub trait Draw {
    fn draw(&self);
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Draw Select Box");
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw Button");
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}