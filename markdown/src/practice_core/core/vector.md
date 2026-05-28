# Vec 练习

## Vec 基础操作

```rust
pub fn vec_test() {
    let mut numbers: Vec<i32> = vec![10, 20, 30, 40, 50];
    println!("Initial Vec: {:?}", numbers);

    numbers.push(60);
    println!("After push(60): {:?}", numbers);

    numbers.insert(2, 25);
    println!("After insert(2, 25): {:?}", numbers);

    numbers.remove(4);
    println!("After remove(4): {:?}", numbers);

    numbers[3] = 100;
    println!("After modifying index 3 to 100: {:?}", numbers);

    if let Some(last) = numbers.pop() {
        println!("Popped element: {}", last);
    }
    println!("After pop(): {:?}", numbers);

    println!("Iterating through Vec:");
    for num in &numbers {
        print!("{} ", num);
    }
    println!();

    match numbers.iter().position(|&x| x == 50) {
        Some(index) => println!("50 found at index: {}", index),
        None => println!("50 Not Found"),
    }

    let sum: i32 = numbers.iter().sum();
    println!("Sum of Vec: {}", sum);

    numbers.sort();
    println!("Sorted Vec: {:?}", numbers);
}
```

## 排序

```rust
{
    let mut nums = vec![5, 3, 8, 1, 2];
    nums.sort();
    nums.sort_unstable();
    println!("{:?}", nums);

    let mut words = vec!["apple", "banana", "grape", "pear"];
    words.sort_by(|a, b| a.len().cmp(&b.len()));
    println!("{:?}", words);

    let mut floats = vec![3.2, 1.5, 4.8, 2.1];
    floats.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut people = vec![("Alice", 30), ("Bob", 25), ("Charlie", 35)];
    people.sort_by_key(|&(_, age)| age);
    println!("{:?}", people);
}
```

## 快速排序实现

```rust
fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_index = partition(arr);
    quicksort(&mut arr[..pivot_index]);
    quicksort(&mut arr[pivot_index + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr[arr.len() - 1];
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}
```
