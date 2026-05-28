# const fn 与 const 泛型

## 简介

const fn 在编译期执行，结果可以用来初始化常量/静态变量。

const 泛型让类型可以被常量值参数化，最典型的就是数组长度。

这两个特性在 Rust 1.56+ 稳定，const 泛型在 1.51+ 稳定。

## const fn

const fn 可以在编译期被调用，也可以在运行期正常调用。

```rust
pub const fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// 在编译期直接求值，不会有任何运行时开销。
pub const FACT_10: u64 = factorial(10);

pub fn show_const_fn() {
    println!("10! = {FACT_10}"); // 值在编译期已确定

    // const fn 也可以在运行时调用，行为完全一样。
    let n = 5u64;
    println!("{n}! = {}", factorial(n));
}
```

## const 泛型：数组长度作为类型参数

在 const 泛型出现之前，`[T; 1]`、`[T; 2]`、`[T; 32]` 是完全不同的类型，无法写一个统一的函数处理所有长度。现在可以用 `const N: usize` 统一。

```rust
pub fn array_sum<const N: usize>(arr: [i32; N]) -> i32 {
    arr.iter().sum()
}

pub fn show_const_generics() {
    // 同一个函数处理不同长度的数组，N 在编译期由具体调用决定。
    println!("sum [1,2,3]     = {}", array_sum([1, 2, 3]));
    println!("sum [10,20]     = {}", array_sum([10, 20]));
    println!("sum [1,2,3,4,5] = {}", array_sum([1, 2, 3, 4, 5]));
}
```

## 带 const 泛型的结构体

固定大小的环形缓冲区，容量是类型的一部分，不需要堆分配。

```rust
pub struct RingBuffer<T, const CAP: usize> {
    data: [Option<T>; CAP],
    head: usize,
    len: usize,
}

impl<T: Copy + Default, const CAP: usize> RingBuffer<T, CAP> {
    pub fn new() -> Self {
        RingBuffer {
            data: [None; CAP],
            head: 0,
            len: 0,
        }
    }

    // 推入一个值；超出容量时覆盖最旧的元素。
    pub fn push(&mut self, value: T) {
        let idx = (self.head + self.len) % CAP;
        if self.len < CAP {
            self.data[idx] = Some(value);
            self.len += 1;
        } else {
            // 已满，覆盖最旧的槽，同时移动 head。
            self.data[self.head] = Some(value);
            self.head = (self.head + 1) % CAP;
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

pub fn show_ring_buffer() {
    // 容量 3 是类型的一部分：RingBuffer<i32, 3>
    let mut buf: RingBuffer<i32, 3> = RingBuffer::new();
    buf.push(1);
    buf.push(2);
    buf.push(3);
    println!("满了，长度: {}", buf.len()); // 3

    buf.push(4); // 覆盖最旧的 1
    println!("覆盖后长度: {}", buf.len()); // 仍是 3
}
```

## 用 const 泛型做编译期断言

下面这个函数只接受非空数组，长度为 0 时直接编译报错。

```rust
pub fn first_element<T, const N: usize>(arr: &[T; N]) -> &T
where
    // N > 0 的约束由调用方在编译期保证；如果传 [T; 0] 则编译失败。
    [T; N]: Sized,
{
    &arr[0]
}
```

## 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_compile_time() {
        // FACT_10 是编译期常量
        assert_eq!(FACT_10, 3_628_800);
    }

    #[test]
    fn array_sum_various_lengths() {
        assert_eq!(array_sum([1, 2, 3]), 6);
        assert_eq!(array_sum([10, 20]), 30);
        assert_eq!(array_sum::<0>([]), 0);
    }

    #[test]
    fn ring_buffer_capacity() {
        let mut buf: RingBuffer<i32, 2> = RingBuffer::new();
        buf.push(1);
        buf.push(2);
        assert_eq!(buf.len(), 2);
        buf.push(3); // 覆盖，长度不变
        assert_eq!(buf.len(), 2);
    }
}
```
