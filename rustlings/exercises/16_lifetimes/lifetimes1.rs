// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?

// TODO: Fix the compiler error by updating the function signature.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 一个结构体持有两个不同生命周期的引用
struct TwoReferences<'a, 'b> {
    first: &'a str,
    second: &'b str,
}

impl<'a, 'b> TwoReferences<'a, 'b> {
    // 方法返回第一个引用（生命周期 'a）
    fn get_first(&self) -> &'a str {
        self.first
    }

    // 方法返回第二个引用（生命周期 'b）
    fn get_second(&self) -> &'b str {
        self.second
    }

    // 方法返回较短的那个生命周期（需要约束）
    fn get_shorter<'c>(&self) -> &'c str
    where
        'a: 'c,
        'b: 'c,
    {
        if self.first.len() < self.second.len() {
            self.first // 'a 必须 >= 'c
        } else {
            self.second // 'b 必须 >= 'c
        }
    }
}

fn main() {
    let long_lived = String::from("活的久");
    let result;

    {
        let short_lived = String::from("活的短");

        let container = TwoReferences {
            first: &long_lived,
            second: &short_lived,
        };

        result = container.get_first();
        println!("{}", result); // 输出："活的久"

        // result2 会失效，因为引用了 short_lived
        let result2 = container.get_second();
        println!("{}", result2); // 输出："活的短"
    }

    // ✅ 可以，因为 result 引用的是 long_lived
    println!("外部访问: {}", result);

    // ❌ 如果取消注释会编译错误
    // println!("{}", result2);  // error: result2 已经失效
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}
