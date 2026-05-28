# 迭代

## 基础迭代

```rust
pub fn basic_iteration() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("numbers {} {} {}", numbers.len(), numbers[0], numbers[1]);

    let slice = &numbers[0..1];
    println!("slice: {:?}", slice);

    let repeated_numbers = [1; 5];
    println!("repeated numbers:: {:?}", repeated_numbers);

    println!("revisit numbers::{:?}", numbers);

    for num in numbers[0..3].iter() {
        println!("{:?}", num);
    }
    for num in numbers.iter_mut() {
        *num += 2;
    }
    println!("numbers {:?}", numbers);

    {
        let mut numbers: [i128; 5] = [1, 2, 3, 4, 5];
        for item in numbers {
            println!("item is {:?}", item);
        }

        let squared: Vec<i128> = numbers.iter().map(|x| x * x).collect();
        print!("squared items::{:?}", squared);

        let even_numbers: Vec<i128> = numbers.iter().copied().filter(|x| *x % 3 == 0).collect();
    }
}
```

## 迭代方法

| 方法 | 含义 |
|------|------|
| `.iter()` | 不可变借用 |
| `.iter_mut()` | 可变借用 |
| `.into_iter()` | 消费所有权 |
