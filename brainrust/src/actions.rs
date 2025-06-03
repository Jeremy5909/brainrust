use std::{cmp::Ordering, iter};

use crate::{Optimization, Program};

impl Program {
    pub(crate) fn add(&mut self, n: i32) {
        match self.optimization {
            Optimization::None => self.add_unomptimised(n),
            Optimization::Speed => self.add_fast(n),
            Optimization::Size => self.add_small(n),
        }
    }
    pub(crate) fn goto(&mut self, i: usize) {
        match self.optimization {
            Optimization::None => self.goto_unomtimised(i),
            Optimization::Speed => self.goto_fast(i),
            Optimization::Size => self.goto_small(i),
        }
    }
    pub(crate) fn set_zero(&mut self) {
        self.out.push_str("[-]");
    }
    fn add_unomptimised(&mut self, n: i32) {
        let c = match n.signum() {
            1 => '+',
            -1 => '-',
            _ => return,
        };
        let out: String = iter::repeat_n(c, n.abs() as usize).collect();
        self.out.push_str(&out);
    }
    fn add_fast(&mut self, n: i32) {
        todo!()
    }
    fn add_small(&mut self, n: i32) {
        todo!()
    }
    fn goto_unomtimised(&mut self, i: usize) {
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
    fn goto_fast(&mut self, i: usize) {
        todo!()
    }
    fn goto_small(&mut self, i: usize) {
        todo!()
    }
}
