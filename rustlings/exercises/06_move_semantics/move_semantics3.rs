// TODO: Fix the compiler error in the function without adding any new line.
fn fill_vec(vec: &[i32]) -> Vec<i32> {
    let mut new_vec = vec.to_vec(); // 内部克隆，调用方看不见
    new_vec.push(88);
    new_vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(&vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
