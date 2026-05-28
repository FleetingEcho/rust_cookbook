pub fn array_practice() {
    let mut numbers: [i32; 10] = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]; // 10个元素的数组
    println!(
        "initial numbers, {:?},length is {:?}, the number at index 2 is::{:?}",
        numbers,
        numbers.len(),
        numbers[2]
    );

    let slice = &numbers[1..4];
    println!("slice, {:?}", slice);

    let repeated_numbers = [1; 5]; // 初始化所有元素
    println!("repeated numbers, {:?}", repeated_numbers);

    // for loop
    // for num in numbers{
    //   println!("number is {:?}",num);//会复制数字，不消耗，是副本
    // }
    println!("{:?}", numbers); // 仍然可以访问

    // iter loop
    for num in numbers[0..2].iter() {
        //iter() 返回 不可变引用 &T，不会消耗数组。
        println!("iter number is {:?}", num);
    }

    for num in numbers.iter_mut() {
        *num *= 2; // 通过 &mut T 修改值
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
        .filter(|x| x % 3 == 0) // 先筛选偶数
        .map(|x| x * 10) // 再乘以 10
        .collect();

    println!("{:?}", result); // [20, 40, 60]
}

/*
iter() 返回 不可变引用 &T。
map 作用于每个元素，默认会保留引用 &T, 数字会自动解引用
filter 需要用 &T 进行比较，但它接受 Fn(&T) -> bool，所以我们可以选择是否解引用。

**只有在二级引用的时候才有意义

用 .copied() 可以避免 *x，让迭代器直接返回 i32 值。

无论是 Vec<i32> 还是 Vec<&i32>，打印输出的内容看起来都一样，但它们在 内存管理 和 所有权 方面是完全不同的。

println! 会自动解引用 &i32，所以无论是 i32 还是 &i32，最终都会被正确地打印出来


even_numbers 拥有这些值，可以修改它们，而不会影响 numbers。
even_numbers1 不拥有数据，只是借用 numbers 里的 i32。

*/
