use std::{cmp::Ordering, iter};

use crate::Program;

impl Program {
    pub(crate) fn add(&mut self, n: i32) {
        let c = match n.signum() {
            1 => '+',
            -1 => '-',
            _ => return,
        };
        let out: String = iter::repeat_n(c, n.abs() as usize).collect();
        self.out.push_str(&out);
    }
    pub(crate) fn goto(&mut self, i: usize) {
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
}
