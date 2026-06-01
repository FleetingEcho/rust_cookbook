// 🔑 要点：关联类型 vs 泛型参数
// 关联类型：对于每个 trait 实现，类型是固定的
// 泛型参数：同一个类型可以有多个不同的实现

// 定义一个 Power trait，使用泛型参数来指定指数类型
trait Power<Exponent> {
    fn power(&self, exponent: Exponent) -> u32;
}

// 为 u32 实现 Power<u16>
impl Power<u16> for u32 {
    fn power(&self, exponent: u16) -> u32 {
        self.pow(exponent as u32)
    }
}

// 为 u32 实现 Power<u32>
impl Power<u32> for u32 {
    fn power(&self, exponent: u32) -> u32 {
        self.pow(exponent)
    }
}

// 为 u32 实现 Power<&u32>
impl Power<&u32> for u32 {
    fn power(&self, exponent: &u32) -> u32 {
        self.pow(*exponent)
    }
}

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
