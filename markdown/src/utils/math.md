# Math 数学工具

常用数值计算函数，展示 Rust 标准库的数学操作。

---

## 1. 基础四则运算

```rust
pub fn add(a: i32, b: i32) -> i32 { a + b }
pub fn subtract(a: i32, b: i32) -> i32 { a - b }
pub fn multiply(a: i32, b: i32) -> i32 { a * b }

// 整数除法：返回 Option，被零除时返回 None 而非 panic
pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

// 取余
pub fn remainder(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a % b) }
}
```

---

## 2. 溢出安全运算

> **警告**：`+`/`-`/`*` 在 debug 模式下溢出会 panic，release 模式下会静默回绕。
> 需要确定行为时，用以下方法：

```rust
pub fn safe_add(a: i32, b: i32) -> Option<i32> {
    a.checked_add(b)      // 溢出时返回 None
}

pub fn saturating_add(a: i32, b: i32) -> i32 {
    a.saturating_add(b)   // 溢出时钳制到 i32::MAX 或 i32::MIN
}

pub fn wrapping_add(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)     // 溢出时回绕（明确表达这是预期行为）
}

// 返回值和是否溢出的标志
pub fn overflowing_add(a: i32, b: i32) -> (i32, bool) {
    a.overflowing_add(b)
}
```

```rust
// 使用示例
assert_eq!(safe_add(i32::MAX, 1), None);
assert_eq!(saturating_add(i32::MAX, 1), i32::MAX);
assert_eq!(wrapping_add(i32::MAX, 1), i32::MIN);
```

---

## 3. 幂运算

```rust
pub fn power_int(base: i32, exp: u32) -> i32 {
    base.pow(exp)
}

pub fn power_float(base: f64, exp: f64) -> f64 {
    base.powf(exp)
}

pub fn square_root(x: f64) -> Option<f64> {
    if x < 0.0 { None } else { Some(x.sqrt()) }
}

pub fn cube_root(x: f64) -> f64 {
    x.cbrt()
}
```

---

## 4. 绝对值 / 符号

```rust
pub fn absolute(x: i32) -> i32 {
    x.abs()
}

// checked_abs：i32::MIN.abs() 会溢出！
pub fn safe_abs(x: i32) -> Option<i32> {
    x.checked_abs()
}

pub fn signum(x: i32) -> i32 {
    x.signum()  // 负数 → -1，零 → 0，正数 → 1
}
```

---

## 5. 最值 / 钳制

```rust
pub fn clamp_value(x: i32, min: i32, max: i32) -> i32 {
    x.clamp(min, max)   // 等价于 x.max(min).min(max)
}

pub fn min_of(a: f64, b: f64) -> f64 {
    a.min(b)
}

pub fn max_of(a: f64, b: f64) -> f64 {
    a.max(b)
}

// 求一组数中的最小值（slice 版）
pub fn slice_min(values: &[i32]) -> Option<i32> {
    values.iter().copied().min()
}

pub fn slice_max(values: &[i32]) -> Option<i32> {
    values.iter().copied().max()
}
```

---

## 6. 浮点数工具

```rust
pub fn round_to(x: f64, decimals: u32) -> f64 {
    let factor = 10f64.powi(decimals as i32);
    (x * factor).round() / factor
}

pub fn is_close(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

pub fn log2(x: f64) -> f64   { x.log2() }
pub fn log10(x: f64) -> f64  { x.log10() }
pub fn ln(x: f64) -> f64     { x.ln() }

// 浮点常量
pub fn show_float_constants() {
    println!("π  = {}", std::f64::consts::PI);
    println!("e  = {}", std::f64::consts::E);
    println!("√2 = {}", std::f64::consts::SQRT_2);
    println!("NaN？{}", f64::NAN.is_nan());
    println!("Inf？{}", f64::INFINITY.is_infinite());
}
```

---

## 7. 统计工具

```rust
pub fn sum(values: &[i64]) -> i64 {
    values.iter().sum()
}

pub fn average(values: &[f64]) -> Option<f64> {
    if values.is_empty() { return None; }
    Some(values.iter().sum::<f64>() / values.len() as f64)
}

pub fn variance(values: &[f64]) -> Option<f64> {
    let avg = average(values)?;
    let v = values.iter().map(|x| (x - avg).powi(2)).sum::<f64>() / values.len() as f64;
    Some(v)
}

pub fn std_dev(values: &[f64]) -> Option<f64> {
    variance(values).map(f64::sqrt)
}
```

---

## 8. 整数工具

```rust
pub fn is_even(n: i32) -> bool { n % 2 == 0 }
pub fn is_odd(n: i32)  -> bool { n % 2 != 0 }

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let limit = (n as f64).sqrt() as u64;
    (3..=limit).step_by(2).all(|i| n % i != 0)
}

pub fn factorial(n: u64) -> Option<u64> {
    (1..=n).try_fold(1u64, |acc, i| acc.checked_mul(i))
}
```

---

## 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_ops() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(divide(10, 3), Some(3));
        assert_eq!(divide(10, 0), None);
    }

    #[test]
    fn test_overflow() {
        assert_eq!(safe_add(i32::MAX, 1), None);
        assert_eq!(saturating_add(i32::MAX, 1), i32::MAX);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp_value(150, 0, 100), 100);
        assert_eq!(clamp_value(-5, 0, 100), 0);
        assert_eq!(clamp_value(50, 0, 100), 50);
    }

    #[test]
    fn test_gcd_lcm() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(lcm(4, 6), 12);
    }

    #[test]
    fn test_prime() {
        assert!(is_prime(2));
        assert!(is_prime(17));
        assert!(!is_prime(1));
        assert!(!is_prime(15));
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), Some(120));
        assert_eq!(factorial(0), Some(1));
    }

    #[test]
    fn test_statistics() {
        let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        assert!((average(&data).unwrap() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_float_close() {
        assert!(is_close(0.1 + 0.2, 0.3, 1e-10));
    }
}
```
