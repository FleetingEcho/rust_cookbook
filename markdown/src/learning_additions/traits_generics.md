# Trait 与泛型

## 简介

泛型让函数可以处理多种类型。trait 让不同类型共享同一组行为。

## 示例代码

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct StudyNote {
    pub title: String,
    pub topic: String,
}

impl Summary for StudyNote {
    fn summarize(&self) -> String {
        format!("{} - {}", self.title, self.topic)
    }
}

pub fn largest<T: Ord + Copy>(items: &[T]) -> Option<T> {
    let mut largest = *items.first()?;

    for &item in items.iter().skip(1) {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}

pub fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}
```

## 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_largest_item() {
        assert_eq!(largest(&[3, 1, 9, 2]), Some(9));
        assert_eq!(largest::<i32>(&[]), None);
    }

    #[test]
    fn summarizes_note() {
        let note = StudyNote {
            title: "所有权".to_string(),
            topic: "Rust 基础".to_string(),
        };

        assert_eq!(note.summarize(), "所有权 - Rust 基础");
    }
}
```

---

## 📘 TypeScript 对比

| Rust | TypeScript |
|------|-----------|
| `fn largest<T: Ord>(items: &[T])` | `function largest<T extends Comparable>(items: T[])` |
| `trait Summary { fn summarize(&self) -> String; }` | `interface Summary { summarize(): string; }` |
| `impl Trait for Type` | `class Type implements Interface` |
| `fn foo(item: &impl Summary)` | `function foo(item: Summary)` |

详细对照 → rust_vs_typescript.rs §9-10
