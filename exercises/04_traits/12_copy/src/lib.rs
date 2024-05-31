// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.

// 生成了 Copy 和 Clone trait 的实现
// PartialEq 对应assert_eq需要的 == 实现
// Add 对应 + 实现
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

use std::ops::Add;
impl Add for WrappingU32 {
    type Output = Self;

    // 设置 Copy ，不会丢失所有权
    fn add(self, other : Self) -> Self {
        WrappingU32::new(self.value.wrapping_add(other.value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
