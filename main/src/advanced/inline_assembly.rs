// Rust 内联汇编（Inline Assembly）详解
// Rust 提供了 asm! 宏，使开发者可以在 Rust 代码中嵌入 汇编指令，主要用于 高性能优化、底层开发（如操作系统、驱动）等。
// 不过，普通项目几乎不会用到，因为 Rust 本身就有很好的优化能力。

// 本文基于 x86/x86-64 汇编，但 ARM、RISC-V 等架构也是支持的。

// 1. 基本概念
// 内联汇编提供了 Rust 无法直接实现 的底层能力，但使用它会绕过 Rust 的安全检查，需要 unsafe 代码块：

// use std::arch::asm;

// unsafe {
//     asm!("nop"); // 空操作（No Operation）
// }
// nop 指令不会执行任何操作，只是占位，通常用于 性能测试、内存对齐 等场景。

// 1.1 适用场景
// 操作系统内核开发（如 Linux Kernel）
// 驱动程序（如显卡、硬件加速）
// 高性能优化（如 SIMD、AVX 指令集）
// 直接控制 CPU 寄存器
// 与其他语言（如 C）互操作
// 2. 变量输入和输出
// 2.1 赋值
// use std::arch::asm;

// let x: u64;
// unsafe {
//     asm!("mov {}, 5", out(reg) x);
// }
// assert_eq!(x, 5);
// 解释：

// asm!("mov {}, 5", out(reg) x);
// mov 指令把 5 存入 x 变量所在的寄存器。
// {} 是占位符，out(reg) x 告诉 Rust 这个变量是输出（out），并由 asm! 选择合适的 寄存器 来存储 x。
// 2.2 多个变量
// use std::arch::asm;

// let i: u64 = 3;
// let o: u64;
// unsafe {
//     asm!(
//         "mov {0}, {1}", // 把 {1} 复制到 {0}
//         "add {0}, 5",   // {0} + 5
//         out(reg) o,     // 输出 o
//         in(reg) i,      // 输入 i
//     );
// }
// assert_eq!(o, 8);
// 解析：

// in(reg) i：将变量 i 作为输入
// out(reg) o：将 o 作为输出
// 格式化字符串：Rust 允许 {0}, {1}, ... 复用变量
// 2.3 inout：输入同时作为输出
// use std::arch::asm;

// let mut x: u64 = 3;
// unsafe {
//     asm!("add {0}, 5", inout(reg) x);
// }
// assert_eq!(x, 8);
// 解析：

// inout(reg) x 让 x 既是输入，又是输出
// 确保变量在同一个寄存器中处理，避免额外的拷贝
// 2.4 inout 带不同变量
// use std::arch::asm;

// let x: u64 = 3;
// let y: u64;
// unsafe {
//     asm!("add {0}, 5", inout(reg) x => y);
// }
// assert_eq!(y, 8);
// 区别：

// x 作为输入
// y 作为输出（但不会影响 x）
// 3. lateout 优化寄存器使用
// 默认情况下，Rust 不会让多个变量共享寄存器，但在某些情况下，可以手动优化：

// use std::arch::asm;

// let mut a: u64 = 4;
// let b: u64 = 4;
// unsafe {
//     asm!(
//         "add {0}, {1}",
//         inlateout(reg) a, // a 先作为输入，后作为输出
//         in(reg) b,        // b 仅作为输入
//     );
// }
// assert_eq!(a, 8);
// 优化点：

// inlateout 允许 a 和 b 共享寄存器，减少 mov 指令
// a 只在输入全部读取后，才会被修改（避免数据覆盖）
// 4. 指定寄存器
// 某些指令只能使用特定寄存器，如 eax、rbx 等：

// use std::arch::asm;

// let cmd = 0xd1;
// unsafe {
//     asm!("out 0x64, eax", in("eax") cmd);
// }
// 解析：

// in("eax") cmd 强制 cmd 存入 eax
// out 0x64, eax：把 eax 的值写入 0x64 端口
// 5. Clobbered（破坏的寄存器）
// 某些指令 会修改特定寄存器，必须告诉编译器，以避免误用：

// use std::arch::asm;

// fn main() {
//     let mut name_buf = [0_u8; 12]; // 存储 CPU ID

//     unsafe {
//         asm!(
//             "push rbx",    // 保护 rbx
//             "cpuid",       // 读取 CPU ID
//             "mov [rdi], ebx",
//             "mov [rdi + 4], edx",
//             "mov [rdi + 8], ecx",
//             "pop rbx",     // 恢复 rbx
//             in("rdi") name_buf.as_mut_ptr(), // name_buf 作为输入
//             inout("eax") 0 => _, // eax 作为输入/输出（丢弃）
//             out("ecx") _, // 告诉编译器 ecx 会被修改
//             out("edx") _,
//         );
//     }

//     let name = core::str::from_utf8(&name_buf).unwrap();
//     println!("CPU Manufacturer ID: {}", name);
// }
// 解析：

// cpuid 指令会修改 ebx, ecx, edx，所以需要 out("ecx") _ 告诉编译器
// push rbx / pop rbx 保护 rbx
// 结果存入 name_buf
// 6. mul 指令示例
// mul 指令用于乘法，计算 64 位 * 64 位的乘法：

use std::arch::asm;

fn mul(a: u64, b: u64) -> u128 {
    let lo: u64;
    let hi: u64;

    unsafe {
        asm!(
            "mul {}",
            in(reg) a,
            inlateout("rax") b => lo, // 低 64 位
            lateout("rdx") hi,        // 高 64 位
        );
    }

    ((hi as u128) << 64) + lo as u128
}
// 解析：

// mul 指令：
// rax 作为 输入
// 乘法结果：
// rax 存储 低 64 位
// rdx 存储 高 64 位
// lateout("rdx")：确保 rdx 只在计算完成后被修改
// 7. 总结
// 概念	作用	示例
// out(reg)	变量作为 输出	out(reg) x
// in(reg)	变量作为 输入	in(reg) y
// inout(reg)	变量 同时作为输入和输出	inout(reg) x
// lateout(reg)	优化寄存器使用	lateout("rdx") hi
// inlateout(reg)	输入后修改	inlateout("rax") b => lo
// Clobbered	保护寄存器	out("eax") _
// Rust 提供的 内联汇编 功能非常强大，但它的使用需要 极端谨慎，一般只在 操作系统、驱动程序、高性能计算 等领域才会用到。