//! Examples of generated bounded integers and the API they produce.

use crate::bounded_integer;

bounded_integer! {
    /// A bounded struct, implemented as a protected newtype.
    ///
    /// This was generated from:
    /// ```text
    /// pub struct BoundedStruct { -8..8 }
    /// ```
    #[bounded_integer = crate]
    pub struct BoundedStruct { -8..8 }
}

bounded_integer! {
    /// A bounded enum.
    ///
    /// This was generated from:
    /// ```text
    /// pub enum BoundedEnum { -8..8 }
    /// ```
    #[bounded_integer = crate]
    pub enum BoundedEnum { -8..8 }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_range {
        ($fn:ident, $bounded:ident) => {
            #[test]
            fn $fn() {
                assert_eq!($bounded::MIN_VALUE, -8);
                assert_eq!($bounded::MAX_VALUE, 7);
                assert_eq!($bounded::MIN.get(), -8);
                assert_eq!($bounded::MAX.get(), 7);
                assert_eq!($bounded::RANGE, 16);

                assert!($bounded::in_range(0));
                assert!($bounded::in_range(-8));
                assert!(!$bounded::in_range(-9));
                assert!($bounded::in_range(7));
                assert!(!$bounded::in_range(8));
            }
        };
    }

    macro_rules! test_saturating {
        ($fn:ident, $bounded:ident) => {
            #[test]
            fn $fn() {
                assert_eq!($bounded::new_saturating(0).get(), 0);
                assert_eq!($bounded::new_saturating(-8).get(), -8);
                assert_eq!($bounded::new_saturating(-9).get(), -8);
                assert_eq!($bounded::new_saturating(i8::MIN).get(), -8);
                assert_eq!($bounded::new_saturating(7).get(), 7);
                assert_eq!($bounded::new_saturating(8).get(), 7);
                assert_eq!($bounded::new_saturating(i8::MAX).get(), 7);
            }
        };
    }

    macro_rules! test_wrapping {
        ($fn:ident, $bounded:ident) => {
            #[test]
            fn $fn() {
                assert_eq!($bounded::new_wrapping(0).get(), 0);
                assert_eq!($bounded::new_wrapping(-8).get(), -8);
                assert_eq!($bounded::new_wrapping(-9).get(), 7);
                assert_eq!($bounded::new_wrapping(7).get(), 7);
                assert_eq!($bounded::new_wrapping(8).get(), -8);
                assert_eq!($bounded::new_wrapping(-16).get(), 0);
                assert_eq!($bounded::new_wrapping(-18).get(), -2);
                assert_eq!($bounded::new_wrapping(-26).get(), 6);
                assert_eq!($bounded::new_wrapping(16).get(), 0);
                assert_eq!($bounded::new_wrapping(24).get(), -8);
            }
        };
    }

    macro_rules! test_arithmetic {
        ($fn:ident, $bounded:ident) => {
            #[test]
            fn $fn() {
                assert_eq!($bounded::new(7).unwrap().abs().get(), 7);
                assert_eq!($bounded::new(-7).unwrap().abs().get(), 7);
                assert_eq!($bounded::new(-2).unwrap().pow(3).get(), -8);
                assert_eq!($bounded::new(-5).unwrap().div_euclid(3).get(), -2);
                assert_eq!($bounded::new(-5).unwrap().rem_euclid(3).get(), 1);
                assert_eq!(($bounded::new(-5).unwrap() + 2).get(), -3);
                assert_eq!(($bounded::new(3).unwrap() - 7).get(), -4);
                assert_eq!(($bounded::new(-2).unwrap() * 3).get(), -6);
                assert_eq!(($bounded::new(7).unwrap() / 3).get(), 2);
                assert_eq!(($bounded::new(7).unwrap() % 3).get(), 1);
            }
        };
    }

    macro_rules! test_iter {
        ($fn:ident, $bounded:ident) => {
            #[test]
            fn $fn() {
                fn b(&n: &i8) -> $bounded {
                    $bounded::new(n).unwrap()
                }

                assert_eq!([3, 2, 1].iter().map(b).sum::<$bounded>().get(), 6);
                assert_eq!([-8, 3, 7, 5, -2].iter().map(b).sum::<$bounded>().get(), 5);
                assert_eq!([7, 6, 4].iter().map(b).sum::<i8>(), 17);
                assert_eq!([-8, 3, 7, 5, -2].iter().map(b).sum::<i8>(), 5);

                assert_eq!([1, 3, 2, 1].iter().map(b).product::<$bounded>().get(), 6);
                assert_eq!([1, 3, 2, 1, 0].iter().map(b).product::<$bounded>().get(), 0);
                assert_eq!([-2, -3, -1].iter().map(b).product::<$bounded>().get(), -6);
                assert_eq!([3, 3].iter().map(b).product::<i8>(), 9);
            }
        };
    }

    test_range!(test_struct_range, BoundedStruct);
    test_saturating!(test_struct_saturating, BoundedStruct);
    test_wrapping!(test_struct_wrapping, BoundedStruct);
    test_arithmetic!(test_struct_arithmetic, BoundedStruct);
    test_iter!(test_struct_iter, BoundedStruct);

    test_range!(test_enum_range, BoundedEnum);
    test_saturating!(test_enum_saturating, BoundedEnum);
    test_wrapping!(test_enum_wrapping, BoundedEnum);
    test_arithmetic!(test_enum_arithmetic, BoundedEnum);
    test_iter!(test_enum_iter, BoundedEnum);
}
