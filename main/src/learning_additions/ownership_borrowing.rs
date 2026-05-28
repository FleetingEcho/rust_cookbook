// 所有权、借用和切片是 Rust 最核心的学习点。
// 这组例子重点演示：谁拥有数据、谁只是临时使用数据、如何避免悬垂引用。

pub fn ownership_move_and_clone() {
    let name = String::from("Rust");

    // String 不是 Copy 类型，下面这一行会把所有权移动给 moved_name。
    let moved_name = name;
    println!("移动后的值: {moved_name}");

    // 如果还想保留原值，就显式 clone。clone 会复制堆上的数据，成本比移动更高。
    let language = String::from("Rust");
    let cloned_language = language.clone();
    println!("原值: {language}, 克隆值: {cloned_language}");
}

pub fn borrow_without_taking_ownership() {
    let text = String::from("hello rust");
    let len = calculate_length(&text);

    // calculate_length 只借用了 text，所以这里还能继续使用 text。
    println!("'{text}' 的长度是 {len}");
}

fn calculate_length(value: &str) -> usize {
    value.len()
}

pub fn mutable_borrow_rule() {
    let mut message = String::from("hello");

    // 同一时间只能有一个可变引用。这样可以避免数据竞争。
    let borrowed = &mut message;
    borrowed.push_str(", rust");

    // borrowed 最后一次使用后，借用结束，message 又可以被使用。
    println!("{message}");
}

pub fn first_word_slice(value: &str) -> &str {
    for (index, byte) in value.bytes().enumerate() {
        if byte == b' ' {
            return &value[..index];
        }
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_word_handles_sentence() {
        assert_eq!(first_word_slice("hello rust"), "hello");
        assert_eq!(first_word_slice("rust"), "rust");
    }
}

// 📘 TypeScript 对比
// ====================
// 所有权是 Rust 独有的——TS 中所有对象通过 GC 管理。
//
// | Rust | TS |
// |------|-----|
// | `let b = a.clone()` 深拷贝 | `let b = JSON.parse(JSON.stringify(a))` |
// | `fn foo(&s)` 借用（不获取所有权） | 默认就是引用传递 |
// | `fn foo(s: String)` 获取所有权 | 传参不需要考虑所有权 |
// | `let s2 = s` 移动所有权 | `let s2 = s` 引用拷贝 |
//
// 详细对照 → rust_vs_typescript.rs §7
