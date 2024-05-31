// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

// 调用 a+b 之后，依旧拥有所有权，需要derive Clone + Copy 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SaturatingU16 {
    value : u16,
}

impl From<u16> for SaturatingU16{
    fn from(s: u16) -> SaturatingU16 {
        SaturatingU16 {
            value: s
        }
    }
}

impl From<u8> for SaturatingU16{
    fn from(s: u8) -> SaturatingU16 {
        SaturatingU16 {
            value: s as u16
        }
    }
}

impl From<&u16> for SaturatingU16{
    fn from(s: &u16) -> SaturatingU16 {
        SaturatingU16 {
            value: *s
        }
    }
}

impl From<&u8> for SaturatingU16{
    fn from(s: &u8) -> SaturatingU16 {
        SaturatingU16 {
            value: *s as u16
        }
    }
}

// Add
use std::ops::Add;
impl Add<SaturatingU16> for SaturatingU16 {
    type Output = Self;

    // 设置 Copy ，不会丢失所有权
    fn add(self, other : Self) -> Self {
        SaturatingU16 {
            value: self.value.saturating_add(other.value),
        }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    // 设置 Copy ，不会丢失所有权
    fn add(self, other : u16) -> Self {
        SaturatingU16 {
            value: self.value.saturating_add(other),
        }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    // 设置 Copy ，不会丢失所有权
    fn add(self, other : &u16) -> Self {
        SaturatingU16 {
            value: self.value.saturating_add(*other),
        }
    }
}

// PartialEq 需要自定义实现,当derive指定时，可以省略结构对应类型的PartialEq实现
/*
impl PartialEq<SaturatingU16> for SaturatingU16{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
*/

// 当你为类型 T 实现 PartialEq<U> 时，eq 方法的签名应为 fn eq(&self, other: &U) -> bool
impl PartialEq<u16> for SaturatingU16{
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}
