pub fn test() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 这个表达式将被赋值给 `y`
        x_cube + x_squared + x
    };

    let z = {
        // 分号抑制了这个表达式,`()` 被赋值给 `z`
        2 * x;
    };

    println!("x 是 {:?}", x);
    println!("y 是 {:?}", y);
    println!("z 是 {:?}", z);
}
