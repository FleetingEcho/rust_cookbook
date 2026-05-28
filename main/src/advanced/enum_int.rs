/*
从整数到枚举的转换有时还是非常需要的，例如你有一个枚举类型，然后需要从外面传入一个整数，用于控制后续的流程走向，此时就需要用整数去匹配相应的枚举

*/

use num_enum::TryFromPrimitive;

#[derive(Debug, TryFromPrimitive)]
#[repr(i32)]
enum MyEnum {
    A = 1,
    B = 2,
    C = 3,
}

fn main() {
    let a = MyEnum::try_from(1);
    let b = MyEnum::try_from(2);
    let c = MyEnum::try_from(3);
    let d = MyEnum::try_from(4); // ❌ 无法匹配，返回 Err(())

    println!("{:?}, {:?}, {:?}", a, b, c); // Ok(A), Ok(B), Ok(C)
    println!("{:?}", d); // Err(())
}

//或者使用macro_rules, 但是没必要



/*
当你知道数值一定不会超过枚举的范围时(例如枚举成员对应 1，2，3，传入的整数也在这个范围内)，就可以使用这个方法完成变形。

#[repr(i32)]
enum MyEnum {
    A = 1, B, C
}

fn main() {
    let x = MyEnum::C;
    let y = x as i32;
    let z: MyEnum = unsafe { std::mem::transmute(y) };

    // match the enum that came from an int
    match z {
        MyEnum::A => { println!("Found A"); }
        MyEnum::B => { println!("Found B"); }
        MyEnum::C => { println!("Found C"); }
    }
}
*/
