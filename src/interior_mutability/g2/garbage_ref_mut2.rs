use std::ops::{Deref, DerefMut};

use super::garbage_ref_cell2::{GRefCell2, GRefCell2State};

pub struct GRefMut2<'g2, T> {
    refcell: &'g2 GRefCell2<T>,
}

impl<'g2, T> GRefMut2<'g2, T> {
    pub fn new(refcell: &'g2 GRefCell2<T>) -> Self {
        GRefMut2 { refcell }
    }
}

impl<T> Drop for GRefMut2<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            GRefCell2State::Shared(_) | GRefCell2State::Unshared => unreachable!(),
            GRefCell2State::Exclusive => self.refcell.state.set(GRefCell2State::Unshared),
        }
    }
}

impl<T> Deref for GRefMut2<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> DerefMut for GRefMut2<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}
