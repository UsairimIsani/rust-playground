use std::cell::RefCell;
#[derive(Debug)]
pub struct Pool<T> {
    items: RefCell<Vec<T>>,
}
pub trait PoolItem {
    fn new() -> Self;
}
impl<T: PoolItem> Pool<T> {
    pub fn new() -> Self {
        Self {
            items: RefCell::new(Vec::new()),
        }
    }
    pub fn get(&self) -> PoolGuard<T> {
        let item = match self.items.borrow_mut().pop() {
            Some(item) => item,
            None => T::new(),
        };
        PoolGuard {
            inner: Some(item),
            items: &self.items,
        }
    }
}
pub struct PoolGuard<'a, T> {
    inner: Option<T>,
    items: &'a RefCell<Vec<T>>,
}
impl<T> Drop for PoolGuard<'_, T> {
    fn drop(&mut self) {
        let item: T = self.inner.take().unwrap();
        self.items.borrow_mut().push(item);
    }
}
impl<T> std::ops::Deref for PoolGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner.as_ref().unwrap()
    }
}
impl<T> std::ops::DerefMut for PoolGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().unwrap()
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_pool() {
        use super::*;
        #[derive(Debug)]
        struct A(usize);
        impl A {
            fn do_thing(&self) {
                println!("{:#?}", self)
            }
            fn inc(&mut self) {
                self.0 += 1;
            }
            fn get(&self) -> usize {
                self.0
            }
        }
        impl PoolItem for A {
            fn new() -> Self {
                Self(0)
            }
        }
        let pool = Pool::new();
        {
            let mut item: PoolGuard<A> = pool.get();
            item.do_thing();
            item.inc();
        }
        {
            let item_2 = pool.get();
            item_2.do_thing();
            println!("{:?}", pool);
        }
    }
}
