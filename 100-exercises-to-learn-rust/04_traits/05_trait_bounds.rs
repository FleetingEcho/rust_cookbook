// 🔑 要点：trait bound 约束泛型类型必须实现某个 trait
// 这里 min 函数需要比较两个值，所以 T 必须实现 PartialOrd
// PartialOrd 是 <, <=, >, >= 等比较运算符对应的 trait

// 添加 trait bound：T 必须可比较
pub fn min<T: std::cmp::PartialOrd>(left: T, right: T) -> T {
    if left <= right {
        left
    } else {
        right
    }
}

// 💡 等价写法：pub fn min<T>(left: T, right: T) -> T where T: PartialOrd { ... }
