pub fn test() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // 将 `an_integer` 复制到 `copied_integer`
    let copied_integer = an_integer;

    println!("整数：{:?}", copied_integer);
    println!("布尔值：{:?}", a_boolean);
    println!("单元值：{:?}", unit);

    // 编译器会对未使用的变量绑定发出警告
    // 可以通过在变量名前加下划线来消除这些警告
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
    // 修复：^ 在变量名前加下划线以消除警告
    // 注意：在浏览器中可能不会显示警告

    test5()
}

fn test2() {
    // 变量绑定默认是不可变的,但可以使用 mut 修饰符来改变这一行为。
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("修改前：{}", mutable_binding);

    // 正确
    mutable_binding += 1;

    println!("修改后：{}", mutable_binding);

    // 错误！不能给不可变变量赋新值
    // _immutable_binding += 1;
}

// 作用域和遮蔽
// 变量绑定有作用域,它们被限制在一个代码块中生存。代码块是由花括号 {} 包围的一系列语句。
fn test3() {
    // 这个绑定存在于 main 函数中
    let long_lived_binding = 1;

    // 这是一个代码块,它的作用域比 main 函数小
    {
        // 这个绑定只存在于此代码块中
        let short_lived_binding = 2;

        println!("内部 short：{}", short_lived_binding);
    }
    // 代码块结束

    // 错误！`short_lived_binding` 在此作用域中不存在
    // println!("外部 short：{}", short_lived_binding);
    // 修复：^ 注释掉此行

    println!("外部 long：{}", long_lived_binding);
}

fn test4() {
    let shadowed_binding = 1;

    {
        println!("被遮蔽前：{}", shadowed_binding);

        // 这个绑定*遮蔽*了外部的绑定
        let shadowed_binding = "abc";

        println!("内部代码块中被遮蔽：{}", shadowed_binding); //abc
    }
    println!("内部代码块外：{}", shadowed_binding); //1

    // 这个绑定*遮蔽*了之前的绑定
    let shadowed_binding = 2;
    println!("外部代码块中被遮蔽：{}", shadowed_binding); //2
}

//先声明再初始化
fn test5() {
    // 声明一个变量绑定
    let a_binding;

    {
        let x = 2;

        // 初始化绑定
        a_binding = x * x;
    }

    println!("绑定：{}", a_binding); //4

    let another_binding;

    // 错误！使用未初始化的绑定
    // println!("另一个绑定：{}", another_binding);
    // 修复：^ 注释掉此行

    another_binding = 1;

    println!("另一个绑定：{}", another_binding); //1
}

//冻结
//当数据以相同名称被不可变地绑定时,它也会冻结。被冻结的数据在不可变绑定离开作用域之前不能被修改
fn test6() {
    let mut _mutable_integer = 7i32;

    {
        // 通过不可变的 `_mutable_integer` 进行遮蔽
        let _mutable_integer = _mutable_integer;

        // 错误！`_mutable_integer` 在此作用域中被冻结
        // _mutable_integer = 50;
        // 修复：^ 注释掉此行

        // `_mutable_integer` 离开作用域
    }

    // 正确！`_mutable_integer` 在此作用域中未被冻结
    _mutable_integer = 3;
}
