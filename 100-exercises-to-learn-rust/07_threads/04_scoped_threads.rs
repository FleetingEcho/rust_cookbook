// 🔑 要点：scope 线程——借用而非获取所有权
// thread::scope 确保所有子线程在作用域结束前完成

pub fn sum(v: Vec<i32>) -> i32 {
    std::thread::scope(|s| {
        let mid = v.len() / 2;
        let (left, right) = v.split_at(mid);

        // scope 内的线程可以借用局部变量
        let handle1 = s.spawn(|| left.iter().sum::<i32>());
        let handle2 = s.spawn(|| right.iter().sum::<i32>());

        handle1.join().unwrap() + handle2.join().unwrap()
    })
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
}
