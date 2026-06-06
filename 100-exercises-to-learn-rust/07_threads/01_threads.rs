// 🔑 要点：thread::spawn 创建新线程
// join() 等待线程结束并获取返回值
// 注意：闭包会获取其捕获变量的所有权

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let mid = v.len() / 2;
    let left = v[..mid].to_vec();
    let right = v[mid..].to_vec();

    // 两个线程分别计算一半的和
    let handle1 = thread::spawn(move || left.iter().sum::<i32>());
    let handle2 = thread::spawn(move || right.iter().sum::<i32>());

    // 等待线程结束并合并结果
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
    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
