// 同样，`mod inaccessible` 和 `mod nested` 会分别定位到
// `nested.rs` 和 `inaccessible.rs` 文件，
// 并将它们的内容插入到这里对应的模块中
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("调用了 `my::function()`");
}

fn private_function() {
    println!("调用了 `my::private_function()`");
}

pub fn indirect_access() {
    print!("调用了 `my::indirect_access()`，它\n> ");

    private_function();
}