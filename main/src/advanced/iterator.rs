/*
Rust 迭代器 (Iterator) 详解
在 Rust 语言中，迭代器 是一个非常重要的概念，几乎贯穿所有 Rust 代码。无论是遍历数组、Vec、HashMap，甚至是数值范围，迭代器都提供了比传统索引访问更简洁和安全的方式。

1. 迭代器基础
Rust 的 迭代器 允许我们遍历集合的元素，而无需关心集合的索引管理。例如，

let arr = [1, 2, 3];
for v in arr {
    println!("{}", v);
}
Rust不会使用索引访问，而是直接把 arr 视为迭代器进行遍历。

2. for 循环的迭代器特性
Rust 的 for 语法是 语法糖，本质上仍然是调用了迭代器的方法。例如，我们可以手动调用 into_iter() 方法：


let arr = [1, 2, 3];
for v in arr.into_iter() {
    println!("{}", v);
}

注意：数组本身 不是 迭代器，而是 实现了 IntoIterator 特征，Rust 自动将其转换为迭代器。


for i in 1..5 {
    println!("{}", i);
}
这里 1..5 也是一个迭代器，它生成 1, 2, 3, 4。

3. 惰性初始化
Rust 中的迭代器是 惰性的，意味着 不会主动执行遍历，除非明确使用它：


let v1 = vec![1, 2, 3];
let v1_iter = v1.iter(); // 仅创建迭代器，不会执行任何操作

for val in v1_iter {
    println!("{}", val); // 这里才开始迭代
}
这样可以确保不会有不必要的性能开销。

4. next() 方法
迭代器的核心方法是 next()，它返回 Some(T) 或 None：


fn main() {
    let arr = [1, 2, 3];
    let mut arr_iter = arr.into_iter();

    assert_eq!(arr_iter.next(), Some(1));
    assert_eq!(arr_iter.next(), Some(2));
    assert_eq!(arr_iter.next(), Some(3));
    assert_eq!(arr_iter.next(), None);
}
要点：

next() 返回 Option<T>，当迭代结束时，返回 None。
迭代是 消耗性的，每次调用 next() 取走一个元素，迭代器不会重置。
由于 next() 改变迭代器的状态，迭代器必须是 可变 (mut)。


5. 模拟 for 循环
Rust 的 for 只是迭代器的语法糖，我们可以用 loop 手动实现它：

let values = vec![1, 2, 3];

let result = match IntoIterator::into_iter(values) {
    mut iter => loop {
        match iter.next() {
            Some(x) => println!("{}", x),
            None => break,
        }
    },
};
result
这里：

into_iter() 让 Vec 变成迭代器。
loop + match 模拟 for 的迭代过程。



6. IntoIterator 特征
Vec<T> 实现了 IntoIterator，可以转换为迭代器：

impl<I: Iterator> IntoIterator for I {
    type Item = I::Item;
    type IntoIter = I;

    fn into_iter(self) -> I {
        self
    }
}
所以，我们甚至可以写 多个 into_iter()：


for v in vec![1, 2, 3].into_iter().into_iter().into_iter() {
    println!("{}", v);
}

*/


//7. into_iter vs iter vs iter_mut

fn main() {
    let values = vec![1, 2, 3];

    for v in values.into_iter() {//拿走所有权。
        println!("{}", v);
    }

    let values = vec![1, 2, 3];
    let _values_iter = values.iter(); //借用，不会影响原值。 不会拿走所有权
    println!("{:?}", values);

    let mut values = vec![1, 2, 3];
    let mut values_iter_mut = values.iter_mut();//可变借用，可以修改值。
    if let Some(v) = values_iter_mut.next() {
        *v = 0;
    }
    println!("{:?}", values); // [0, 2, 3]
}

/*
区别：

into_iter() 拿走所有权。
iter() 借用，不会影响原值。
iter_mut() 可变借用，可以修改值。
*/
/*

8. 迭代器适配器
(1) 消费者适配器
这些方法会 消耗 迭代器的元素：

sum()：求和
collect()：收集成 Vec<T> 等
count()：计数
for_each()：遍历

let v1 = vec![1, 2, 3];
let total: i32 = v1.iter().sum();
assert_eq!(total, 6);


(2) 迭代器适配器
这些方法不会直接消耗迭代器，而是返回新的迭代器：

map()：映射值
filter()：过滤
zip()：合并两个迭代器

let v1 = vec![1, 2, 3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
assert_eq!(v2, vec![2, 3, 4]);

*/

use std::collections::HashMap;

fn main() {
    let names = ["Alice", "Bob"];
    let ages = [25, 30];
    let people: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
    println!("{:?}", people);
}


// 9. 自定义迭代器
// 如果我们要自定义迭代器，只需实现 Iterator 特征：


struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
/*
10. 迭代器 vs for 循环的性能
Rust 的迭代器 和 for 循环一样快，因为编译器会优化它们 (zero-cost abstractions)。


fn sum_iter(x: &[f64]) -> f64 {
    x.iter().sum::<f64>()
}


总结：

1.迭代器是惰性的
2.next() 逐步获取元素
3.使用 map()、filter() 进行链式操作
4.不会损失性能

📘 TypeScript 对比
====================
| 特性 | Rust | TypeScript |
|------|------|-----------|
| 创建 | `v.iter()` / `v.into_iter()` | `v[Symbol.iterator]()` |
| 惰性 | ✅ 不 `.collect()` 就不执行 | ❌ 即时执行 |
| map | `.map(\|x\| x * 2)` | `.map(x => x * 2)` |
| filter | `.filter(\|x\| x > 0)` | `.filter(x => x > 0)` |
| 消费 | `.collect()`, `.sum()`, `.for_each()` | `.reduce()`, `.forEach()` |
| 三种 iter | `iter()` / `iter_mut()` / `into_iter()` | 只有一种 |

⚠️ 重要区别：
- Rust 的 `.map()` / `.filter()` 是惰性的——它们创建新的迭代器，
  直到调用 `.collect()` 才真正执行。
- TS 的数组方法立即执行并创建中间数组（可能浪费内存）。
- Rust 迭代器是零开销抽象——编译后 ≈ 手写循环的性能。

详细对照 → rust_vs_typescript.rs §14 "迭代器"