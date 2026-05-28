pub fn vec_test() {
    // 1️⃣ 创建一个 Vec 并填充数据
    let mut numbers: Vec<i32> = vec![10, 20, 30, 40, 50];
    println!("Initial Vec: {:?}", numbers);

    // 2️⃣ 追加元素 60
    numbers.push(60);
    println!("After push(60): {:?}", numbers);

    // 3️⃣ 在索引 2 插入 25
    numbers.insert(2, 25);
    println!("After insert(2, 25): {:?}", numbers);

    // 4️⃣ 删除索引 4 的元素（40）
    numbers.remove(4);
    println!("After remove(4): {:?}", numbers);

    // 5️⃣ 修改索引 3 的元素为 100
    numbers[3] = 100;
    println!("After modifying index 3 to 100: {:?}", numbers);

    // 6️⃣ 弹出最后一个元素
    if let Some(last) = numbers.pop() {
        println!("Popped element: {}", last);
    }
    println!("After pop(): {:?}", numbers);

    // 7️⃣ 遍历 Vec 并打印所有元素
    println!("Iterating through Vec:");
    for num in &numbers {
        print!("{} ", num);
    }
    println!();

    // 8️⃣ 查找 50 的索引
    match numbers.iter().position(|&x| x == 50) {
        Some(index) => println!("50 found at index: {}", index),
        None => println!("50 Not Found"),
    }

    // 9️⃣ 计算 Vec 中所有元素的和
    let sum: i32 = numbers.iter().sum();
    println!("Sum of Vec: {}", sum);

    // 🔟 排序 Vec
    numbers.sort();
    println!("Sorted Vec: {:?}", numbers);

    let vec_numbers = vec![1, 2, 3, 4, 5];
    for num in vec_numbers.into_iter() {
        // vec_numbers 被消耗
        println!("{}", num);
    }
    // println!("{:?}", vec_numbers); // ❌ 编译错误，numbers 已被移动

    {
        let mut nums = vec![5, 3, 8, 1, 2];
        nums.sort(); // 默认升序排序
        nums.sort_unstable(); // 不稳定排序, 速度更快
        println!("{:?}", nums); // [1, 2, 3, 5, 8]
    }

    {
        //自定义排序
        let mut words = vec!["apple", "banana", "grape", "pear"];
        words.sort_by(|a, b| a.len().cmp(&b.len())); // 按字符串长度排序
        println!("{:?}", words); // ["pear", "apple", "grape", "banana"]

        {
            //降序呢？
            words.sort_by(|a, b| b.len().cmp(&a.len())); // 降序

            //浮点数？
            let mut floats = vec![3.2, 1.5, 4.8, 2.1];
            floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }
    }

    {
        let mut people = vec![("Alice", 30), ("Bob", 25), ("Charlie", 35)];
        people.sort_by_key(|&(_, age)| age); // 按年龄升序
        println!("{:?}", people); // [("Bob", 25), ("Alice", 30), ("Charlie", 35)]
        people.sort_by_key(|&(_, age)| std::cmp::Reverse(age));
    }

    {
        let mut nums = vec![5, 3, 8, 1, 2];
        quicksort(&mut nums);
        println!("{:?}", nums); // [1, 2, 3, 5, 8]
    }
}

fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    quicksort(&mut arr[..pivot_index]);
    quicksort(&mut arr[pivot_index + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr[arr.len() - 1]; // 选择最后一个元素作为 pivot
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    return i;
}
