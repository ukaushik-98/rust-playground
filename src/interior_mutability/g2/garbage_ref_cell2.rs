use std::cell::UnsafeCell;

use super::{garbage_cell2::GCell2, garbage_ref2::GRef2};

#[derive(Clone, Copy)]
pub enum GRefCell2State {
    Unshared,
    Shared(i32),
    Exclusive,
}

pub struct GRefCell2<T> {
    pub(crate) state: GCell2<GRefCell2State>,
    pub(crate) value: UnsafeCell<T>,
}

impl<T> GRefCell2<T> {
    pub fn new(value: T) -> Self {
        GRefCell2 {
            state: GCell2::new(GRefCell2State::Unshared),
            value: UnsafeCell::new(value),
        }
    }

    pub fn borrow(&self) -> Option<GRef2<T>> {
        match self.state.get() {
            GRefCell2State::Exclusive => None,
            GRefCell2State::Unshared => {
                self.state.set(GRefCell2State::Shared(0));
                Some(GRef2::new(&self))
            }
            GRefCell2State::Shared(x) => {
                self.state.set(GRefCell2State::Shared(x + 1));
                Some(GRef2::new(&self))
            }
        }
    }

    pub fn borrow_mut(&self) -> Option<&mut T> {
        match self.state.get() {
            GRefCell2State::Unshared => {
                self.state.set(GRefCell2State::Exclusive);
                unsafe { Some(&mut *self.value.get()) }
            }
            GRefCell2State::Exclusive | GRefCell2State::Shared(_) => None,
        }
    }
}
