use super::garbage_ref_cell::{GarbageRefCell, RefCellState};

pub struct GarbageRefMut<'garbage_ref_cell, T> {
    refcell: &'garbage_ref_cell GarbageRefCell<T>,
}

impl<'garbage_ref_cell, T> GarbageRefMut<'garbage_ref_cell, T> {
    pub fn new(refcell: &'garbage_ref_cell GarbageRefCell<T>) -> Self {
        GarbageRefMut { refcell }
    }
}

impl<'garbage_ref_cell, T> Drop for GarbageRefMut<'garbage_ref_cell, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefCellState::Shared(_) | RefCellState::Unshared => unreachable!(),
            RefCellState::Exclusive => {
                self.refcell.state.set(RefCellState::Unshared);
            }
        }
    }
}
