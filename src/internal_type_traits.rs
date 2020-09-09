use std::{
    fmt,
    iter::{Product, Sum},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
};

// Skipped:
//
// - `is_signed_int_t<T>`   (probably won't be used directly in `modint.rs`)
// - `is_unsigned_int_t<T>` (probably won't be used directly in `modint.rs`)
// - `to_unsigned_t<T>`     (not used in `fenwicktree.rs`)

// We will remove unnecessary bounds later.

/// Corresponds to `std::is_integral` in C++.
pub(crate) trait Integral:
    'static
    + Send
    + Sync
    + Copy
    + Ord
    + Not<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + Sum
    + Product
    + BitOr<Output = Self>
    + BitAnd<Output = Self>
    + BitXor<Output = Self>
    + BitOrAssign
    + BitAndAssign
    + BitXorAssign
    + Shl<Output = Self>
    + Shr<Output = Self>
    + ShlAssign
    + ShrAssign
    + fmt::Display
    + fmt::Debug
    + fmt::Binary
    + fmt::Octal
{
    fn zero() -> Self;
    fn one() -> Self;
}

macro_rules! impl_integral {
    ($($ty:ty),*) => {
        $(
            impl Integral for $ty {
                #[inline]
                fn zero() -> Self {
                    0
                }

                #[inline]
                fn one() -> Self {
                    1
                }
            }
        )*
    };
}

impl_integral!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
