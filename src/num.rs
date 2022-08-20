use std::ops::{Add, AddAssign, Mul, MulAssign, Rem};

pub trait Num:
    Add<Output = Self>
    + AddAssign
    + Mul<Output = Self>
    + MulAssign
    + Rem<Output = Self>
    + Sized
    + Copy
    + PartialOrd
{
    const ZERO: Self;
    const ONE: Self;
}

pub fn two<T: Num>() -> T {
    T::ONE + T::ONE
}
pub fn three<T: Num>() -> T {
    T::ONE + T::ONE + T::ONE
}

macro_rules! mk_num_impl {
    ($t:ident) => {
        impl Num for $t {
            const ZERO: $t = 0;
            const ONE: $t = 1;
        }
    };
}

mk_num_impl!(i32);
mk_num_impl!(i64);
mk_num_impl!(u64);
mk_num_impl!(usize);
