# 生命周期

## 简介

生命周期告诉编译器：返回的引用和哪个输入引用活得一样久。它不改变值的实际存活时间，只是把借用关系写清楚。

## 示例代码

```rust
pub fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

pub struct Excerpt<'a> {
    // 结构体里保存引用时，必须写生命周期参数。
    pub part: &'a str,
}

impl<'a> Excerpt<'a> {
    pub fn level(&self) -> i32 {
        1
    }

    // 这里用了生命周期省略规则：返回值默认和 &self 的生命周期相关。
    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("通知: {announcement}");
        self.part
    }
}

pub fn first_sentence(text: &str) -> Excerpt<'_> {
    let end = text.find('.').unwrap_or(text.len());
    Excerpt { part: &text[..end] }
}
```

## 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chooses_longest_str() {
        assert_eq!(longest("rust", "go"), "rust");
    }

    #[test]
    fn excerpt_borrows_from_text() {
        let text = "Rust 很严格. 但这种严格能换来安全。";
        let excerpt = first_sentence(text);
        assert_eq!(excerpt.part, "Rust 很严格");
    }
}
```

---

## 📘 TypeScript 对比

生命周期——TS 中不存在！

最容易的理解方式：生命周期 `'a` ≈ 编译器用来标记“这个引用至少要活多久”的标签。

**Rust：**

```rust
fn first<'a>(x: &'a str, y: &'a str) -> &'a str { x }
// 读作：x 和 y 至少活 'a 这么长，返回的引用也活 'a 这么长
```

TS 完全不需要这个——GC 自动管理对象生命周期。Rust 在编译期做这件事，零运行时开销。

详细对照 → rust_vs_typescript.rs §8
