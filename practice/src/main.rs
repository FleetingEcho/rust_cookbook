struct WordCounter {
    words: Vec<String>,
}

impl WordCounter {
    // 创建一个空的 WordCounter
    fn new() -> Self { 
        WordCounter { words:vec![] }
     }

    // 添加一个词（获取所有权）
    fn add(&mut self, word: String) {
        self.words.push(word);
     }

    // 返回长度大于 n 的词的数量（不消耗 self）
    fn count_longer_than(&self, n: usize) -> usize {
        self.words.iter().filter(|w| w.len()>n).count()
     }

    // 返回所有词拼成的字符串，用空格分隔（不消耗 self）
    fn join(&self) -> String {
        return self.words.join(" ")
     }
}

fn main() {
    let mut wc = WordCounter::new();
    wc.add(String::from("hi"));
    wc.add(String::from("hello"));
    wc.add(String::from("rust"));
    wc.add(String::from("programming"));

    assert_eq!(wc.count_longer_than(3), 2);  // "hello", "programming" 等等看你算
    assert_eq!(wc.join(), "hi hello rust programming");
    println!("all assertions passed!");
}