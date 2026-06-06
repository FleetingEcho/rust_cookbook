// 🔑 要点：Vec::leak 将堆内存泄漏为 &'static mut [T]
// 泄漏的内存不会被释放，在整个程序运行期间有效

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    // 泄漏 Vec 的堆分配，获得 'static 切片
    let slice: &'static [i32] = v.leak();
    let mid = slice.len() / 2;
    let (left, right) = slice.split_at(mid);

    let handle1 = thread::spawn(move || left.iter().sum::<i32>());
    let handle2 = thread::spawn(move || right.iter().sum::<i32>());

    handle1.join().unwrap() + handle2.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }
    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }
    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }
    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }
}
