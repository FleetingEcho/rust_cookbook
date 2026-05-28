/*
Rust 的 Sized 和不定长类型（DST）
在 Rust 中，类型可以分为两大类：

定长类型 (Sized)：大小在编译时确定，例如 i32、Vec<T>、String 等。
不定长类型 (DST，Dynamically Sized Type)：大小只有在运行时才能确定，例如 str、切片 [T] 和特征对象 dyn Trait。



1. 动态大小类型（DST）
特点：

无法直接使用，必须通过引用或智能指针（如 Box<T>）间接使用。
Rust 需要在编译时知道变量的大小，否则无法分配内存。

(1) str（动态字符串类型）
你可能熟悉 &str，但 str 本身是 DST，无法直接创建：
// ❌ 无法编译
let s: str = "Hello there!";
为什么 &str 是 Sized？

&str 只是一个 指针，存储在 栈上，指向 堆上的 str 数据。
这个指针包含 数据地址 和 长度，因此编译器可以推断其大小。
示例：


fn main() {
    let s: &str = "Hello there!";
    println!("{}", s);
}

(2) 切片 [T]
数组的 切片（如 [T]）也是 DST，因为它的长度是运行时确定的：


fn main() {
    let array: [i32; 3] = [1, 2, 3];
    let slice: &[i32] = &array[..]; // ✅ `&[T]` 是 `Sized`
    println!("{:?}", slice);
}
slice 存储在栈上的引用，指向 堆上的数据，所以 &[T] 是 Sized。
但无法直接创建 DST 类型：


fn main() {
    let arr: [i32] = [1, 2, 3]; // ❌ 编译错误
}
错误原因：编译器需要在编译期知道数组的大小，而 [T] 是不定长的。

(3) 特征对象 dyn Trait
特征对象（如 dyn MyTrait）也是 DST，不能直接使用：

trait MyTrait {
    fn foo(&self);
}

// ❌ 直接使用 `MyTrait` 会报错
fn foobar_3(thing: MyTrait) {}

fn main() {}
如何使用？

通过 &dyn Trait 或 Box<dyn Trait>：

fn foobar_1(thing: &dyn MyTrait) {}     // ✅ 引用
fn foobar_2(thing: Box<dyn MyTrait>) {} // ✅ 智能指针


2. Sized 特征
作用：

Rust 需要确保 泛型默认是 Sized，以便在栈上分配空间。
默认所有类型都实现 Sized，除了 DST（str、[T]、dyn Trait）。
(1) 泛型中的 Sized
默认情况下，泛型 T 被假设为 Sized：


fn generic<T>(t: T) {}
等同于：


fn generic<T: Sized>(t: T) {}
这意味着 T 必须是 编译期已知大小的类型，因此 DST 不能直接传入。

(2) 让泛型支持 DST
如果希望 T 既能是 Sized，也能是 DST，需要使用 ?Sized：


fn generic<T: ?Sized>(t: &T) {
    // ✅ 允许 `T` 是 DST
}
?Sized 允许 T 可能是 DST。
必须通过引用 &T 传递，因为 DST 不能直接在栈上存储。
示例：


fn print_str(s: &str) {
    println!("{}", s);
}

fn print_anything<T: ?Sized>(t: &T) {
    println!("{:?}", std::mem::size_of_val(t));
}

fn main() {
    let s: &str = "Hello";
    print_anything(s); // ✅ 允许 DST
}



3. Box<T> 处理 DST
问题：为什么 Box<dyn Trait> 可以，但 Box<str> 不行？


fn main() {
    let s1: Box<str> = Box::new("Hello there!" as str); // ❌ 编译错误
}
错误原因：

Box<dyn Trait> 只需要知道 可以调用哪些方法
而 Box<str> 需要知道 完整的大小。

Box<str> 缺少 str 的长度信息，导致编译器无法分配正确的内存。

(1) 解决 Box<str>
正确的用法：


fn main() {
    let s1: Box<str> = "Hello there!".into(); // ✅ OK
}
原理：

"Hello there!" 是 &str，带有长度信息。
.into() 允许将 &str 转换成 Box<str>，编译器会自动完成转换。


(2) Box<[T]>（动态数组）

fn main() {
    let arr: Box<[i32]> = vec![1, 2, 3].into();
    println!("{:?}", arr);
}

vec![1, 2, 3] 生成 Vec<i32>，但 Vec 具有固定大小。
.into() 自动转换成 Box<[i32]>，以便在 堆上存储动态大小数组。
总结



类型	是        否 Sized	          说明
i32, Vec<T>, String	✅ 是	固定大小，编译期可确定
str, [T]	❌ 不是	长度不固定，必须使用 &str 或 Box<[T]>
dyn Trait	❌ 不是	必须使用 &dyn Trait 或 Box<dyn Trait>
T: Sized	✅ 是	泛型默认 Sized
T: ?Sized	❓ 可变	允许 T 是 DST，但必须用引用 &T

如何使用 DST？
&str、&[T]、&dyn Trait ✅
Box<str>、Box<[T]> ✅

直接 str、[T]、dyn Trait ❌

Rust 通过 Sized 机制保证编译时大小确定，而 ?Sized 允许更灵活的泛型。使用 Box<T> 处理 DST 时，需要确保 引用携带足够的元数据（如长度）。

*/