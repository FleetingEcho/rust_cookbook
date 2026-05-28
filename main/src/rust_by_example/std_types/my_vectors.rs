pub fn test() {
    // 迭代器可以被收集到向量中
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("将 (0..10) 收集到：{:?}", collected_iterator);

    // 可以使用 `vec!` 宏初始化向量
    let mut xs = vec![1i32, 2, 3];
    println!("初始向量：{:?}", xs);

    // 在向量末尾插入新元素
    println!("将 4 推入向量");
    xs.push(4);
    println!("向量：{:?}", xs);

    // 错误！不可变向量无法增长
    // collected_iterator.push(0);
    // 修复：^ 注释掉此行

    // `len` 方法返回当前存储在向量中的元素数量
    println!("向量长度：{}", xs.len());

    // 使用方括号进行索引（索引从 0 开始）
    println!("第二个元素：{}", xs[1]);

    // `pop` 移除并返回向量的最后一个元素
    println!("弹出最后一个元素：{:?}", xs.pop());

    // 越界索引会导致 panic
    // println!("第四个元素：{}", xs[3]);
    // 修复：^ 注释掉此行

    // 可以轻松地遍历 `Vector`
    println!("xs 的内容：");
    for x in xs.iter() {
        println!("> {}", x);
    }

    // 遍历 `Vector` 时，可以同时用一个单独的变量（`i`）
    // 来枚举迭代计数
    for (i, x) in xs.iter().enumerate() {
        println!("在位置 {} 的值是 {}", i, x);
    }

    // 借助 `iter_mut`，可变的 `Vector` 也可以被遍历，
    // 并且允许修改每个值
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("更新后的向量：{:?}", xs);
}

/*
Vectors
向量（Vector）是可调整大小的数组。与切片（Slice）类似，它们的大小在编译时是未知的，但可以随时增长或缩小。向量由 3 个参数表示：

指向数据的指针
长度
容量
容量表示为向量预留的内存量。只要长度小于容量，向量就可以增长。当需要超过这个阈值时，向量会被重新分配更大的容量。
*/
