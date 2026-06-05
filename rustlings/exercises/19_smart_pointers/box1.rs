// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the "cons list", a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: The value of the current item and
// the next item. The last item is a value called `Nil`.

// TODO: Use a `Box` in the enum definition to make the code compile.
#[derive(PartialEq, Debug)]
// ✅ Box 是一个指针（8字节），指向堆上的数据
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// TODO: Create an empty cons list.
fn create_empty_list() -> List {
    List::Nil
}

// TODO: Create a non-empty cons list.
fn from_vec(v: Vec<i32>) -> List {
    v.iter()
        .rev()
        .fold(List::Nil, |acc, &x| List::Cons(x, Box::new(acc)))
}

// 假设 vec = [1, 2, 3]
// 不使用 rev()：从左到右处理
fn from_vec_without_rev(v: Vec<i32>) -> List {
    v.iter()
        .fold(List::Nil, |acc, &x| List::Cons(x, Box::new(acc)))
}

// 执行过程：
// 1. acc = Nil, x=1 → Cons(1, Nil)        // 得到 [1]
// 2. acc = [1], x=2 → Cons(2, [1])        // 得到 [2,1]  ❌ 顺序反了
// 3. acc = [2,1], x=3 → Cons(3, [2,1])    // 得到 [3,2,1] ❌

// 结果：[3,2,1] 但我们需要 [1,2,3]

fn create_non_empty_list() -> List {
    from_vec(vec![1, 2, 3])
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
