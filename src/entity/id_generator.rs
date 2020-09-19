use std::hash::Hash;

pub trait IDType: Clone + Copy + Hash + Eq {
    fn next(&mut self) -> Self;
    fn new() -> Self;
}

#[macro_export]
macro_rules! new_id_type {
    ($kind:ty) => {
        impl IDType for $kind {
            fn next(&mut self) -> Self {
                *self += 1 as $kind;
                *self
            }

            fn new() -> Self {
                0 as $kind
            }
        }
    };
}

new_id_type!(u16);
new_id_type!(u32);
new_id_type!(u64);
new_id_type!(usize);

pub struct IDGenerator<T: IDType = u64> {
     incrementer: T,
}

impl<T: IDType> IDGenerator<T> {
    pub fn new() -> Self {
        IDGenerator{incrementer: T::new()}
    }

    pub fn gen(&mut self) -> T {
        self.incrementer.next()
    }
}

