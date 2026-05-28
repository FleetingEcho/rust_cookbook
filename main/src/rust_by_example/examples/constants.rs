// 全局变量在所有其他作用域之外声明。
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // 在函数中访问常量
    n > THRESHOLD
}

fn test() {
    let n = 16;

    // 在主线程中访问常量
    println!("这是 {}", LANGUAGE);
    println!("阈值为 {}", THRESHOLD);
    println!("{} 是 {}", n, if is_big(n) { "大的" } else { "小的" });

    // 错误！不能修改 `const`。
    THRESHOLD = 5;
    // 修复：^ 注释掉此行
}