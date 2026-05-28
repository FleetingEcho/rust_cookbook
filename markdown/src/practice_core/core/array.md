# 数组练习

## 基础操作

```rust
pub fn array_practice() {
    let mut numbers: [i32; 10] = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    println!(
        "initial numbers, {:?},length is {:?}, the number at index 2 is::{:?}",
        numbers,
        numbers.len(),
        numbers[2]
    );

    let slice = &numbers[1..4];
    println!("slice, {:?}", slice);

    let repeated_numbers = [1; 5];
    println!("repeated numbers, {:?}", repeated_numbers);

    println!("{:?}", numbers);

    for num in numbers[0..2].iter() {
        println!("iter number is {:?}", num);
    }

    for num in numbers.iter_mut() {
        *num *= 2;
    }

    let squared: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("squared numbers, {:?}", squared);

    let even_numbers: Vec<i32> = numbers.iter().copied().filter(|x| x % 3 == 0).collect();
    let even_numbers1: Vec<&i32> = numbers.iter().filter(|x| *x % 3 == 0).collect();
    println!(
        "even numbers, {:?}, address is {:?}",
        even_numbers, even_numbers1
    );

    let result: Vec<i32> = numbers
        .iter()
        .copied()
        .filter(|x| x % 3 == 0)
        .map(|x| x * 10)
        .collect();

    println!("{:?}", result);
}
```

## 迭代器说明

- `iter()` 返回不可变引用 `&T`，不会消耗数组。
- `iter_mut()` 返回可变引用 `&mut T`，可以修改值。
- `map` 作用于每个元素，默认保留引用 `&T`。
- `filter` 需要 `&T` 进行比较，接受 `Fn(&T) -> bool`。
- 用 `.copied()` 可以避免 `*x`，让迭代器直接返回值。

> `even_numbers` 拥有这些值，可以修改它们。`even_numbers1` 不拥有数据，只是借用数组里的 `i32`。
