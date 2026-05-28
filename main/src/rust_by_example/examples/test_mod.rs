// 一个名为 `my_mod` 的模块
mod my_mod {
    // 模块中的项默认为私有可见性。
    fn private_function() {
        println!("调用了 `my_mod::private_function()`");
    }

    // 使用 `pub` 修饰符来覆盖默认的可见性。
    pub fn function() {
        println!("调用了 `my_mod::function()`");
    }

    // 同一模块中的项可以访问其他项，
    // 即使是私有的。
    pub fn indirect_access() {
        print!("调用了 `my_mod::indirect_access()`，它\n> ");
        private_function();
    }

    // 模块也可以嵌套
    pub mod nested {
        pub fn function() {
            println!("调用了 `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("调用了 `my_mod::nested::private_function()`");
        }

        // 使用 `pub(in path)` 语法声明的函数只在给定的路径中可见。
        // `path` 必须是父模块或祖先模块
        pub(in crate::rust_by_example::examples::test_mod::my_mod) fn public_function_in_my_mod() {
            print!("调用了 `my_mod::nested::public_function_in_my_mod()`，它\n> ");
            public_function_in_nested();
        }

        // 使用 `pub(self)` 语法声明的函数只在当前模块中可见，
        // 这与将它们保持为私有是一样的
        pub(self) fn public_function_in_nested() {
            println!("调用了 `my_mod::nested::public_function_in_nested()`");
        }

        // 使用 `pub(super)` 语法声明的函数仅在父模块中可见

        pub(super) fn public_function_in_super_mod() {
            println!("调用了 `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("调用了 `my_mod::call_public_function_in_my_mod()`，它\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) 使函数仅在当前 crate 内可见
    pub(crate) fn public_function_in_crate() {
        println!("调用了 `my_mod::public_function_in_crate()`");
    }

    // 嵌套模块遵循相同的可见性规则
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("调用了 `my_mod::private_nested::function()`");
        }

        // 私有父项仍会限制子项的可见性，
        // 即使子项被声明为在更大范围内可见。
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("调用了 `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("调用了 `function()`");
}

pub fn test() {
    // 模块允许消除同名项之间的歧义。
    function();
    my_mod::function();

    // 公有项（包括嵌套模块内的）可以从父模块外部访问。
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) 项可以在同一个 crate 的任何地方调用
    my_mod::public_function_in_crate();

    // pub(in path) 项只能在指定的模块内调用
    // 错误！函数 `public_function_in_my_mod` 是私有的
    //my_mod::nested::public_function_in_my_mod();
    // TODO ^ 尝试取消此行注释

    // 模块的私有项不能被直接访问，即使它们嵌套在公有模块中：

    // 错误！`private_function` 是私有的
    //my_mod::private_function();
    // TODO ^ 尝试取消此行注释

    // 错误！`private_function` 是私有的
    //my_mod::nested::private_function();
    // TODO ^ 尝试取消此行注释

    // 错误！`private_nested` 是一个私有模块
    //my_mod::private_nested::function();
    // TODO ^ 尝试取消此行注释

    // 错误！`private_nested` 是一个私有模块
    // my_mod::private_nested::restricted_function();
    // TODO ^ 尝试取消此行注释
}
