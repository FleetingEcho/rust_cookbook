// 运行命令：cargo run -p learning_notes --example rts_raii_drop
//
// ============================================================
// TypeScript 版本（注释掉）：
// ============================================================
// // RAII = Resource Acquisition Is Initialization
// // 资源获取即初始化：对象的生命周期 = 资源持有期
// //
// // TS/JS 没有 RAII 概念，资源管理依赖 GC 或手动 try/finally：
//
// // 文件操作
// let fileHandle: FileHandle | null = null;
// try {
//     fileHandle = fs.openSync("data.txt");
//     const content = fs.readFileSync(fileHandle);
//     // 使用内容...
// } finally {
//     if (fileHandle) fs.closeSync(fileHandle);  // 必须手动清理
// }
//
// // 锁操作
// const lock = new Mutex();
// lock.acquire();
// try {
//     // 临界区...
// } finally {
//     lock.release();  // 必须手动释放
// }
//
// // 计时器
// const timer = setInterval(() => {}, 1000);
// // 必须手动调用 clearInterval(timer)
//
// // GC 的局限性：GC 只管理内存，不管理文件句柄、锁、网络连接等 OS 资源
// // JS 的 WeakRef / FinalizationRegistry 可以监听对象被 GC，但不保证执行时机
// ============================================================

use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn main() {
    // ============================================================
    // 一、RAII 核心思想
    // TS 程序员需要理解的思维转变：
    //
    // TypeScript:   new File() → 手动 close()
    //               lock() → 手动 unlock()
    //               忘记释放 = 资源泄漏
    //
    // Rust (RAII):  File::open() → 离开作用域自动 close()
    //               lock() → 作用域结束自动 unlock()
    //               忘记释放 = 编译错误（不可能的）
    // ============================================================

    println!("=== RAII 核心思想 ===");
    println!("TS: 你负责 acquire → ... → release（try/finally）");
    println!("Rust: 你负责 acquire → ... 编译器负责 release");
    println!();

    // ============================================================
    // 二、RAII 在标准库中的应用
    // ============================================================

    // 1. Mutex 锁（最经典的 RAII 示例）
    // TS: lock(); try { ... } finally { unlock(); }
    println!("--- Mutex 锁（RAII）---");
    let counter = Arc::new(Mutex::new(0_i32));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // lock() 返回 MutexGuard，离开此作用域自动释放锁！
            let mut guard = c.lock().unwrap();
            *guard += 1;
            // guard 在这里被 drop，锁自动释放
            // TS 需要 try/finally 手动释放
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("counter = {}", *counter.lock().unwrap()); // 10

    // 2. 显式看看 RAII 的效果
    {
        let mut guard = counter.lock().unwrap();
        *guard += 1;
        println!("锁已获取，guard 还在作用域内");
        // 这里 guard 还活着，锁未释放
    } // guard 离开作用域 → 锁自动释放
    println!("锁已自动释放\n");

    // 3. 文件操作（模拟，演示 Drop）
    println!("--- 文件写入（RAII）---");
    {
        // File::create 返回 File，它实现了 Drop
        let mut file = File::create(Path::new("/tmp/raii_demo.txt"))
            .expect("创建文件失败（在 Windows 上可能需要调整路径）");
        file.write_all(b"Hello from Rust RAII!").expect("写入失败");
        println!("文件已写入，但尚未关闭");
        // 文件在这里自动 flush + close
    } // file.drop() 被自动调用 → 文件关闭
    println!("文件已自动关闭\n");

    // 4. 即使在错误路径上，RAII 也保证清理
    println!("--- 错误路径上的 RAII ---");
    fn might_fail(should_fail: bool) -> Result<(), String> {
        let mut file =
            File::create("/tmp/raii_error_demo.txt").map_err(|e| format!("创建文件失败: {e}"))?;
        file.write_all(b"some data")
            .map_err(|e| format!("写入失败: {e}"))?;

        if should_fail {
            // 即使提前 return Err，file 也会被正确关闭
            return Err(String::from("模拟错误"));
        }
        // 正常路径：file 在这里关闭
        Ok(())
    }

    match might_fail(true) {
        Ok(_) => println!("成功"),
        Err(e) => println!("错误（文件已自动关闭）: {e}"),
    }
    // 无论成功还是失败，文件句柄都不会泄漏！

    // ============================================================
    // 三、自定义 Drop 实现
    // TS 没有对应的析构函数概念
    // JS class 没有析构函数，FinalizationRegistry 不可靠
    // ============================================================

    println!("\n=== 自定义 Drop ===");

    struct Timer {
        name: &'static str,
        start: Instant,
    }

    impl Timer {
        fn new(name: &'static str) -> Self {
            println!(">>> Timer '{name}' 启动");
            Timer {
                name,
                start: Instant::now(),
            }
        }
    }

    impl Drop for Timer {
        fn drop(&mut self) {
            let elapsed = self.start.elapsed();
            println!("<<< Timer '{}' 停止，耗时: {:?}", self.name, elapsed);
            // 这里自动记录耗时，TS 需要 try/finally 确保输出
        }
    }

    // 使用 Timer
    fn do_work() {
        let _timer = Timer::new("工作计时器");
        // 模拟一些工作
        let mut sum = 0_i64;
        for i in 0..1_000_000 {
            sum += i;
        }
        println!("计算结果: {sum}");
        // _timer 在这里自动 drop，打印耗时
    }

    do_work();

    // ============================================================
    // 四、数据库连接模拟
    // ============================================================
    println!("\n--- 数据库连接（模拟）---");

    struct DbConnection {
        id: u32,
        connected: bool,
    }

    impl DbConnection {
        fn connect(id: u32) -> Self {
            println!(">>> DB #{id}: 连接已建立");
            DbConnection {
                id,
                connected: true,
            }
        }

        fn query(&self, sql: &str) {
            if !self.connected {
                panic!("连接已关闭！");
            }
            println!("DB #{}: 执行查询: {sql}", self.id);
        }
    }

    impl Drop for DbConnection {
        fn drop(&mut self) {
            if self.connected {
                println!("<<< DB #{}: 连接已关闭", self.id);
                self.connected = false;
            }
        }
    }

    // 正常路径：连接自动关闭
    {
        let db = DbConnection::connect(1);
        db.query("SELECT * FROM users");
    } // db.drop() → "DB #1: 连接已关闭"

    // 提前返回：连接也自动关闭
    fn get_user(db_id: u32) -> Result<String, String> {
        let db = DbConnection::connect(db_id);
        db.query("SELECT name FROM users WHERE id = 1");

        if db_id == 0 {
            return Err(String::from("无效 DB ID"));
        }

        Ok(String::from("Alice"))
    }

    match get_user(0) {
        Ok(name) => println!("用户: {name}"),
        Err(e) => println!("错误: {e}（但连接已自动关闭）"),
    }

    // ============================================================
    // 五、std::mem::drop（提前释放）
    // TS 没有直接对应，但在作用域结束前可以手动释放
    // 一个用途：释放锁后允许后续代码不持锁
    // ============================================================
    println!("\n--- std::mem::drop（提前释放）---");

    let lock_obj = Arc::new(Mutex::new(vec![1, 2, 3]));

    {
        let guard = lock_obj.lock().unwrap();
        println!("持有锁，准备做一些快速操作...");
        // 做完必要的操作后，提前释放锁
        drop(guard); // 显式释放锁，不需要等作用域结束
        println!("锁已提前释放，其他线程可以继续");

        // 这里可以做不需要锁的工作...
        println!("做一些不需要锁的工作...");
    } // guard 已 drop，不会重复释放

    // TS 没有 RAII 等价写法：
    // const lock = mutex.lock();
    // try { ... } finally { lock.unlock(); }
    // Rust 自动做 finally 的工作

    // ============================================================
    // 六、Rc 和 Arc 的引用计数（也是 Drop 的应用）
    // TS 的 GC 在后台运行，Rust 的 Rc/Arc 在 drop 时立即调整计数
    // ============================================================
    println!("\n--- Rc 引用计数 ---");

    use std::rc::Rc;
    {
        let a = Rc::new(String::from("hello"));
        println!("引用计数: {}", Rc::strong_count(&a)); // 1
        {
            let _b = Rc::clone(&a);
            println!("引用计数: {}", Rc::strong_count(&a)); // 2
        } // _b.drop() → 计数减1
        println!("引用计数: {}", Rc::strong_count(&a)); // 1

        // 当 a 离开作用域时，计数变为 0，内存立即释放
        // TS 的 GC 无法精确控制释放时机
    } // a.drop() → 计数0 → 内存释放

    // ============================================================
    // 七、作用域守卫（Scope Guard）模式
    // 这是 Rust 中常见的 RAII 应用：在离开作用域时执行某操作
    // TS 需要 try/finally 或 defer 库
    // ============================================================
    println!("\n--- 作用域守卫模式 ---");

    struct Defer<F: FnMut()> {
        f: F,
    }

    impl<F: FnMut()> Drop for Defer<F> {
        fn drop(&mut self) {
            (self.f)(); // 离开作用域时执行闭包
        }
    }

    fn defer<F: FnMut()>(f: F) -> Defer<F> {
        Defer { f }
    }

    // 使用：类似 TS 的 try/finally
    {
        let _guard = defer(|| println!(">> 清理操作：关闭文件"));
        let _guard2 = defer(|| println!(">> 清理操作：释放锁"));
        println!("主逻辑执行中...");
        if true {
            // return 也会触发 defer！
            println!("提前返回，defer 仍然会执行");
        }
    } // 两个 defer 在这里反向执行（类似 TS finally 栈）
      // 输出顺序：
      //   主逻辑执行中...
      //   >> 清理操作：释放锁
      //   >> 清理操作：关闭文件

    // ============================================================
    // 八、RAII 在 Rust 生态中的常见应用
    // ============================================================
    println!("\n--- RAII 的生态应用 ---");

    println!("1. std::fs::File — 文件自动关闭");
    println!("2. std::sync::Mutex — 锁自动释放");
    println!("3. std::sync::RwLockReadGuard / RwLockWriteGuard — 读写锁");
    println!("4. std::io::BufWriter — 自动 flush");
    println!("5. tracing/log 的 span — 进入/离开日志");
    println!("6. sqlx::Transaction — 事务自动提交/回滚");
    println!("7. tempfile::TempDir — 临时目录自动删除");
    println!("8. scopeguard crate — defer! 宏");

    // ============================================================
    // 总结对照表
    // ============================================================
    println!("\n=== Rust vs TS 资源管理总结 ===");
    println!("┌───────────────────────────────────┬─────────────────────────────────────┐");
    println!("│ TypeScript                        │ Rust                                │");
    println!("├───────────────────────────────────┼─────────────────────────────────────┤");
    println!("│ GC 管理内存                       │ 所有权系统 + Drop 管理所有资源      │");
    println!("│ 手动 try/finally 释放非内存资源   │ 自动释放（RAII + Drop trait）       │");
    println!("│ 可能忘记 close/unlock 导致泄漏     │ 编译器确保释放（无法忘记）         │");
    println!("│ 清理代码在 try 块的 finally 中     │ 清理代码在资源的 Drop::drop() 中    │");
    println!("│ setTimeout/clearTimeout 手动清除   │ Timer drop 时自动取消               │");
    println!("│ FinalizationRegistry 不可靠        │ Drop 确定性执行                    │");
    println!("│ 作用域结束不自动触发清理           │ 离开作用域 = 触发所有 Drop         │");
    println!("│ 错误路径容易忘记 finally           │ 错误路径的 Drop 同样执行            │");
    println!("│ 未定义变量的释放时机               │ 变量离开作用域立即释放             │");
    println!("└───────────────────────────────────┴─────────────────────────────────────┘");
}
