// 🔑 要点：'static 生命周期——整个程序运行期间有效
// 静态变量可以直接在线程间共享（无需分配）

use std::thread;

pub fn sum(slice: &'static [i32]) -> i32 {
    let mid = slice.len() / 2;
    let (left, right) = slice.split_at(mid);

    let handle1 = thread::spawn(move || left.iter().sum::<i32>());
    let handle2 = thread::spawn(move || right.iter().sum::<i32>());

    handle1.join().unwrap() + handle2.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn empty() { static ARRAY: [i32; 0] = []; assert_eq!(sum(&ARRAY), 0); }
    #[test] fn one() { static ARRAY: [i32; 1] = [1]; assert_eq!(sum(&ARRAY), 1); }
    #[test] fn five() { static ARRAY: [i32; 5] = [1,2,3,4,5]; assert_eq!(sum(&ARRAY), 15); }
}
