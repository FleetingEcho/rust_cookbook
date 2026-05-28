/*
1. as 类型转换
as 关键字用于进行基本类型转换，适用于整数、浮点数、字符等。

示例：


fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    if a < (b as i32) {
        println!("Ten is less than one hundred.");
    }
}
注意：

as 不能保证安全转换，例如 300_i32 as i8 会得到 44，因为 i8 只能存储 -128 到 127。

示例：

fn main() {
    let a = 3.1 as i8;    // 浮点数转整数
    let b = 100_i8 as i32; // i8 转 i32
    let c = 'a' as u8;    // 字符转 u8（97）

    println!("{},{},{}", a, b, c);
}



2. 内存地址转换
Rust 允许将指针转换为整数，或反向转换。

示例：


fn main() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize;
    let second_address = first_address + 4; // i32 占 4 字节
    let p2 = second_address as *mut i32;

    unsafe { *p2 += 1; }

    assert_eq!(values[1], 3);
}


3. TryInto 安全转换
当 as 不能确保安全时，可以使用 TryInto，它会返回 Result 以便错误处理。

示例：


use std::convert::TryInto;

fn main() {
    let a: u8 = 10;
    let b: u16 = 1500;

    let b_: u8 = match b.try_into() {
        Ok(value) => value,
        Err(_) => {
            println!("转换失败");
            0
        }
    };

    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}
特点：

TryInto 可捕获溢出错误（如 1500_i16.try_into::<u8>() 会失败）。



4. 结构体类型转换
如果要在结构体之间转换，通常需要手动进行字段映射。

示例：

*/

struct Foo {
    x: u32,
    y: u16,
}

struct Bar {
    a: u32,
    b: u16,
}

fn reinterpret(foo: Foo) -> Bar {
    Bar { a: foo.x, b: foo.y }
}

/*
5. 方法调用中的隐式转换
Rust 允许在方法调用时进行隐式转换，如自动引用、解引用。

示例：


fn do_stuff<T: Clone>(value: &T) {
    let cloned = value.clone();
}
value.clone() 会根据 T 的类型决定是否直接调用 clone。

&T 默认实现 Clone，所以 cloned 的类型为 T。
如果 T 没有 Clone，cloned 变成 &T。



自动索引访问示例：
use std::rc::Rc;

fn main() {
    let array: Rc<Box<[i32; 3]>> = Rc::new(Box::new([1, 2, 3]));
    let first_entry = array[0]; // 经过多层解引用后才能访问
    println!("{}", first_entry);
}


6. transmute（危险转换）
transmute 允许在任意两个大小相同的类型之间转换，是最不安全的类型转换方式。

示例：
use std::mem;

fn main() {
    let x: i32 = 42;
    let y: f32 = unsafe { mem::transmute::<i32, f32>(x) };
    println!("{}", y); // 输出的浮点数是未定义的
}
⚠️ 极度危险，可能导致未定义行为

常见应用：

裸指针转函数指针：

fn foo() -> i32 { 0 }

fn main() {
    let pointer = foo as *const ();
    let function: fn() -> i32 = unsafe { std::mem::transmute(pointer) };
    assert_eq!(function(), 0);
}
延长或缩短生命周期：

struct R<'a>(&'a i32);

// 延长生命周期
unsafe fn extend_lifetime<'b>(r: R<'b>) -> R<'static> {
    std::mem::transmute::<R<'b>, R<'static>>(r)
}

// 缩短生命周期
unsafe fn shorten_lifetime<'b, 'c>(r: &'b mut R<'static>) -> &'b mut R<'c> {
    std::mem::transmute::<&'b mut R<'static>, &'b mut R<'c>>(r)
}


总结
方式	适用范围	安全性
as	基本类型转换	❌ 溢出时不报错
TryInto	数值类型转换	✅ 捕获溢出错误
reinterpret	结构体转换	✅ 安全但啰嗦
隐式转换	方法调用等	✅ 自动处理
transmute	任意类型转换	⚠️ 极度危险
Rust 设计类型转换时优先保证安全性，推荐尽量使用 TryInto，避免 as 和 transmute 造成的潜在问题。
*/