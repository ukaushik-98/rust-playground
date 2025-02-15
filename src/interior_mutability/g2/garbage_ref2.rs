use std::ops::Deref;

use super::garbage_ref_cell2::{GRefCell2, GRefCell2State};

pub struct GRef2<'g2, T> {
    refcell: &'g2 GRefCell2<T>,
}
impl<'g2, T> GRef2<'g2, T> {
    pub fn new(refcell: &'g2 GRefCell2<T>) -> Self {
        GRef2 { refcell }
    }
}

impl<'g2, T> Drop for GRef2<'g2, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            GRefCell2State::Exclusive | GRefCell2State::Unshared => unreachable!(),
            GRefCell2State::Shared(1) => {
                self.refcell.state.set(GRefCell2State::Unshared);
            }
            GRefCell2State::Shared(x) => {
                self.refcell.state.set(GRefCell2State::Shared(x - 1));
            }
        }
    }
}

impl<'g2, T> Deref for GRef2<'g2, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}
