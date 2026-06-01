// TODO: Fix the compiler error by adding one or two characters.
#[rustfmt::skip]
macro_rules! my_macro {   // 定义一个名为 my_macro 的宏
    () => {               // 第一种模式：不带参数
        println!("Check out my macro!");
    };                    // 分号分隔不同的模式
    
    ($xxx:expr) => {      // 第二种模式：接受一个表达式参数
        println!("Look at this other macro: {}", $xxx);
    }                     // 最后一个模式后分号可选
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
