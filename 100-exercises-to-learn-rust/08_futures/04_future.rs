// 🔑 要点：Future 是惰性的——需要 poll 才能推进
// Rc<T> 不是 Send，不能在跨 await 的范围内持有
// 解决方案：在 yield_now 前 drop Rc，或用块限制作用域
// Requires: tokio (with "full" features)
//! TODO: get the code to compile by **re-ordering** the statements
//!  in the `example` function. You're not allowed to change the
//!  `spawner` function nor what each line does in `example`.
//!   You can wrap existing statements in blocks `{}` if needed.
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    let non_send = Rc::new(1);
    yield_now().await;
    println!("{}", non_send);
}
