// 运行命令：cargo run -p learning_notes --example rts_arrays
//
// ============================================================
// TypeScript 版本（注释掉）：
// ============================================================
// // 固定长度（TS 用 readonly 或 tuple）
// const fixed: readonly number[] = [1, 2, 3, 4, 5];
//
// // 动态数组
// const arr: number[] = [1, 2, 3, 4, 5];
// arr.push(6);
// arr.pop();
// arr.unshift(0);                   // 头部插入
// arr.shift();                      // 头部删除
// arr.splice(2, 0, 99);             // 中间插入
// arr.splice(2, 1);                 // 中间删除
// arr.includes(3);
// arr.indexOf(3);
// arr.findIndex(x => x > 3);
// arr.slice(1, 3);                  // 不修改原数组
// arr.reverse();                    // 原地反转
// arr.sort((a, b) => a - b);
// arr.map(x => x * 2);
// arr.filter(x => x % 2 === 0);
// arr.reduce((acc, x) => acc + x, 0);
// arr.find(x => x > 3);
// arr.some(x => x > 3);
// arr.every(x => x > 0);
// arr.flat();
// arr.concat([6, 7]);
// arr.join(", ");
// arr.length = 0;                   // 清空
// ============================================================

fn main() {
    // ============================================================
    // 一、固定数组 [T; N]
    // TS 对应：readonly T[]（长度固定，类型固定）
    // 存在栈上，大小编译期确定
    // ============================================================
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("固定数组: {:?}", arr);
    println!("长度: {}", arr.len()); // TS: arr.length
    println!("第一个: {}", arr[0]);
    println!("最后一个: {}", arr[arr.len() - 1]);
    println!("切片: {:?}", &arr[1..4]); // TS: arr.slice(1, 4)

    // 初始化相同值
    let zeros = [0_i32; 5]; // [0, 0, 0, 0, 0]
    let ones = [1_u8; 3];
    println!("零数组: {:?}", zeros);
    println!("一数组: {:?}", ones);

    // 遍历固定数组必须借用 &arr，否则会移动所有权（TS 无此问题）
    for x in &arr {
        // TS: for (const x of arr)
        print!("{x} ");
    }
    println!();

    // 带索引遍历
    for (i, x) in arr.iter().enumerate() {
        // TS: arr.forEach((x, i) => ...)
        print!("[{i}]={x} ");
    }
    println!();

    // ============================================================
    // 二、动态数组 Vec<T>
    // TS 对应：number[] 或 Array<number>
    // 存在堆上，长度可变
    // ============================================================
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("\nVec 初始: {:?}", v);

    // --- 增 ---
    v.push(6); // TS: push()，尾部追加
    println!("push(6): {:?}", v);

    v.insert(0, 0); // TS: unshift(0) 或 splice(0,0,0)，指定位置插入
    println!("insert(0,0): {:?}", v);

    v.insert(3, 99); // TS: splice(3, 0, 99)
    println!("insert(3,99): {:?}", v);

    // --- 删 ---
    v.pop(); // TS: pop()，尾部删除，返回 Option
    println!("pop(): {:?}", v);

    v.remove(0); // TS: shift() 或 splice(0,1)，指定位置删除
    println!("remove(0): {:?}", v);

    // swap_remove：与最后一个元素交换后删除，O(1)
    // TS 无对应操作。不关心顺序时比 remove() 快得多
    let mut sv = vec![10, 20, 30, 40, 50];
    println!("swap_remove 前: {:?}", sv);
    sv.swap_remove(1); // 删除索引 1，把 50 挪过来
    println!("swap_remove(1): {:?}", sv); // [10, 50, 30, 40]

    v.retain(|&x| x != 99); // 保留满足条件的元素，TS: filter() 赋值回去
    println!("retain(!= 99): {:?}", v);

    v.clear(); // TS: arr.length = 0
    println!("clear(): {:?}", v);

    // drain：从 Vec 移除范围元素并拥有它们的所有权
    // TS: splice() 最接近，但 splice 不转移所有权
    let mut dv = vec![1, 2, 3, 4, 5, 6];
    let drained: Vec<i32> = dv.drain(1..4).collect();
    println!("drain [1..4]: dv={:?}, drained={:?}", dv, drained);

    // --- capacity / reserve（内存管理，TS 无对应概念）---
    let mut cv = Vec::with_capacity(10);
    println!("初始 capacity: {}", cv.capacity());
    cv.extend(0..10);
    println!("push 10个后 capacity: {}", cv.capacity());
    cv.push(11); // capacity 翻倍 → 20
    println!("push 第11个后 capacity: {}", cv.capacity());
    cv.shrink_to_fit(); // 缩到刚好够用
    println!("shrink_to_fit 后 capacity: {}", cv.capacity());

    // 重置继续演示
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    println!("\n重置 Vec: {:?}", v);

    // --- 安全索引 .get() ---
    // TS: arr[100] 返回 undefined
    // Rust: v[100] 直接 panic（越界！要用 .get()）
    println!("v.get(100): {:?}", v.get(100)); // None，不会 panic
    println!("v.get(2): {:?}", v.get(2)); // Some(&4)

    // --- 查找 ---
    println!("contains(5): {}", v.contains(&5)); // TS: includes(5)

    // position：TS indexOf() 找不到返回 -1，Rust 返回 Option<usize>
    println!("position(5): {:?}", v.iter().position(|&x| x == 5)); // TS: indexOf(5)
    println!("rposition(5): {:?}", v.iter().rposition(|&x| x == 5)); // TS: lastIndexOf(5)

    // find / findIndex
    let found = v.iter().find(|&&x| x > 4); // TS: find()
    println!("find >4: {:?}", found);

    let idx = v.iter().position(|&x| x > 4); // TS: findIndex()
    println!("findIndex >4: {:?}", idx);

    // some / every
    println!("some >8: {}", v.iter().any(|&x| x > 8)); // TS: some()
    println!("every >0: {}", v.iter().all(|&x| x > 0)); // TS: every()

    // --- 切片 ---
    println!("slice [2..5]: {:?}", &v[2..5]); // TS: slice(2, 5)，不含 5
    println!("slice [..3]: {:?}", &v[..3]); // TS: slice(0, 3)

    // --- chunks / windows（TS 无原生等价物）---
    let cv: Vec<i32> = (1..=10).collect();
    for chunk in cv.chunks(3) {
        print!("{:?} ", chunk); // 每 3 个一组
    }
    println!();

    for window in cv.windows(3) {
        print!("{:?} ", window); // 滑动窗口，步长 1
    }
    println!();

    // --- split_at（安全地一分为二，TS 无对应）---
    let (left, right) = cv.split_at(4);
    println!("split_at(4): left={:?}, right={:?}", left, right);

    // --- 排序 ---
    let mut s = v.clone();
    s.sort(); // TS: sort((a,b) => a-b)，升序
    println!("升序: {:?}", s);

    s.sort_by(|a, b| b.cmp(a)); // 降序
    println!("降序: {:?}", s);

    s.sort_by_key(|&x| std::cmp::Reverse(x)); // 另一种降序写法
    println!("Reverse 降序: {:?}", s);

    // --- 反转 ---
    s.reverse(); // TS: reverse()（原地反转）
    println!("reverse: {:?}", s);

    // --- 二分查找（需有序，TS 无原生方法）---
    let bs = vec![1, 3, 5, 7, 9, 11];
    println!("binary_search(5): {:?}", bs.binary_search(&5)); // Ok(2)
    println!("binary_search(6): {:?}", bs.binary_search(&6)); // Err(3)，6 应插入位置 3

    // --- 去重 ---
    // TS: [...new Set(arr)]
    s.sort();
    s.dedup(); // 去除连续重复（需先排序）
    println!("dedup (去重): {:?}", s);

    // --- map / filter / reduce ---
    let nums = vec![1, 2, 3, 4, 5, 6];

    let doubled: Vec<i32> = nums.iter().map(|&x| x * 2).collect(); // TS: map()
    println!("map *2: {:?}", doubled);

    let evens: Vec<i32> = nums.iter().filter(|&&x| x % 2 == 0).cloned().collect(); // TS: filter()
    println!("filter 偶数: {:?}", evens);

    let sum: i32 = nums.iter().sum(); // TS: reduce((a,b)=>a+b,0)
    let product: i32 = nums.iter().product();
    println!("sum: {sum}, product: {product}");

    let fold_sum = nums.iter().fold(0, |acc, &x| acc + x); // TS: reduce()
    println!("fold sum: {fold_sum}");

    // --- partition：一次遍历分出两类（TS 无原生方法）---
    let (big, small): (Vec<_>, Vec<_>) = nums.iter().cloned().partition(|&x| x > 3);
    println!("partition >3: big={:?}, small={:?}", big, small);

    // --- concat / extend ---
    let mut a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    a.extend(&b); // TS: concat() 或 push(...b)
    println!("extend: {:?}", a);

    // --- join ---
    let joined = a
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println!("join: {joined}"); // TS: join(", ")

    // --- flat_map（类似 TS flatMap）---
    let nested_words = vec!["hello world", "rust is great"];
    let words: Vec<&str> = nested_words
        .iter()
        .flat_map(|s| s.split_whitespace())
        .collect();
    println!("flat_map: {:?}", words); // TS: flatMap(s => s.split(" "))

    // --- collect 成 HashMap / HashSet ---
    // TS: new Map(arr) / new Set(arr)
    let pairs = vec![("a", 1), ("b", 2), ("c", 3)];
    let map: std::collections::HashMap<_, _> = pairs.into_iter().collect();
    println!("collect HashMap: {:?}", map);
    let uniq: std::collections::HashSet<_> = vec![1, 2, 2, 3, 3, 3].into_iter().collect();
    println!("collect HashSet (去重): {:?}", uniq);

    // --- 二维数组 ---
    // TS: number[][]
    let matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    for row in &matrix {
        println!("{:?}", row);
    }
    println!("matrix[1][2] = {}", matrix[1][2]); // 6

    // --- 统计 ---
    let data = vec![1, 2, 3, 4, 5];
    println!("min: {:?}", data.iter().min()); // TS: Math.min(...data)
    println!("max: {:?}", data.iter().max()); // TS: Math.max(...data)
    println!(
        "count 偶数: {}",
        data.iter().filter(|&&x| x % 2 == 0).count()
    );

    // ============================================================
    // 参考：频繁头部操作请用 VecDeque
    // TS: arr.unshift(0) / arr.shift() 是 O(n)
    // Rust Vec: .insert(0, x) / .remove(0) 也是 O(n)
    // VecDeque: push_front / pop_front 是 O(1)
    // ============================================================
    use std::collections::VecDeque;
    let mut deque: VecDeque<i32> = VecDeque::from(vec![2, 3, 4]);
    deque.push_front(1);
    deque.push_back(5);
    println!("VecDeque: {:?}", deque); // [1, 2, 3, 4, 5]
    println!("VecDeque pop_front: {:?}", deque.pop_front()); // Some(1)
    println!("VecDeque pop_back: {:?}", deque.pop_back()); // Some(5)
}
