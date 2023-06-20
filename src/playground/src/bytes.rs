pub struct ByteMut {
    bytes: Vec<u8>,
    cursor: usize,
}

impl ByteMut {
    pub fn new(bytes: Vec<u8>) -> ByteMut {
        ByteMut {
            bytes,
            cursor: 0,
        }
    }

    pub fn is_available(&self) -> bool {
        self.cursor < self.bytes.len()
    }
}