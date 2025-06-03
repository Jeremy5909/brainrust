use crate::Program;

impl Program {
    pub(crate) fn debug_msg(&self, msg: &str) {
        if self.debug {
            eprintln!("{msg}");
        }
    }
    pub(crate) fn get_unused_index(&mut self) -> usize {
        let mut i = 0;
        loop {
            if !self.used_indexes.contains(&(i)) {
                self.used_indexes.push(i);
                return i;
            } else {
                i += 1;
            }
        }
    }
    pub(crate) fn unuse_index(&mut self) {
        let i = self
            .used_indexes
            .iter()
            .position(|&x| x == self.index)
            .unwrap();
        self.used_indexes.remove(i);
    }
}
