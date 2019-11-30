use std::sync::{
    atomic::{fence, AtomicUsize, Ordering},
    Arc,
};

pub(crate) struct ArcSlice<T> {
    data: *mut [T],
    count: Arc<AtomicUsize>,
}

impl<T> ArcSlice<T> {
    pub fn new(slice: Box<[T]>) -> ArcSlice<T> {
        let data = Box::into_raw(slice);

        ArcSlice {
            data,
            count: Arc::new(AtomicUsize::new(1)),
        }
    }
}

unsafe impl<T> Send for ArcSlice<T> {}
unsafe impl<T> Sync for ArcSlice<T> {}

impl<T> Clone for ArcSlice<T> {
    fn clone(&self) -> Self {
        self.count.fetch_add(1, Ordering::Relaxed);

        ArcSlice {
            data: self.data.clone(),
            count: self.count.clone(),
        }
    }
}

impl<T> AsRef<[T]> for ArcSlice<T> {
    fn as_ref(&self) -> &[T] {
        unsafe { &*self.data }
    }
}

impl<T> std::ops::Deref for ArcSlice<T> {
    type Target = [T];

    fn deref<'a>(&'a self) -> &'a [T] {
        unsafe { &*self.data }
    }
}

impl<T> Drop for ArcSlice<T> {
    fn drop(&mut self) {
        if self.count.fetch_sub(1, Ordering::Release) != 1 {
            return;
        }

        fence(Ordering::Acquire);

        unsafe {
            std::mem::drop(Box::from_raw(self.data));
        }
    }
}
