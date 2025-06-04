use std::{cmp::Ordering, iter};

use crate::Program;

impl Program {
    pub(crate) fn add(&mut self, n: i32) {
        if self.optimized {
            self.add_optimized(n)
        } else {
            self.add_unoptimized(n)
        }
    }
    pub(crate) fn goto(&mut self, i: usize) {
        if self.optimized {
            self.goto_optimized(i);
        } else {
            self.goto_unoptimized(i);
        }
    }
    pub(crate) fn set_zero(&mut self) {
        self.out.push_str("[-]");
    }
    fn add_unoptimized(&mut self, n: i32) {
        let c = match n.signum() {
            1 => '+',
            -1 => '-',
            _ => return,
        };
        let out: String = iter::repeat_n(c, n.abs() as usize).collect();
        self.out.push_str(&out);
    }
    fn add_optimized(&mut self, n: i32) {
        todo!()
    }
    fn goto_unoptimized(&mut self, i: usize) {
        let c = match self.index.cmp(&i) {
            Ordering::Less => '>',
            Ordering::Greater => '<',
            Ordering::Equal => return,
        };
        let out: String =
            iter::repeat_n(c, (self.index as i32 - i as i32).abs() as usize).collect();
        self.index = i;
        self.out.push_str(&out);
    }
    fn goto_optimized(&mut self, i: usize) {
        todo!()
    }
}
