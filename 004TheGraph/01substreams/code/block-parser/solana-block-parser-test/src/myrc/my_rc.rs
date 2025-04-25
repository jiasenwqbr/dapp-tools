use std::{ops::Deref, ptr::NonNull, sync::atomic::{AtomicUsize, Ordering}};

struct MyRcInner<T> {
    value:T,
    count:AtomicUsize,
} 

pub struct MyRc<T> {
    ptr:NonNull<MyRcInner<T>>,
}

impl<T> MyRc<T> {
    pub fn new(value: T) -> Self {
        let boxed = Box::new(MyRcInner {
            value,
            count: AtomicUsize::new(1),
        });

        MyRc {
            ptr: unsafe { NonNull::new_unchecked(Box::into_raw(boxed)) },
        }
    }

    pub fn strong_count(&self) -> usize {
        unsafe { self.ptr.as_ref().count.load(Ordering::SeqCst) }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        unsafe {
            self.ptr.as_ref().count.fetch_add(1, Ordering::SeqCst);
        }
        MyRc { ptr: self.ptr }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.ptr.as_ref().value }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        let count = unsafe {
            let inner = self.ptr.as_ref();
            inner.count.fetch_sub(1, Ordering::SeqCst)
        };

        if count == 1 {
            unsafe {
                // 最后一个引用了，释放内存
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}