// 🔑 要点：孤儿规则（Orphan Rule）
// 当实现 trait 时，trait 或类型**至少有一个**是在当前 crate 中定义的
// 不能为外部类型实现外部 trait
//
// 下面的代码会报错：PartialEq 和 u32 都是标准库的，不属于当前 crate
// 解决方案：删除这段代码，继续下一个练习

// impl PartialEq for u32 {
//     fn eq(&self, _other: &Self) -> bool {
//         todo!()
//     }
// }

fn main() {
    // 孤儿规则是 Rust 一致性系统的重要组成部分
    // 它确保：不会有两个 crate 同时为同一个类型实现同一个 trait
}
