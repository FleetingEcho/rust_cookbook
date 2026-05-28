/*
#![allow(unused_variables)]
type File = String;

fn open(f: &mut File) -> bool {
    true
}
fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    //read(&mut f1, &mut vec![]);
    close(&mut f1);
}
*/

// String
fn main(){
  let s = String::from("hello");

  let slice = &s[0..2]; // slice
  let slice = &s[..2]; //slice same as above

  {let s = String::from("hello");

let len = s.len();

let slice = &s[4..len];
let slice = &s[4..]; // same
let slice = &s[..];// all string
  }

}


/*
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
fn first_word(s: &String) -> &str {
    &s[..1]
}
报错
因为：
1. s.clear(); 需要清空改变 String，因此它需要一个可变借用
2.println!("the first word is: {}", word); 又使用了不可变借用
*/



fn main() {
  // String 与 &str 的转换
    let s = String::from("hello,world!");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());
}

fn say_hello(s: &str) {
    println!("{}",s);
}

/*
 Rust 不允许去索引字符串：因为索引操作，我们总是期望它的性能表现是 O(1)，然而对于 String 类型来说，无法保证这一点，因为 Rust 可能需要从 0 开始去遍历字符串来定位合法的字符。
通过索引区间来访问字符串时，需要格外的小心，一不注意，就会导致你程序的崩溃！
let hello = "中国人";

let s = &hello[0..2];
这里提示的很清楚，我们索引的字节落在了 中 字符的内部，这种返回没有任何意义。

*/

fn main() {
    let mut s = String::from("Hello "); // 必须可变

    s.push_str("rust");// 可以 push字面量
    println!("追加字符串 push_str() -> {}", s);

    s.push('!'); // 只能 push 一个字符
    println!("追加字符 push() -> {}", s);

    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);

    // 插入字符 insert() -> Hello, rust!
// 插入字符串 insert_str() -> Hello, I like rust!
//

    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");// 替换所有匹配到的
    dbg!(new_string_replace);


    let string_replace = "I like rust. Learning rust is my favorite!";
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);// 只替换一次
    dbg!(new_string_replacen);//new_string_replacen = "I like RUST. Learning rust is my favorite!"

    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);//该方法是直接操作原来的字符串，不会返回新的字符串。该方法需要使用 mut 关键字修饰。
}

fn main() {
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
    /*
    p1 = Some(
   '!',
    )
    p2 = Some(
      '文',
    )
    string_pop = "rust pop 中"
    */
}

// 因为中文占3 个字节，所以会报错，不在边界上

fn main() {
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);
}

fn main() {
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);// 因为一个汉字 3 字节
    dbg!(string_truncate);//string_truncate = "测"

}

fn main() {
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);
}



fn main() {
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str， 这里+也就是 add() 方法的第二个参数是一个引用的类型。
    let result = string_append + &string_rust;
    let mut result = result + "!"; // `result + "!"` 中的 `result` 是不可变的
    result += "!!!";

    println!("连接字符串 + -> {}", result); //连接字符串 + -> hello rust!!!!

}

fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + &s2;
    assert_eq!(s3,"hello,world!");
    // 下面的语句如果去掉注释，就会报错
    // println!("{}",s1);
}

fn main() {
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);//hello rust!

}


fn main(){

for c in "中国人".chars() {
    println!("{}", c);
}

for b in "中国人".bytes() {
    println!("{}", b);
}
/*
228
184
173
229
155
189
228
186
186

*/
}


// 📘 TypeScript 对比
// ====================
// Rust 的复合类型对应 TS 的 class + object：
//
// ```rust
// struct User { name: String, age: u8 }      // Rust
// impl User { fn greet(&self) { ... } }       // 方法在 impl 块
// ```
// ```ts
// class User {                                // TS
//   constructor(public name: string, public age: number) {}
//   greet() { ... }                           // 方法在 class 内部
// }
// ```
//
// | 维度 | Rust | TypeScript |
// |------|------|-----------|
// | 结构体 | `struct` 纯数据 | `class` 数据+方法 |
// | 方法定义 | 单独 `impl` 块 | class 内部定义 |
// | 更新语法 | `User { name: "a", ..old }` | `{ ...old, name: "a" }` |
// | 字段可见性 | 默认私有（`pub` 公开） | `public` / `private` |
// | new 方法 | 惯例 `fn new() -> Self` | `constructor()` |
//
// 详细对照 → rust_vs_typescript.rs §4 "复合类型"