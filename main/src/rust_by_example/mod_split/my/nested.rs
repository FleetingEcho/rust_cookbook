pub fn function() {
    println!("调用了 `my::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("调用了 `my::nested::private_function()`");
}