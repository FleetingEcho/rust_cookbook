// ============================================================
// Command Pattern — 将操作封装成对象，支持撤销/重做
// 对比 TS: 11_command.ts
// 运行: cargo run --bin command
// ============================================================

trait Command {
    fn execute(&mut self, text: &mut String);
    fn undo(&mut self, text: &mut String);
    fn name(&self) -> &str;
}

struct Insert { content: String, snapshot: String }
struct Replace { from: String, to: String, snapshot: String }
struct UpperCase { snapshot: String }

impl Insert {
    fn new(content: &str) -> Self { Self { content: content.into(), snapshot: String::new() } }
}

impl Command for Insert {
    fn name(&self) -> &str { "Insert" }
    fn execute(&mut self, text: &mut String) {
        self.snapshot = text.clone();
        text.push_str(&self.content);
    }
    fn undo(&mut self, text: &mut String) { *text = self.snapshot.clone(); }
}

impl Replace {
    fn new(from: &str, to: &str) -> Self {
        Self { from: from.into(), to: to.into(), snapshot: String::new() }
    }
}

impl Command for Replace {
    fn name(&self) -> &str { "Replace" }
    fn execute(&mut self, text: &mut String) {
        self.snapshot = text.clone();
        *text = text.replace(&self.from, &self.to);
    }
    fn undo(&mut self, text: &mut String) { *text = self.snapshot.clone(); }
}

impl UpperCase {
    fn new() -> Self { Self { snapshot: String::new() } }
}

impl Command for UpperCase {
    fn name(&self) -> &str { "UpperCase" }
    fn execute(&mut self, text: &mut String) {
        self.snapshot = text.clone();
        *text = text.to_uppercase();
    }
    fn undo(&mut self, text: &mut String) { *text = self.snapshot.clone(); }
}

struct Editor {
    text: String,
    history: Vec<Box<dyn Command>>,
}

impl Editor {
    fn new() -> Self { Self { text: String::new(), history: Vec::new() } }

    fn exec(&mut self, mut cmd: Box<dyn Command>) {
        cmd.execute(&mut self.text);
        println!("[{}] -> {:?}", cmd.name(), self.text);
        self.history.push(cmd);
    }

    fn undo(&mut self) {
        if let Some(mut cmd) = self.history.pop() {
            cmd.undo(&mut self.text);
            println!("[Undo {}] -> {:?}", cmd.name(), self.text);
        } else {
            println!("[Undo] 没有可撤销的操作");
        }
    }
}

fn main() {
    println!("=== Command Pattern ===\n");

    let mut editor = Editor::new();

    println!("--- 执行命令 ---");
    editor.exec(Box::new(Insert::new("hello world")));
    editor.exec(Box::new(Replace::new("world", "rust")));
    editor.exec(Box::new(UpperCase::new()));

    println!("\n--- 撤销 ---");
    editor.undo();
    editor.undo();
    editor.undo();
    editor.undo(); // 没有更多了
}
