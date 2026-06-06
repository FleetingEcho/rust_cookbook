// 🔑 要点：&mut [i32] 可变切片——可以修改元素
// 平方函数：将切片中每个元素替换为其平方

fn squared(slice: &mut [i32]) {
    for val in slice.iter_mut() {
        *val = (*val) * (*val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        let mut s = vec![];
        squared(&mut s);
        assert_eq!(s, vec![]);
    }
    #[test]
    fn one() {
        let mut s = [2];
        squared(&mut s);
        assert_eq!(s, [4]);
    }
    #[test]
    fn multiple() {
        let mut s = vec![2, 4];
        squared(&mut s);
        assert_eq!(s, vec![4, 16]);
    }
}
