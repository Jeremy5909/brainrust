use crate::{Error, Program};

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
    pub(crate) fn allocate_str(&mut self, len: usize) -> usize {
        let mut start = 0;
        // + 1 for terminator
        let len = len + 1;
        loop {
            if (start..start + len).all(|i| !self.used_indexes.contains(&i)) {
                (start..start + len).for_each(|i| self.used_indexes.push(i));
                return start;
            }
            start += 1;
        }
    }
    pub(crate) fn deallocate(&mut self, index: usize) {
        let i = self.used_indexes.iter().position(|&x| x == index).unwrap();
        self.used_indexes.remove(i);
    }
    pub(crate) fn get_var(&mut self, name: &str) -> Result<usize, Error> {
        self.vars
            .get(name)
            .copied()
            .ok_or(Error::VariableNotFound(name.to_owned()))
    }
}
