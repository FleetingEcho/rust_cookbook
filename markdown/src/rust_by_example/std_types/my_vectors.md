# Vectors

## 向量基础

```rust
pub fn test() {
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("将 (0..10) 收集到：{:?}", collected_iterator);

    let mut xs = vec![1i32, 2, 3];
    println!("初始向量：{:?}", xs);

    xs.push(4);
    println!("向量：{:?}", xs);

    println!("向量长度：{}", xs.len());
    println!("第二个元素：{}", xs[1]);
    println!("弹出最后一个元素：{:?}", xs.pop());

    println!("xs 的内容：");
    for x in xs.iter() {
        println!("> {}", x);
    }

    for (i, x) in xs.iter().enumerate() {
        println!("在位置 {} 的值是 {}", i, x);
    }

    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("更新后的向量：{:?}", xs);
}
```

## Vector 内部结构

向量由 3 个参数表示：

- 指向数据的指针
- 长度
- 容量

只要长度小于容量，向量就可以增长。超过容量时会重新分配更大的内存。
