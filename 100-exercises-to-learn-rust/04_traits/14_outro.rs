// 🔑 要点：综合练习——实现 SaturatingU16 类型
// 需要实现：
// - From 多种类型的转换
// - Add 加法（饱和到 u16::MAX）
// - PartialEq 比较
// - Debug 打印
use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SaturatingU16 {
    value: u16,
}

// From<u16>
impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 { value }
    }
}

// From<u8>
impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16 { value: value as u16 }
    }
}

// From<&u16>
impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16 { value: *value }
    }
}

// From<&u8>
impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16 { value: *value as u16 }
    }
}

// Add<SaturatingU16>
impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: SaturatingU16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(other.value),
        }
    }
}

// Add<u16>
impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: u16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(other),
        }
    }
}

// Add<&u16>
impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: &u16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(*other),
        }
    }
}

// Add<&SaturatingU16>
impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: &SaturatingU16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(other.value),
        }
    }
}

// PartialEq<u16>
impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}

#[cfg(test)]
mod integration_tests {
    use super::SaturatingU16;

    #[test]
    fn test_saturating_u16() {
        let a: SaturatingU16 = (&10u8).into();
        let b: SaturatingU16 = 5u8.into();
        let c: SaturatingU16 = u16::MAX.into();
        let d: SaturatingU16 = (&1u16).into();
        let e = &c;

        assert_eq!(a + b, SaturatingU16::from(15u16));
        assert_eq!(a + c, SaturatingU16::from(u16::MAX));
        assert_eq!(a + d, SaturatingU16::from(11u16));
        assert_eq!(a + a, 20u16);
        assert_eq!(a + 5u16, 15u16);
        assert_eq!(a + e, SaturatingU16::from(u16::MAX));
    }
}
