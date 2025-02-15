use std::cell::UnsafeCell;

use super::{garbage_cell::GarbageCell, garbage_ref::GarbageRef};

#[derive(Clone, Copy)]
pub enum RefCellState {
    Unshared,
    Shared(i32),
    Exclusive,
}

pub struct GarbageRefCell<T> {
    pub(crate) value: UnsafeCell<T>,
    pub(crate) state: GarbageCell<RefCellState>,
}

struct GarbageRefMut<'garbage_ref_cell, T> {
    value: &'garbage_ref_cell mut T,
}

impl<T> GarbageRefCell<T> {
    pub fn new(value: T) -> Self {
        GarbageRefCell {
            value: UnsafeCell::new(value),
            state: GarbageCell::new(RefCellState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<GarbageRef<T>> {
        match self.state.get() {
            RefCellState::Unshared => {
                self.state.set(RefCellState::Shared(1));
                Some(GarbageRef::new(self))
            }
            RefCellState::Shared(x) => {
                self.state.set(RefCellState::Shared(x + 1));
                Some(GarbageRef::new(self))
            }
            RefCellState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<&mut T> {
        match self.state.get() {
            RefCellState::Unshared => {
                self.state.set(RefCellState::Exclusive);
                unsafe { Some(&mut *self.value.get()) }
            }
            RefCellState::Shared(_) | RefCellState::Exclusive => None,
        }
    }
}
