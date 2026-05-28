# Helper 工具函数

跨模块通用 helper，展示常见的 Rust 辅助函数模式。

---

## 1. 引用跨模块常量

```rust
use crate::config::constants::MAX_POINTS;

pub fn print_max_points() {
    println!("Max points: {}", MAX_POINTS);
}
```

通过 `use` 从 `config::constants` 模块引入 `MAX_POINTS` 常量，在 `helper` 模块中使用。

---

## 2. 类型转换 Helper

```rust
// 安全地将 usize 转为 i32（用于下标转偏移量等场景）
pub fn usize_to_i32(n: usize) -> Option<i32> {
    i32::try_from(n).ok()
}

// 将 bool 转为 0/1（某些 API 或序列化需要）
pub fn bool_to_int(b: bool) -> u8 {
    b as u8
}

// 将字符串 "true"/"false"/"1"/"0" 解析为 bool
pub fn parse_bool(s: &str) -> Option<bool> {
    match s.trim().to_lowercase().as_str() {
        "true"  | "1" | "yes" | "on"  => Some(true),
        "false" | "0" | "no"  | "off" => Some(false),
        _ => None,
    }
}
```

---

## 3. 验证 Helper

```rust
// 非空检查：空字符串或纯空白都视为"空"
pub fn is_blank(s: &str) -> bool {
    s.trim().is_empty()
}

pub fn require_non_blank(s: &str, field: &str) -> Result<(), String> {
    if is_blank(s) {
        Err(format!("字段 '{field}' 不能为空"))
    } else {
        Ok(())
    }
}

// 范围检查
pub fn in_range(value: i64, min: i64, max: i64) -> bool {
    (min..=max).contains(&value)
}

pub fn require_in_range(value: i64, min: i64, max: i64, name: &str) -> Result<(), String> {
    if in_range(value, min, max) {
        Ok(())
    } else {
        Err(format!("{name} 必须在 [{min}, {max}] 范围内，实际值: {value}"))
    }
}
```

---

## 4. Option / Result 辅助

```rust
// 将 Option<T> 转为 Result<T, E>，附带自定义错误信息
pub fn require<T>(opt: Option<T>, msg: &str) -> Result<T, String> {
    opt.ok_or_else(|| msg.to_string())
}

// 对 Result 中的 Err 添加上下文前缀
pub fn add_context<T>(result: Result<T, String>, context: &str) -> Result<T, String> {
    result.map_err(|e| format!("{context}: {e}"))
}

// 静默忽略错误（只在确实不关心结果时使用）
pub fn ignore<T, E>(result: Result<T, E>) {
    let _ = result;
}
```

---

## 5. 集合 Helper

```rust
// 对切片去重，保持顺序（标准库没有内置）
pub fn dedup_stable<T: PartialEq + Clone>(items: &[T]) -> Vec<T> {
    let mut seen = Vec::new();
    for item in items {
        if !seen.contains(item) {
            seen.push(item.clone());
        }
    }
    seen
}

// 将 Vec 分成每组 size 个的子 Vec
pub fn chunks_owned<T: Clone>(items: Vec<T>, size: usize) -> Vec<Vec<T>> {
    items.chunks(size).map(|c| c.to_vec()).collect()
}

// 判断一个切片是否包含所有给定元素
pub fn contains_all<T: PartialEq>(haystack: &[T], needles: &[T]) -> bool {
    needles.iter().all(|n| haystack.contains(n))
}
```

---

## 6. 时间 / 计时 Helper

```rust
use std::time::{Duration, Instant};

// 测量一段代码的执行时间
pub fn measure<F, T>(label: &str, f: F) -> T
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    println!("{label} 耗时: {:?}", start.elapsed());
    result
}

// 将毫秒数格式化为人类可读字符串
pub fn format_duration(ms: u64) -> String {
    let d = Duration::from_millis(ms);
    let secs = d.as_secs();
    if secs >= 3600 {
        format!("{}h {}m {}s", secs / 3600, (secs % 3600) / 60, secs % 60)
    } else if secs >= 60 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else if secs > 0 {
        format!("{}.{:03}s", secs, d.subsec_millis())
    } else {
        format!("{}ms", d.subsec_millis())
    }
}
```

---

## 7. 调试 Helper

```rust
// 打印值并返回它（用于调试调用链中途的值）
// 等同于 dbg! 宏，但可以自定义标签
pub fn trace<T: std::fmt::Debug>(label: &str, value: T) -> T {
    println!("[TRACE] {label} = {value:?}");
    value
}

// 断言两个浮点数近似相等（避免浮点精度问题）
pub fn assert_float_eq(a: f64, b: f64, epsilon: f64) {
    assert!(
        (a - b).abs() < epsilon,
        "浮点数不相等: {a} != {b} (epsilon={epsilon})"
    );
}
```

---

## 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bool() {
        assert_eq!(parse_bool("true"),  Some(true));
        assert_eq!(parse_bool("False"), Some(false));
        assert_eq!(parse_bool("1"),     Some(true));
        assert_eq!(parse_bool("off"),   Some(false));
        assert_eq!(parse_bool("maybe"), None);
    }

    #[test]
    fn test_blank() {
        assert!(is_blank(""));
        assert!(is_blank("   "));
        assert!(!is_blank("hi"));
    }

    #[test]
    fn test_require_non_blank() {
        assert!(require_non_blank("hello", "name").is_ok());
        assert!(require_non_blank("  ", "name").is_err());
    }

    #[test]
    fn test_dedup_stable() {
        let v = vec![1, 2, 2, 3, 1, 4];
        assert_eq!(dedup_stable(&v), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_chunks() {
        let v = vec![1, 2, 3, 4, 5];
        let chunks = chunks_owned(v, 2);
        assert_eq!(chunks, vec![vec![1, 2], vec![3, 4], vec![5]]);
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(500), "500ms");
        assert_eq!(format_duration(90_000), "1m 30s");
    }

    #[test]
    fn test_measure() {
        let result = measure("测试", || 1 + 1);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_require() {
        assert_eq!(require(Some(42), "缺少值"), Ok(42));
        assert!(require::<i32>(None, "缺少值").is_err());
    }
}
```
