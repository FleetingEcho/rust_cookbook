/*
结构体可见性
结构体的字段具有额外的可见性级别。字段默认为私有，可以使用 pub 修饰符来覆盖。这种可见性只在从结构体定义模块外部访问时才有意义，其目的是实现信息隐藏（封装）。
*/
mod my {
    // 一个具有泛型类型 `T` 公有字段的公有结构体
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // 一个具有泛型类型 `T` 私有字段的公有结构体
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // 一个公有的构造方法
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents: contents }
        }
    }
}

pub fn test() {
    // 具有公有字段的公有结构体可以正常构造
    let open_box = my::OpenBox {
        contents: "公开信息",
    };

    // 且可以正常访问其字段。
    println!("打开的盒子包含：{}", open_box.contents);

    // 具有私有字段的公有结构体不能使用字段名来构造。
    // 错误！`ClosedBox` 有私有字段
    //let closed_box = my::ClosedBox { contents: "机密信息" };
    // TODO ^ 尝试取消此行注释

    // 然而，具有私有字段的结构体可以使用公有构造函数创建
    let _closed_box = my::ClosedBox::new("机密信息");

    // 且无法访问公有结构体的私有字段。
    // 错误！`contents` 字段是私有的
    //println!("封闭的盒子包含：{}", _closed_box.contents);
    // TODO ^ 尝试取消此行注释

    test2();
}

fn function() {
    println!("调用了 `function()`");
}

mod cool {
    pub fn function() {
        println!("调用了 `cool::function()`");
    }
}

mod my_super_self {
    fn function() {
        println!("调用了 `my_super_self::function()`");
    }

    mod cool {
        pub fn function() {
            println!("调用了 `my_super_self::cool::function()`");
        }
    }

    pub fn indirect_call() {
        // 让我们从这个作用域访问所有名为 `function` 的函数！
        print!("调用了 `my_super_self::indirect_call()`，它\n> ");

        // `self` 关键字指的是当前模块作用域 - 在这里是 `my_super_self`。
        // 调用 `self::function()` 和直接调用 `function()` 都会
        // 得到相同的结果，因为它们指向同一个函数。
        self::function();
        function();

        // 我们也可以使用 `self` 来访问 `my_super_self` 内的另一个模块：
        self::cool::function();

        // `super` 关键字指的是父作用域（`my_super_self` 模块之外）。
        super::function();

        // 这将绑定到 *crate* 作用域中的 `cool::function`。
        // 在这种情况下，crate 作用域是最外层作用域。
        {
            use crate::rust_by_example::examples::struct_visit::cool::function as root_function;
            root_function();
        }
    }
}

fn test2() {
    my_super_self::indirect_call();
}

/*
use 声明
use 声明可以将完整路径绑定到新名称，以便更轻松地访问。它通常这样使用：
*/
/*
use crate::deeply::nested::{
    my_first_function,
    my_second_function,
    AndATraitType
};

fn main() {
    my_first_function();
}


// 将 `deeply::nested::function` 路径绑定到 `other_function`。
use deeply::nested::function as other_function;

fn function() {
    println!("调用了 `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("调用了 `deeply::nested::function()`");
        }
    }
}

fn main() {
    // 更方便地访问 `deeply::nested::function`
    other_function();

    println!("进入代码块");
    {
        // 这等同于 `use deeply::nested::function as function`。
        // 这个 `function()` 将遮蔽外部的同名函数。
        use crate::deeply::nested::function;

        // `use` 绑定具有局部作用域。在这种情况下，
        // `function()` 的遮蔽仅在此代码块内有效。
        function();

        println!("离开代码块");
    }

    function();
}
*/
