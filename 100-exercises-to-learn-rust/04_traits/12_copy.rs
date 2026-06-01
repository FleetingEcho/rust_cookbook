// 🔑 要点：Copy trait 是 Clone 的子 trait
// Copy 类型在赋值时自动复制，而不是移动
// 基本类型（整数、布尔等）都实现了 Copy
// 自定义类型可以通过 #[derive(Copy, Clone)] 实现

// 为 WrappingU32 添加 Copy + Clone
// 注意：Copy 要求 Clone
#[derive(Clone, Copy, Debug)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

// 实现 Add 以支持 +
use std::ops::Add;

impl Add for WrappingU32 {
    type Output = WrappingU32;

    fn add(self, other: WrappingU32) -> WrappingU32 {
        WrappingU32 {
            value: self.value.wrapping_add(other.value),
        }
    }
}

// 实现 PartialEq 以支持 assert_eq!
impl PartialEq for WrappingU32 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
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
        // 因为实现了 Copy，x, y 在相加后仍然可用
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
