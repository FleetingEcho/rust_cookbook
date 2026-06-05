// 我们称 array 为数组，Vector 为动态数组。
pub fn array_test() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // 类似 length 5
}

/*
报错
let array = [String::from("rust is good!"); 8];

println!("{:#?}", array);

因为String不是 Copy 类型
你的代码报错的原因是 数组初始化 [value; n] 需要 value 实现 Copy，但 String 不是 Copy 类型，所以不能这样使用。

let array = [value; n];
来初始化一个数组，表示 创建 n 个 value 的副本。但这个 value 必须实现 Copy trait，否则 Rust 不知道如何复制它。
但是 String 存储在堆上，不能直接复制，所以 Rust 不允许使用 [String::from("rust is good!"); 8]，因为 String 没有 Copy trait。

*/

fn example_string_array() {
    // ⛔ [String::from("rust is good!"); 8] 无法编译，String 不是 Copy 类型
    // let array = [String::from("rust is good!"); 8];

    // ✅ 方案 1：用 vec![]
    let array = vec![String::from("rust is good!"); 8];
    println!("{:#?}", array);

    // ⛔ [expr; N] 同样需要 Copy — 下面写法无法编译:
    // let array2 = [String::from("rust is good!"); 8].map(|s| s.clone());
    // ✅ 方案 2：用 from_fn 构造数组再 map
    let array2 = std::array::from_fn::<String, 8, _>(|_| String::from("rust is good!"));
    println!("{:#?}", array2);
}

fn example_array_2d() {
    // 编译器自动推导出one的类型
    let one = [1, 2, 3];
    // 显式类型标注
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

    // 借用arrays的元素用作循环中
    for a in &arrays {
        // 如果写 arrays就变成了，`a` 试图接管 `arrays` 内元素的所有权
        // Rust 会尝试移动 arrays 的每个元素，但 arrays 是栈上固定大小的数组，Rust 不会自动克隆。所以，你需要借用它，而不是移动它。
        // 这里的 `&` 表示借用 `arrays`，不会移动它的元素
        // // `a` 变成 `[u8; 3]`，意味着 `arrays` 里的元素被移动
        // ✅ 牢记：在固定大小数组 [T; N] 上使用 for，必须 &借用！ 🚀

        print!("{:?}: ", a);
        // 将a变成一个迭代器，用于循环
        // 你也可以直接用for n in a {}来进行循环
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }

    // vec不需要借用
    let arrays = vec![vec![1, 2, 3], vec![4, 5, 6]];

    for a in arrays {
        // Vec<T> 默认会借用，除非 `into_iter()`
        print!("{:?}: ", a);
    }
}

fn example_array_iter() {
    let a = [1, 2, 3];

    for n in &a {
        // ✅ 这里 `&a` 让 `n` 变成 `&i32`
        println!("{}", n);
    }

    println!("{:?}", a); // ✅ `a` 仍然可用
}

fn example_array_ref() {
    let a = [1, 2, 3];

    let ref_a = &a; // `&a` 获取 `a` 的地址

    println!("{:p}", ref_a); // 打印 `a` 在内存中的地址
}
/*
虽然 &a 表面上是获取 a 的地址，但 Rust 的借用机制比 C 语言的指针更安全，因为：

✅ &a 本质上就是获取 a 的地址，但它带有 Rust 的安全机制：

防止悬垂指针（借用不能超过 a 的生命周期）。
防止数据竞争（不可变借用和可变借用不能同时存在）。
不会导致 a 失效（不像 for n in a 那样移动所有权）。
🚀 可以把 &a 理解为一个“安全指针”，不仅仅是地址，还保证了数据安全！
*/

fn example_array_vec_ops() {
    // 📌 1️⃣ 数组 `[T; N]` 操作
    let mut arr = [1, 2, 3, 4, 5];

    println!("数组: {:?}", arr);
    arr[0] = 10;
    println!("修改后: {:?}", arr);
    println!("第一个元素: {}", arr[0]);

    // 遍历数组
    for num in &arr {
        print!("{} ", num);
    }
    println!();

    let filtered: Vec<_> = arr.iter().filter(|&x| x % 2 == 0).collect();

    println!("偶数元素: {:?}", filtered);

    let mapped: Vec<_> = arr.iter().map(|x| x * 10).collect();

    println!("乘以 10: {:?}", mapped);

    // 切片 & 长度
    println!("数组切片: {:?}", &arr[1..4]);
    println!("数组长度: {}", arr.len());

    // 📌 2️⃣ Vector `Vec<T>` 操作
    let mut vec = vec![1, 2, 3, 4, 5];

    println!("\nVector: {:?}", vec);
    vec.push(6);
    println!("push(6): {:?}", vec);
    vec.pop();
    println!("pop(): {:?}", vec);
    println!("第 2 个元素: {}", vec[1]);
    println!("安全获取 get(10): {:?}", vec.get(10));

    // 遍历 Vector
    for num in &vec {
        print!("{} ", num);
    }
    println!();

    // 过滤 & 映射
    let filtered: Vec<_> = vec.iter().filter(|&&x| x % 2 == 0).collect();
    println!("偶数元素: {:?}", filtered);

    let mapped: Vec<_> = vec.iter().map(|x| x * 10).collect();
    println!("乘以 10: {:?}", mapped);

    // 插入 & 删除
    vec.insert(2, 99);
    println!("insert(2, 99): {:?}", vec);
    vec.remove(2);
    println!("remove(2): {:?}", vec);

    // Vector 切片 & 清空
    println!("Vector 切片: {:?}", &vec[1..3]);
    vec.clear();
    println!("clear(): {:?}", vec);
}

/*

📌 关键点总结
1️⃣ {:?} 是 Debug 格式化输出


println!("{:?}", vec);   // 单行输出
println!("{:#?}", vec);  // 更美观的多行格式
2️⃣ iter() 生成 &T，map() 和 filter() 的区别


方法	作用	代码示例
.iter()	创建 借用的迭代器，返回 &T	vec.iter()
`.map(	x	x * 10)`
`.filter(	&&x	x % 2 == 0)`
.collect::<Vec<_>>()	收集迭代器结果到 Vec	.collect()
📌 iter()、map()、filter() 深入解析
1️⃣ iter() 生成 &T


let a = [1, 2, 3];
for x in a.iter() {
    println!("x = {}, 地址 = {:p}", x, x);
}
📌 iter() 生成 &i32

ini
Copy
Edit
x = 1, 地址 = 0x1234
x = 2, 地址 = 0x1238
x = 3, 地址 = 0x123C
2️⃣ map() 自动解引用


let mapped: Vec<_> = vec.iter().map(|x| x * 10).collect();
x 是 &i32，但 Rust 自动解引用 *x * 10，所以 x * 10 直接可用。
3️⃣ filter() 需要 &&x


let filtered: Vec<_> = vec.iter().filter(|&&x| x % 2 == 0).collect();
iter() 生成 &i32
filter() 传 &T，所以 x 是 &&i32
&&x 先解一次 & 变 &i32，再解一次变 i32
x % 2 == 0 需要 i32，所以 &&x 先解两次
🚀 什么时候 &x，什么时候 &&x？
类型	iter() 生成的类型	需要几次 &
数组 [T; N]	Iterator<Item = &i32>	`
Vector Vec<T>	Iterator<Item = &i32>，但 filter() 传 &&i32	`
🔥 结论
数组 [T; N] 只需 &x，因为 iter() 直接返回 &i32。
Vector Vec<T> 需要 &&x，因为 filter() 传 &T，导致 x 变 &&i32。
💡 如果 filter(|&x| x % 2 == 0) 报错，就换成 filter(|&&x| x % 2 == 0)！ 🚀

// 📘 TypeScript 对比
// ====================
// | 特性 | Rust | TypeScript |
// |------|------|-----------|
// | 数组类型 | `[T; N]` — 定长（编译期已知） | `T[]` 或 `Array<T>` — 动态长度 |
// | 动态数组 | `Vec<T>` | `Array<T>` |
// | 越界访问 | 运行时 panic！ | 返回 `undefined` |
// | 遍历 | `for i in &arr` 或迭代器 | `for...of` 或 `.forEach()` |
// | 切片 | `&arr[0..3]`（引用，不拷贝） | `.slice(0,3)`（返回新数组） |
//
// ⚠️ TS 没有"定长数组"的概念——所有数组都是动态的。
//    Rust 的 `[T; N]` 是编译期大小固定的栈上数据，性能更高。
//
// 详细对照 → rust_vs_typescript.rs §4 "复合类型"
*/
