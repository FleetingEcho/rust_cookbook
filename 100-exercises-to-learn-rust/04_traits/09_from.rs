// 🔑 要点：From trait 用于类型转换（消耗式转换）
// 实现了 From<T> 就自动获得了 Into<T>
// 42.into() 会调用 From<u32> for WrappingU32 的实现

#[allow(dead_code)]
pub struct WrappingU32 {
    value: u32,
}

// 实现 From<u32> for WrappingU32
impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { value }
    }
}

#[allow(dead_code)]
fn example() {
    // 通过 Into trait 转换：42.into()
    let _wrapping: WrappingU32 = 42.into();
    // 通过 From trait 转换：WrappingU32::from(42)
    let _wrapping = WrappingU32::from(42);
}
