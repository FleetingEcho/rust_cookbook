// ============================================================
// Command Pattern — 将操作封装成对象，支持撤销/重做
// 对比 Rust: 11_command.rs
// 运行: npx ts-node 11_command.ts
// ============================================================

interface Command {
  execute(text: string): string;
  undo(text: string): string;
  readonly name: string;
}

class Insert implements Command {
  readonly name = "Insert";
  private snapshot = "";
  constructor(private content: string) {}
  execute(text: string): string { this.snapshot = text; return text + this.content; }
  undo(_: string): string       { return this.snapshot; }
}

class Replace implements Command {
  readonly name = "Replace";
  private snapshot = "";
  constructor(private from: string, private to: string) {}
  execute(text: string): string { this.snapshot = text; return text.replaceAll(this.from, this.to); }
  undo(_: string): string       { return this.snapshot; }
}

class UpperCase implements Command {
  readonly name = "UpperCase";
  private snapshot = "";
  execute(text: string): string { this.snapshot = text; return text.toUpperCase(); }
  undo(_: string): string       { return this.snapshot; }
}

class Editor {
  private text = "";
  private history: Command[] = [];

  exec(cmd: Command) {
    this.text = cmd.execute(this.text);
    console.log(`[${cmd.name}] -> ${JSON.stringify(this.text)}`);
    this.history.push(cmd);
  }

  undo() {
    const cmd = this.history.pop();
    if (!cmd) { console.log("[Undo] 没有可撤销的操作"); return; }
    this.text = cmd.undo(this.text);
    console.log(`[Undo ${cmd.name}] -> ${JSON.stringify(this.text)}`);
  }
}

// --- main ---
console.log("=== Command Pattern ===\n");

const editor = new Editor();

console.log("--- 执行命令 ---");
editor.exec(new Insert("hello world"));
editor.exec(new Replace("world", "typescript"));
editor.exec(new UpperCase());

console.log("\n--- 撤销 ---");
editor.undo();
editor.undo();
editor.undo();
editor.undo();
