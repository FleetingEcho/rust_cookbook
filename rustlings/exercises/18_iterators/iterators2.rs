// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

// 当前代码
fn capitalize_words_vector1(words: &[&str]) -> Vec<String> {
    let new_words = words.to_vec(); // ❌ 不必要的转换
    new_words
        .iter()
        .map(|x| capitalize_first(x))
        .map(|x| x.to_string())
        .collect()
    //                                    ↑ 已经返回 String  ↑ 又转一次 String？
}

fn capitalize_words_string1(words: &[&str]) -> String {
    let arr: Vec<String> = words
        .to_vec()
        .iter()
        .map(|x| capitalize_first(x).to_string())
        .collect();
    //                     ↑ 不必要        ↑ iter() ↑ capitalize_first 已经返回 String，不需要 .to_string()
    arr.join("")
}

fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|&word| capitalize_first(word)).collect()
    //     ↑ 遍历切片      ↑ 解引用    ↑ 直接收集
}

fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|&word| capitalize_first(word)).collect()
    //     ↑ 直接 collect() 到 String，不需要 join
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
