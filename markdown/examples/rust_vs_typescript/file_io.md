# Rust vs TypeScript: 文件 IO 与路径操作

**运行命令：** `cargo run -p learning_notes --example rts_file_io`

## TypeScript 版本

```ts
import fs from "fs/promises";
import path from "path";

// 读文件
const content = await fs.readFile("data.txt", "utf-8");

// 写文件
await fs.writeFile("output.txt", "hello", "utf-8");

// 追加
await fs.appendFile("log.txt", "新的一行\n");

// 检查是否存在
await fs.access("file.txt").then(() => true).catch(() => false);

// 读目录
const entries = await fs.readdir("./src");

// 递归创建目录
await fs.mkdir("a/b/c", { recursive: true });

// 删除文件
await fs.unlink("temp.txt");

// 路径操作
const full = path.join(__dirname, "data", "file.txt");
const ext  = path.extname("file.txt");   // ".txt"
const base = path.basename("a/b/c.txt"); // "c.txt"
const dir  = path.dirname("a/b/c.txt");  // "a/b"
```

## Rust 文件 IO vs TS 的关键差异

1. **同步为主**：`std::fs` 全是同步 API；异步版本在 `tokio::fs`（API 几乎一致）
2. **显式错误处理**：每个 IO 操作返回 `Result`，不能忽略
3. **路径类型**：`std::path::Path`（借用）/ `PathBuf`（拥有），类似 `&str` vs `String`
4. **字节优先**：读写文件默认操作 `Vec<u8>`，读文本需要显式 `String::from_utf8`

---

## 一、读文件

```rust
use std::fs;

// 最简单：整个文件读为 String（自动处理 UTF-8）
fn read_text(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

// 读为字节（二进制文件）
fn read_bytes(path: &str) -> Result<Vec<u8>, std::io::Error> {
    fs::read(path)
}

// 逐行读取（大文件时避免一次性加载到内存）
use std::io::{BufRead, BufReader};

fn read_lines(path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()  // collect::<Result<Vec<_>, _>>()
}
```

---

## 二、写文件

```rust
use std::fs;
use std::io::Write;

// 覆盖写入（文件不存在则创建）
fn write_text(path: &str, content: &str) -> Result<(), std::io::Error> {
    fs::write(path, content)
}

// 追加写入
fn append_text(path: &str, content: &str) -> Result<(), std::io::Error> {
    use std::fs::OpenOptions;
    let mut file = OpenOptions::new()
        .create(true)   // 不存在则创建
        .append(true)   // 追加模式
        .open(path)?;
    file.write_all(content.as_bytes())
}

// 分多次写入（BufWriter 减少系统调用次数）
use std::io::BufWriter;

fn write_lines(path: &str, lines: &[&str]) -> Result<(), std::io::Error> {
    let file = fs::File::create(path)?;
    let mut writer = BufWriter::new(file);
    for line in lines {
        writeln!(writer, "{line}")?;
    }
    Ok(())
    // BufWriter 在 drop 时自动 flush
}
```

---

## 三、检查文件是否存在

```rust
use std::path::Path;

// 推荐：用 try_exists，区分"不存在"和"没权限"
fn file_exists(path: &str) -> bool {
    Path::new(path).try_exists().unwrap_or(false)
}

// 获取文件元信息
fn file_info(path: &str) -> Result<(), std::io::Error> {
    let metadata = fs::metadata(path)?;
    println!("大小: {} 字节", metadata.len());
    println!("是文件: {}", metadata.is_file());
    println!("是目录: {}", metadata.is_dir());
    println!("只读: {}", metadata.permissions().readonly());
    Ok(())
}
```

---

## 四、目录操作

```rust
// 创建目录（包含中间目录，相当于 mkdir -p）
fn create_dir(path: &str) -> Result<(), std::io::Error> {
    fs::create_dir_all(path)
}

// 读取目录内容
fn list_dir(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut entries = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        entries.push(name);
    }
    entries.sort();
    Ok(entries)
}

// 递归遍历目录（找所有 .rs 文件）
fn find_rs_files(dir: &str) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    let mut result = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            result.extend(find_rs_files(path.to_str().unwrap())?);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            result.push(path);
        }
    }
    Ok(result)
}

// 删除文件 / 目录
fn remove_file(path: &str) -> Result<(), std::io::Error> {
    fs::remove_file(path)
}

fn remove_dir(path: &str) -> Result<(), std::io::Error> {
    fs::remove_dir_all(path)  // 递归删除，相当于 rm -rf
}
```

---

## 五、路径操作（std::path）

`Path`（借用）和 `PathBuf`（拥有）的关系就像 `&str` 和 `String`。

```rust
use std::path::{Path, PathBuf};

fn path_examples() {
    // 构建路径（自动处理各平台分隔符 / 和 \）
    let path = PathBuf::from("/home/user").join("projects").join("main.rs");
    println!("{}", path.display()); // /home/user/projects/main.rs

    // 拼接（push 修改自身）
    let mut p = PathBuf::from("/tmp");
    p.push("output");
    p.set_extension("json"); // /tmp/output.json

    // 拆解路径
    let path = Path::new("/home/user/projects/main.rs");
    println!("文件名:  {:?}", path.file_name());       // Some("main.rs")
    println!("扩展名:  {:?}", path.extension());        // Some("rs")
    println!("stem:    {:?}", path.file_stem());        // Some("main")
    println!("父目录:  {:?}", path.parent());           // Some("/home/user/projects")
    println!("是绝对:  {}",   path.is_absolute());      // true

    // 转为字符串
    let s: &str = path.to_str().unwrap();               // 如果是有效 UTF-8
    let s: String = path.to_string_lossy().to_string(); // 有损转换，总能成功

    // 获取绝对路径（解析 . 和 ..）
    let abs = Path::new("./src/../src/main.rs").canonicalize();
}
```

---

## 六、临时文件与常用目录

```rust
use std::env;

fn system_dirs() {
    // 系统临时目录（/tmp 或 C:\Users\...\AppData\Local\Temp）
    let tmp = env::temp_dir();
    println!("临时目录: {}", tmp.display());

    // 当前工作目录
    let cwd = env::current_dir().unwrap();
    println!("当前目录: {}", cwd.display());

    // 可执行文件所在目录（常用来定位资源文件）
    let exe_dir = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    println!("程序目录: {}", exe_dir.display());
}

// 创建临时文件（用 tempfile crate 更方便，标准库没有内置）
// [dev-dependencies]
// tempfile = "3"
//
// let tmp = tempfile::NamedTempFile::new()?; // 自动在 drop 时删除
```

---

## 七、异步文件 IO（tokio::fs）

API 与 `std::fs` 几乎一致，只是加了 `.await`：

```rust
use tokio::fs;
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};

async fn async_read(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path).await
}

async fn async_write(path: &str, content: &str) -> Result<(), std::io::Error> {
    fs::write(path, content).await
}

// 逐行异步读取
async fn async_read_lines(path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = fs::File::open(path).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut result = Vec::new();
    while let Some(line) = lines.next_line().await? {
        result.push(line);
    }
    Ok(result)
}
```

> **什么时候用 tokio::fs？** 在 async 上下文（Web 服务器、并发任务）中，文件 IO 操作也应该用异步版本，避免阻塞 tokio 线程池。

---

## TS vs Rust 文件 IO 对照

| 操作 | TypeScript (Node.js) | Rust (同步) | Rust (异步 tokio) |
|------|---------------------|------------|-----------------|
| 读文本 | `fs.readFile(p, 'utf-8')` | `fs::read_to_string(p)?` | `fs::read_to_string(p).await?` |
| 读字节 | `fs.readFile(p)` | `fs::read(p)?` | `fs::read(p).await?` |
| 写文件 | `fs.writeFile(p, data)` | `fs::write(p, data)?` | `fs::write(p, data).await?` |
| 追加 | `fs.appendFile(p, data)` | `OpenOptions::append(true)` | `OpenOptions::append(true)` |
| 是否存在 | `fs.access(p)` | `Path::new(p).try_exists()` | `fs::try_exists(p).await` |
| 读目录 | `fs.readdir(p)` | `fs::read_dir(p)?` | `fs::read_dir(p).await?` |
| 创建目录 | `fs.mkdir(p, {recursive:true})` | `fs::create_dir_all(p)?` | `fs::create_dir_all(p).await?` |
| 删除文件 | `fs.unlink(p)` | `fs::remove_file(p)?` | `fs::remove_file(p).await?` |
| 路径拼接 | `path.join(a, b)` | `Path::new(a).join(b)` | 同步 |
| 文件名 | `path.basename(p)` | `path.file_name()` | 同步 |
| 扩展名 | `path.extname(p)` | `path.extension()` | 同步 |
