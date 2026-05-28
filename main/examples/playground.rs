use learning_notes::learning_additions::{
    error_handling, iterators, ownership_borrowing, pattern_matching,
};

fn main() {
    // 这个文件是临时练习入口。
    // 读到哪个模块，就在这里 use 它，然后调用你想观察的函数。
    // 运行命令：
    // cargo run -p learning_notes --example playground

    ownership_borrowing::borrow_without_taking_ownership();

    let first_word = ownership_borrowing::first_word_slice("hello rust");
    println!("第一个单词: {first_word}");

    let result = error_handling::option_and_result_flow("20");
    println!("错误处理结果: {result:?}");

    let squared = iterators::square_even_numbers(&[1, 2, 3, 4, 5, 6]);
    println!("偶数平方: {squared:?}");

    let command = pattern_matching::Command::Move { x: 3, y: 4 };
    println!("{}", pattern_matching::describe_command(command));
}
