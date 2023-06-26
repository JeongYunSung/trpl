use std::num::Wrapping;
use std::ops::Index;

struct Recurrence {
    mem: [u64; 2],
    pos: usize,
}

struct IndexOffset<'a> {
    slice: &'a [u64; 2],
    offset: usize,
}

impl<'a> Index<usize> for IndexOffset<'a> {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        use std::num::Wrapping;

        let index = Wrapping(index);
        let offset = Wrapping(self.offset);
        let window = Wrapping(2);

        let real_index = index - offset + window;
        &self.slice[real_index.0]
    }
}

impl Iterator for Recurrence {
    type Item = u64;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < 2 {
            let next_val = self.mem[self.pos];
            self.pos += 1;
            Some(next_val)
        } else {
            let next_val = {
                let n = self.pos;
                let a = IndexOffset { slice: &self.mem, offset: n };
                a[n-2] + a[n-1]
            };
            {
                use std::mem::swap;

                let mut swap_tmp = next_val;
                for i in [1,0] {
                    swap(&mut swap_tmp, &mut self.mem[i]);
                }
            }
            self.pos += 1;
            Some(next_val)
        }
    }
}

pub fn fibonacci_test() {
    let recurrence = Recurrence { mem: [0, 1], pos: 0 };

    for e in recurrence.take(10) {
        println!("{e}");
    }
}