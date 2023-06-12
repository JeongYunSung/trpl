//! # TPRL
//!
//! The Ruet Programming Language에 대한 설명이다.

//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

/// # This is library document
///
/// Simplify print "lib_print"
pub fn lib_print() {
    println!("lib_print")
}

/// lib_print_crate()
pub(crate) fn lib_print_crate() {
    println!("lib_print_crate")
}

/// 두 개의 숫자를 비교합니다.
///
/// # Examples
///
/// ```
/// use crate::trpl::add_one;
///
/// let arg = 5;
/// let answer = add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lib_print() {
        lib_print();
    }

    #[test]
    fn test_add_one() {
        assert_eq!(add_one(3), 4);
    }
}