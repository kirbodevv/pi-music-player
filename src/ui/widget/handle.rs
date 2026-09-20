use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

struct HandleInner<T> {
    value: T,
    version: u64,
}

pub struct Handle<T> {
    inner: Rc<RefCell<HandleInner<T>>>,
}

impl<T: Default> Default for Handle<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}

impl<T> Handle<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Rc::new(RefCell::new(HandleInner { value, version: 0 })),
        }
    }

    pub fn get(&self) -> Ref<'_, T> {
        Ref::map(self.inner.borrow(), |inner| &inner.value)
    }

    pub fn modify(&self, f: impl FnOnce(&mut T)) {
        let mut inner = self.inner.borrow_mut();

        f(&mut inner.value);
        inner.version += 1;
    }

    pub fn version(&self) -> u64 {
        self.inner.borrow().version
    }
}
