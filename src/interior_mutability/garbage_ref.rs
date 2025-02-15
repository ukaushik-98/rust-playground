use std::ops::Deref;

use super::garbage_ref_cell::{GarbageRefCell, RefCellState};

pub struct GarbageRef<'garbage_ref_cell, T> {
    refcell: &'garbage_ref_cell GarbageRefCell<T>,
}

impl<'garbage_ref_cell, T> GarbageRef<'garbage_ref_cell, T> {
    pub fn new(refcell: &'garbage_ref_cell GarbageRefCell<T>) -> GarbageRef<'garbage_ref_cell, T> {
        GarbageRef { refcell }
    }
}

impl<'garbage_ref_cell, T> Drop for GarbageRef<'garbage_ref_cell, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefCellState::Exclusive | RefCellState::Unshared => unreachable!(),
            RefCellState::Shared(1) => {
                self.refcell.state.set(RefCellState::Unshared);
            }
            RefCellState::Shared(x) => {
                self.refcell.state.set(RefCellState::Shared(x - 1));
            }
        }
    }
}

impl<'garbage_ref_cell, T> Deref for GarbageRef<'garbage_ref_cell, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}
