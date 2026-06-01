#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: This function shouldn't always return an `Ok`.
        // Read the tests below to clarify what should be returned.
        
        // match value {
        //     0=>Err(CreationError::Zero),
        //     1..=100=> Ok(Self(value as u64)),
        //     _=>Err(CreationError::Negative),
        // }
        // match value {
        //     value if value > 0 => Ok(Self(value as u64)),
        //     0 => Err(CreationError::Zero),
        //     _ => Err(CreationError::Negative),  // 所有负数
        // }

        
        // 方法	类型	失败时	使用场景
        // as	强制转换	静默溢出/截断	确定安全时
        // into()	自动转换	编译错误	总是成功的转换
        // try_into()	尝试转换	返回 Result	可能失败的转换

        if value > 0 {
            match value.try_into() {
                Ok(v) => Ok(Self(v)),
                Err(_) => Err(CreationError::Negative), // 理论上不会发生
            }
        } else if value == 0 {
            Err(CreationError::Zero)
        } else {
            Err(CreationError::Negative)
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
