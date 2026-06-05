// 运行命令：cargo run -p learning_notes --example rts_tuples
//
// ============================================================
// TypeScript 版本（注释掉）：
// ============================================================
// // 元组类型
// const t: [number, string, boolean] = [42, "hello", true];
// const [num, str, flag] = t;    // 解构
// t[0]; t[1]; t[2];              // 索引访问
//
// // 函数返回多个值
// function minMax(arr: number[]): [number, number] {
//     return [Math.min(...arr), Math.max(...arr)];
// }
// const [min, max] = minMax([3, 1, 4, 1, 5]);
//
// // 命名元组（TS 4.0+）
// type Range = [start: number, end: number];
//
// // 忽略某些元素
// const [first, , third] = [1, 2, 3];
//
// // 用 Map 的 entries 返回元组数组
// const entries: [string, number][] = Object.entries({ a: 1 }) as any;
// ============================================================

use std::collections::HashMap;

fn main() {
    // ============================================================
    // 一、基本元组
    // TS: const t: [number, string, boolean] = [42, "hello", true]
    // ============================================================
    let t: (i32, &str, bool) = (42, "hello", true);
    println!("元组: {:?}", t);

    // --- 索引访问（从 .0 开始，不是 [0]）---
    // TS: t[0], t[1], t[2]
    println!(".0 = {}", t.0);
    println!(".1 = {}", t.1);
    println!(".2 = {}", t.2);

    // --- 解构 ---
    // TS: const [num, text, flag] = t
    let (num, text, flag) = t;
    println!("解构: num={num}, text={text}, flag={flag}");

    // 忽略某些字段用 _ 占位
    // TS: const [first, , last] = t
    let (first, _, last) = t;
    println!("忽略中间: first={first}, last={last}");

    // ============================================================
    // 二、可变元组
    // ============================================================
    let mut point = (0_i32, 0_i32);
    point.0 = 10;
    point.1 = 20;
    println!("坐标: {:?}", point);

    // ============================================================
    // 三、函数返回多个值（元组最常见的用途）
    // TS 通常返回对象 { min, max }，Rust 习惯用元组
    // ============================================================
    fn min_max(v: &[i32]) -> (i32, i32) {
        let min = *v.iter().min().unwrap();
        let max = *v.iter().max().unwrap();
        (min, max)
    }

    fn divide(a: i32, b: i32) -> (i32, i32) {
        (a / b, a % b) // 同时返回商和余数，TS 需要对象或两次调用
    }

    let data = [3, 1, 4, 1, 5, 9, 2, 6];
    let (min, max) = min_max(&data);
    println!("min={min}, max={max}");

    let (quotient, remainder) = divide(17, 5);
    println!("17/5: 商={quotient}, 余={remainder}");

    // ============================================================
    // 四、嵌套元组
    // ============================================================
    let nested = ((1_i32, 2_i32), (3_i32, 4_i32));
    println!("嵌套 [0][0]={}, [1][1]={}", nested.0 .0, nested.1 .1);

    // ============================================================
    // 五、单元素元组（注意必须加逗号）
    // TS: [42] 是 [number] 类型
    // ============================================================
    let single = (42_i32,); // (42) 只是括号，不是元组！
    println!("单元素元组: {:?}", single);
    println!("取值: {}", single.0);

    // ============================================================
    // 六、元组数组（常见于 HashMap::iter() 等）
    // TS: [string, number][]
    // ============================================================
    let pairs: Vec<(&str, i32)> = vec![("alice", 90), ("bob", 85), ("charlie", 92)];

    for (name, score) in &pairs {
        // TS: for (const [name, score] of pairs)
        println!("{name}: {score}");
    }

    // HashMap 的 iter() 返回 (&K, &V) 元组
    let mut map = HashMap::new();
    map.insert("x", 1_i32);
    map.insert("y", 2);
    for (k, v) in &map {
        // TS: for (const [k, v] of map)
        println!("key={k}, val={v}");
    }

    // ============================================================
    // 七、元组结构体（命名元组）
    // TS 4.0+: type Point = [x: number, y: number]
    // Rust: 给元组包装一个类型名，增加语义
    // ============================================================
    struct Meters(f64); // 包装类型，防止把 kg 传给需要 m 的函数
    struct Kilograms(f64);

    fn print_distance(d: Meters) {
        println!("距离: {} 米", d.0);
    }

    let dist = Meters(3.14);
    let _weight = Kilograms(70.0);
    print_distance(dist);
    // print_distance(_weight); // ❌ 类型不匹配，编译时就能发现错误

    // 另一个例子：坐标点
    struct Point(f64, f64);
    let p = Point(3.0, 4.0);
    let distance = (p.0 * p.0 + p.1 * p.1).sqrt();
    println!("到原点距离: {:.2}", distance);

    // ============================================================
    // 八、用元组作为 HashMap 的复合键
    // TS: Map<string, number> 用模板字符串 `${x},${y}` 作键
    // Rust 可以直接用元组作键（只要实现了 Eq + Hash）
    // ============================================================
    let mut grid: HashMap<(i32, i32), &str> = HashMap::new();
    grid.insert((0, 0), "原点");
    grid.insert((1, 0), "右");
    grid.insert((0, 1), "上");
    println!("(0,0): {:?}", grid.get(&(0, 0)));
    println!("(1,0): {:?}", grid.get(&(1, 0)));

    // ============================================================
    // 九、模式匹配元组（Rust 特有，TS 无直接对应）
    // ============================================================
    let pair = (true, 42_i32);
    let msg = match pair {
        (true, n) if n > 0 => format!("正数: {n}"),
        (true, _) => "非正数".to_string(),
        (false, _) => "false".to_string(),
    };
    println!("match 元组: {msg}");

    // ============================================================
    // 十、单元类型 ()
    // TS: void（函数无返回值）
    // Rust: 无返回值的函数隐式返回 ()，也叫 unit type
    // ============================================================
    fn do_nothing() -> () { // 等价于 fn do_nothing() {
                            // 什么也不做
    }
    let unit: () = do_nothing();
    println!("unit: {:?}", unit); // ()
}
