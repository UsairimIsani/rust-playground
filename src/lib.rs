pub mod pool;
#[cfg(test)]
mod tests {
    #[test]
    fn test_pool() {
        use crate::pool::{Pool, PoolGuard, PoolItem};
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
            assert_eq!(1, item.get());
        }
        {
            let mut item_2 = pool.get();
            item_2.do_thing();
            item_2.inc();
            assert_eq!(2, item_2.get());
        }
        println!("{:?}", pool);
    }
}
