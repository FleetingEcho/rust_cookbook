// 这个 crate 是一个库
#![crate_type = "lib"]
// 这个库的名称是 "rary"
#![crate_name = "rary"]

pub fn public_function() {
    println!("调用了 rary 的 `public_function()`");
}

fn private_function() {
    println!("调用了 rary 的 `private_function()`");
}

pub fn indirect_access() {
    print!("调用了 rary 的 `indirect_access()`，它\n> ");

    private_function();
}

fn main(){
  println!("success");
}

// rustc  --crate-type=lib ./src/mod_split/rary.rs
//当使用 crate_type 属性时，我们就不再需要向 rustc 传递 --crate-type 标志。

// rustc ./src/mod_split/rary.rs