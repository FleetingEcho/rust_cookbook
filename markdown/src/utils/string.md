# String 字符串工具

常用字符串处理函数，展示 Rust `String` / `&str` 的典型操作模式。

---

## 1. 大小写转换

```rust
pub fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

pub fn to_lowercase(s: &str) -> String {
    s.to_lowercase()
}

// 首字母大写（Rust 标准库没有内置，需要手写）
pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None    => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
    }
}
```

---

## 2. 去除空白

```rust
pub fn trim(s: &str) -> &str {
    s.trim()
}

pub fn trim_start(s: &str) -> &str {
    s.trim_start()
}

pub fn trim_end(s: &str) -> &str {
    s.trim_end()
}

// 去除指定字符
pub fn trim_char(s: &str, ch: char) -> &str {
    s.trim_matches(ch)
}
```

---

## 3. 查找与包含

```rust
pub fn contains_str(haystack: &str, needle: &str) -> bool {
    haystack.contains(needle)
}

pub fn starts_with(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
}

pub fn ends_with(s: &str, suffix: &str) -> bool {
    s.ends_with(suffix)
}

// 找到第一次出现的位置（字节偏移）
pub fn find_first(s: &str, pattern: &str) -> Option<usize> {
    s.find(pattern)
}

// 找到最后一次出现的位置
pub fn find_last(s: &str, pattern: &str) -> Option<usize> {
    s.rfind(pattern)
}
```

---

## 4. 分割与拼接

```rust
// 按分隔符分割，返回迭代器（惰性）
pub fn split_by(s: &str, delimiter: &str) -> Vec<&str> {
    s.split(delimiter).collect()
}

// 按空白符分割（自动处理多个连续空格）
pub fn split_words(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

// 只分割 N 次（取前 N+1 段）
pub fn split_n(s: &str, delimiter: &str, n: usize) -> Vec<&str> {
    s.splitn(n + 1, delimiter).collect()
}

// 用分隔符拼接字符串切片
pub fn join(parts: &[&str], separator: &str) -> String {
    parts.join(separator)
}

// 重复字符串
pub fn repeat(s: &str, n: usize) -> String {
    s.repeat(n)
}
```

---

## 5. 替换

```rust
pub fn replace_all(s: &str, from: &str, to: &str) -> String {
    s.replace(from, to)
}

// 只替换前 N 个
pub fn replace_n(s: &str, from: &str, to: &str, n: usize) -> String {
    s.replacen(from, to, n)
}
```

---

## 6. 数字解析

```rust
// 通用解析，任何实现了 FromStr 的类型
pub fn parse_int(s: &str) -> Result<i64, std::num::ParseIntError> {
    s.trim().parse()
}

pub fn parse_float(s: &str) -> Result<f64, std::num::ParseFloatError> {
    s.trim().parse()
}

// 有默认值的安全解析
pub fn parse_int_or(s: &str, default: i64) -> i64 {
    s.trim().parse().unwrap_or(default)
}
```

---

## 7. 格式化与填充

```rust
// 左对齐，宽度 width，用 pad_char 填充右边
pub fn pad_right(s: &str, width: usize, pad_char: char) -> String {
    format!("{s:<width$}", s = s, width = width)
        .replace(' ', &pad_char.to_string())
}

// 数字补零（最常见："{:0>5}"）
pub fn zero_pad(n: i64, width: usize) -> String {
    format!("{:0>width$}", n, width = width)
}

// 使用 format! 的常见格式
pub fn format_examples() {
    println!("{:>10}",   "right");    // 右对齐，宽度 10
    println!("{:<10}",   "left");     // 左对齐
    println!("{:^10}",   "center");   // 居中
    println!("{:0>5}",   42);         // 用 0 填充：00042
    println!("{:#010x}", 255);        // 十六进制：0x000000ff
    println!("{:.3}",    3.14159);    // 保留 3 位小数：3.142
    println!("{:e}",     1_000_000f64); // 科学计数：1e6
}
```

---

## 8. Unicode 与字符操作

```rust
// 字符数（人类可读长度）vs 字节数（内存占用）
pub fn char_count(s: &str) -> usize {
    s.chars().count()
}

pub fn byte_count(s: &str) -> usize {
    s.len()
}

// 遍历字符
pub fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}

pub fn char_at(s: &str, index: usize) -> Option<char> {
    s.chars().nth(index)
}

// 判断是否是纯 ASCII
pub fn is_ascii(s: &str) -> bool {
    s.is_ascii()
}

// 收集每个字符
pub fn chars_vec(s: &str) -> Vec<char> {
    s.chars().collect()
}

// 从字符 Vec 还原为 String
pub fn from_chars(chars: &[char]) -> String {
    chars.iter().collect()
}
```

---

## 9. 字符串构建

```rust
// 逐步构建字符串
pub fn build_csv(fields: &[&str]) -> String {
    let mut result = String::new();
    for (i, field) in fields.iter().enumerate() {
        if i > 0 { result.push(','); }
        result.push_str(field);
    }
    result
}

// 用迭代器 + collect 构建（更惯用）
pub fn build_lines(items: &[&str]) -> String {
    items.iter()
        .map(|s| format!("- {s}"))
        .collect::<Vec<_>>()
        .join("\n")
}
```

---

## 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(to_uppercase("hello"), "HELLO");
        assert_eq!(capitalize("hello world"), "Hello world");
    }

    #[test]
    fn test_trim() {
        assert_eq!(trim("  hello  "), "hello");
        assert_eq!(trim_char("***hi***", '*'), "hi");
    }

    #[test]
    fn test_find() {
        assert_eq!(find_first("hello world", "world"), Some(6));
        assert!(contains_str("hello", "ell"));
        assert!(starts_with("hello", "hel"));
        assert!(ends_with("hello", "llo"));
    }

    #[test]
    fn test_split_join() {
        let parts = split_by("a,b,c", ",");
        assert_eq!(parts, vec!["a", "b", "c"]);
        assert_eq!(join(&["a", "b", "c"], "-"), "a-b-c");
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse_int("  42  "), Ok(42));
        assert_eq!(parse_int_or("bad", -1), -1);
        assert!((parse_float("3.14").unwrap() - 3.14).abs() < 1e-10);
    }

    #[test]
    fn test_unicode() {
        let s = "你好世界";
        assert_eq!(char_count(s), 4);
        assert_eq!(byte_count(s), 12); // 每个汉字 3 字节
        assert_eq!(first_char(s), Some('你'));
    }

    #[test]
    fn test_zero_pad() {
        assert_eq!(zero_pad(42, 5), "00042");
    }
}
```
