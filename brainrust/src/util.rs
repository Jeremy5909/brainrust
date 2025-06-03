use crate::Program;

impl Program {
    pub(crate) fn debug_msg(&self, msg: &str) {
        if self.debug {
            eprintln!("{msg}");
        }
    }
    pub(crate) fn allocate(&mut self) -> usize {
        let mut i = 0;
        loop {
            if !self.used_indexes.contains(&i) {
                self.used_indexes.push(i);
                return i;
            }
            i += 1;
        }
    }
    pub(crate) fn allocate_arr(&mut self, len: usize) -> usize {
        let mut start = 0;
        loop {
            if (start..start + len).all(|i| !self.used_indexes.contains(&i)) {
                (start..start + len).for_each(|i| self.used_indexes.push(i));
                return start;
            }
            start += 1;
        }
    }
    pub(crate) fn deallocate(&mut self) {
        let i = self
            .used_indexes
            .iter()
            .position(|&x| x == self.index)
            .unwrap();
        self.used_indexes.remove(i);
    }
}
