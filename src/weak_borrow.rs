use core::borrow::Borrow;

pub trait WeakBorrow<T: ?Sized> {
    fn weak_borrow(&self) -> &T;
}

impl<T: Borrow<U> + ?Sized, U: ?Sized> WeakBorrow<U> for T {
    fn weak_borrow(&self) -> &U {
        self.borrow()
    }
}
