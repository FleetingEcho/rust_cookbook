// 运行命令：cargo run -p learning_notes --example rts_closures_iter
//
// ============================================================
// TypeScript 版本（注释掉）：
// ============================================================
// // 箭头函数
// const double = (x: number) => x * 2;
// const add = (a: number, b: number) => a + b;
// const greet = (name: string) => `Hello, ${name}!`;
//
// // 捕获外部变量（TS 自动捕获，无需声明）
// const multiplier = 3;
// const multiply = (x: number) => x * multiplier;
//
// // 高阶函数
// function applyTwice(f: (x: number) => number, x: number): number {
//     return f(f(x));
// }
//
// // 工厂函数（返回函数）
// function makeAdder(n: number) {
//     return (x: number) => x + n;
// }
// const add5 = makeAdder(5);
//
// // 迭代器链
// const nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
// nums.filter(x => x % 2 === 0).map(x => x * x).reduce((a, b) => a + b, 0);
// nums.find(x => x > 5);
// nums.findIndex(x => x > 5);
// nums.some(x => x > 8);
// nums.every(x => x > 0);
// nums.flatMap(x => [x, x * 2]);
// nums.slice(0, 3);            // take
// nums.slice(3);               // skip
// nums.entries();              // enumerate
// [...a, ...b];                // chain/concat
// a.map((v, i) => [v, b[i]]); // zip（手动）
// ============================================================

fn main() {
    // ============================================================
    // 一、闭包基础
    // TS 对应：箭头函数 const fn = (x: T) => ...
    // ============================================================

    // 基本闭包（编译器可以推断参数和返回类型）
    // TS: const double = (x: number) => x * 2
    let double = |x: i32| x * 2;
    let add = |a: i32, b: i32| a + b;
    let is_even = |x: i32| x % 2 == 0;
    println!("double(5): {}", double(5));
    println!("add(3,4): {}", add(3, 4));
    println!("is_even(4): {}", is_even(4));

    // 多行闭包（用花括号）
    // TS: const complex = (x: number) => { const d = x * 2; return d + 1; }
    let complex = |x: i32| {
        let doubled = x * 2;
        doubled + 1 // 最后一个表达式是返回值，不需要 return
    };
    println!("complex(5): {}", complex(5));

    // ============================================================
    // 二、捕获外部变量
    // TS 自动捕获，无需声明方式；Rust 需要指定捕获方式
    // ============================================================

    // 默认：按引用捕获（最常见，不转移所有权）
    let multiplier = 3_i32;
    let multiply = |x| x * multiplier; // 自动捕获 multiplier 的引用
    println!("multiply(5): {}", multiply(5));
    println!("multiplier 还能用: {multiplier}"); // 未被移走，还可以用

    // move 闭包：将变量所有权移入闭包（常用于线程/异步，生命周期需要更长）
    // TS 没有对应概念（JS 有 GC）
    let greeting = String::from("你好");
    let greet = move |name: &str| format!("{}, {}！", greeting, name);
    println!("{}", greet("Alice"));
    // println!("{}", greeting); // ❌ greeting 所有权已移入闭包

    // ============================================================
    // 三、Fn / FnMut / FnOnce（三种闭包 trait）
    // TS 所有函数都是可重复调用的，Rust 区分三种
    // ============================================================

    // Fn：可以多次调用，只读捕获（最常见）
    // TS: (x: number) => number
    fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
        f(f(x))
    }
    println!("Fn apply_twice: {}", apply_twice(|x| x + 3, 7)); // 13

    // FnMut：可以多次调用，可变捕获
    // TS: 闭包内修改外部变量（TS 没有明确区分）
    let mut count = 0;
    let mut increment = || {
        count += 1;
        count
    };
    println!("FnMut: {}", increment()); // 1
    println!("FnMut: {}", increment()); // 2
    println!("FnMut: {}", increment()); // 3

    // FnOnce：只能调用一次（消耗了捕获的变量）
    // TS 没有对应概念
    let name = String::from("Rust");
    let consume = move || {
        println!("FnOnce 消耗: {name}");
        // name 在这里被消耗
    };
    consume();
    // consume(); // ❌ 不能再调用

    // ============================================================
    // 四、工厂函数（返回闭包）
    // TS: function makeAdder(n: number) { return (x: number) => x + n; }
    // ============================================================
    fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n // move 把 n 的所有权移入闭包
    }

    fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x * n
    }

    let add5 = make_adder(5);
    let triple = make_multiplier(3);
    println!("add5(10): {}", add5(10)); // 15
    println!("triple(7): {}", triple(7)); // 21

    // ============================================================
    // 五、迭代器链
    // TS 对应：Array 方法链 .filter().map().reduce()
    // 关键差异：Rust 迭代器是惰性的，不调用 collect() 不会执行
    // ============================================================
    let numbers = vec![1_i32, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // filter + map + sum
    // TS: numbers.filter(x => x % 2 === 0).map(x => x * x).reduce((a,b) => a+b, 0)
    let result: i32 = numbers
        .iter()
        .filter(|&&x| x % 2 == 0) // 偶数
        .map(|&x| x * x) // 平方
        .sum(); // 求和
    println!("偶数平方和: {result}"); // 4+16+36+64+100 = 220

    // map（TS: map()）
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("map *2: {:?}", doubled);

    // filter（TS: filter()）
    let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    println!("filter 偶数: {:?}", evens);

    // fold（TS: reduce()）
    // TS: numbers.reduce((acc, x) => acc + x, 0)
    let sum = numbers.iter().fold(0_i32, |acc, &x| acc + x);
    println!("fold sum: {sum}");

    // --- 聚合方法 ---
    println!("sum: {}", numbers.iter().sum::<i32>()); // TS: reduce((a,b)=>a+b,0)
    println!("product: {}", numbers.iter().product::<i32>());
    println!("min: {:?}", numbers.iter().min()); // TS: Math.min(...arr)
    println!("max: {:?}", numbers.iter().max()); // TS: Math.max(...arr)
    println!("count: {}", numbers.iter().count()); // TS: arr.length

    // --- 查找 ---
    let first_even = numbers.iter().find(|&&x| x % 2 == 0); // TS: find()
    println!("find: {:?}", first_even);

    let pos = numbers.iter().position(|&x| x > 5); // TS: findIndex()
    println!("position: {:?}", pos);

    // --- 判断 ---
    println!("any >8: {}", numbers.iter().any(|&x| x > 8)); // TS: some()
    println!("all >0: {}", numbers.iter().all(|&x| x > 0)); // TS: every()

    // --- take / skip（TS: slice）---
    let first3: Vec<_> = numbers.iter().take(3).collect(); // TS: slice(0, 3)
    let rest: Vec<_> = numbers.iter().skip(3).collect(); // TS: slice(3)
    println!("take(3): {:?}", first3);
    println!("skip(3): {:?}", rest);

    // --- enumerate（TS: entries() 或 forEach 的 index 参数）---
    for (i, val) in numbers.iter().enumerate().take(3) {
        println!("  [{i}] = {val}"); // TS: arr.forEach((v, i) => ...)
    }

    // --- flat_map（TS: flatMap）---
    // TS: ["hello world", "rust"].flatMap(s => s.split(" "))
    let sentences = vec!["hello world", "rust is great"];
    let words: Vec<&str> = sentences
        .iter()
        .flat_map(|s| s.split_whitespace())
        .collect();
    println!("flat_map: {:?}", words);

    // --- zip（TS 需要手动实现）---
    // TS: a.map((v, i) => [v, b[i]])
    let letters = vec!['a', 'b', 'c'];
    let digits = vec![1_i32, 2, 3];
    let zipped: Vec<_> = letters.iter().zip(digits.iter()).collect();
    println!("zip: {:?}", zipped); // [('a', 1), ('b', 2), ('c', 3)]

    // --- chain（TS: concat 或展开 [...a, ...b]）---
    let a = vec![1_i32, 2, 3];
    let b = vec![4_i32, 5, 6];
    let chained: Vec<_> = a.iter().chain(b.iter()).collect();
    println!("chain: {:?}", chained);

    // --- 计数分组 ---
    let count_evens = numbers.iter().filter(|&&x| x % 2 == 0).count(); // TS: filter().length
    println!("偶数个数: {count_evens}");

    // --- 字符串收集 ---
    // TS: numbers.map(String).join(", ")
    let joined = numbers
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println!("join: {joined}");

    // --- 惰性求值演示（Rust 独有，TS 的 Array 方法是即时执行的）---
    // 下面的代码不会执行，因为没有 collect()
    let _lazy = numbers.iter().map(|&x| {
        // 这段代码不会被调用，直到被消耗（collect/sum/any 等）
        x * 1000
    });
    // 只有加上 .collect::<Vec<_>>() 才会真正执行
    println!("（惰性迭代器只在被消耗时才执行）");
}
