# Rust 格式化输出

## 1. 基本格式化

Rust 使用 `println!` 宏进行格式化输出，与 C 语言的 `printf` 类似，但更加类型安全。使用 `{}` 作为占位符，参数按位置顺序填入。

```rust
fn main() {
    println!("Hello, world!");
    println!("name: {}, age: {}", "Alice", 30);
}
```

## 2. 位置参数

通过索引可以重用前面的参数，避免重复书写。

```rust
fn main() {
    println!("{0} is {1} years old", "Alice", 30);
    println!("{1} {0}", "Alice", 30);
}
```

## 3. 命名参数

使用命名参数可以提高可读性，尤其当参数较多时。

```rust
fn main() {
    println!("{name} is {age} years old", name = "Alice", age = 30);
}
```

## 4. 格式修饰符

### 4.1 填充与对齐

可以使用 `{:>width}` 右对齐、`{:<width}` 左对齐、`{:>width$}` 动态宽度等。

```rust
fn main() {
    println!("{:>5}", 5);
    println!("{:>5}", 10);
    println!("{:>5}", 100);
}
```

**运行结果：**

```
    5
   10
  100
```

### 4.2 数值格式化

支持不同进制的输出，`{:#x}` 输出十六进制并加 `0x` 前缀，`{:#o}` 输出八进制加 `0o` 前缀。

```rust
fn main() {
    println!("{:#x}", 255);
    println!("{:#o}", 64);
}
```

**运行结果：**

```
0xff
0o100
```

### 4.3 浮点数精度

使用 `:.N` 控制小数位数。

```rust
fn main() {
    println!("{:.2}", 3.1415926);
}
```

**运行结果：**

```
3.14
```

## 5. Debug 格式化

`{:?}` 用于实现 `Debug` 类型的值，`{:#?}` 提供带缩进的美化输出。

```rust
fn main() {
    let point = (3, 4);
    println!("{:?}", point);
    println!("{:#?}", point);
}
```

---

## 📘 TypeScript 对比

**Rust：**

```rust
println!("Hello, {}", name);
```

**TypeScript：**

```ts
console.log(`Hello, ${name}`);
```

| 特性 | Rust | TypeScript |
|------|------|-----------|
| 模板字符串 | `format!("{} {}", a, b)` | `` `${a} ${b}` `` |
| 占位符 | `{}`, `{0}`, `{name}` | `${expr}` |
| 对齐 | `{:>10}`, `{:>width$}` | `padStart()` / `padEnd()` |
| Debug | `{:?}` / `{:#?}` | `util.inspect()` |
| 性能 | 编译期检查参数数量 | 运行时拼接 |
