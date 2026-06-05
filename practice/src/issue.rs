use std::fmt;

// ── enum：状态机，变体可携带数据 ──────────────────────────
#[derive(Debug, Clone, PartialEq)]
enum Status {
    Open,
    InProgress { assignee: String }, // 带命名字段
    Closed(Resolution),              // 带元组数据
}

#[derive(Debug, Clone, PartialEq)]
enum Resolution {
    Fixed,
    WontFix,
    Duplicate(u32),
} // 嵌套枚举，Duplicate 携带原 ID

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
} // 派生 Ord，可直接比较大小

// ── trait：定义行为契约 ──────────────────────────────────
trait Describable {
    fn one_line(&self) -> String;
    fn is_open(&self) -> bool {
        self.one_line().contains("Open")
    } // 默认实现
}

// ── struct：数据载体 ─────────────────────────────────────
#[derive(Debug)]
struct Issue {
    id: u32,
    title: String,
    priority: Priority,
    status: Status,
}

struct Tracker {
    issues: Vec<Issue>,
    next_id: u32,
}

// ── impl：方法实现 ───────────────────────────────────────
impl Issue {
    fn new(id: u32, title: &str, priority: Priority) -> Self {
        Issue {
            id,
            title: title.into(),
            priority,
            status: Status::Open,
        }
    }

    // 状态流转：返回 Result，非法转换给出错误信息
    fn assign(&mut self, to: &str) -> Result<(), String> {
        match &self.status {
            Status::Open => {
                self.status = Status::InProgress {
                    assignee: to.into(),
                };
                Ok(())
            }
            other => Err(format!("#{} 当前是 {other:?}，无法分配", self.id)),
        }
    }

    fn close(&mut self, res: Resolution) -> Result<(), String> {
        match &self.status {
            Status::Closed(_) => Err(format!("#{} 已经关闭了", self.id)),
            _ => {
                self.status = Status::Closed(res);
                Ok(())
            }
        }
    }
}

impl Describable for Issue {
    fn one_line(&self) -> String {
        // 用 match 解构携带数据的 enum 变体
        let status_str = match &self.status {
            Status::Open => "Open".into(),
            Status::InProgress { assignee } => format!("InProgress({assignee})"),
            Status::Closed(Resolution::Duplicate(id)) => format!("Duplicate(#{id})"),
            Status::Closed(res) => format!("Closed({res:?})"),
        };
        format!(
            "#{} [{:?}] {:<35} {}",
            self.id, self.priority, self.title, status_str
        )
    }
}

impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.one_line())
    }
}

impl Tracker {
    fn new() -> Self {
        Tracker {
            issues: vec![],
            next_id: 1,
        }
    }

    fn add(&mut self, title: &str, priority: Priority) -> u32 {
        let id = self.next_id;
        self.issues.push(Issue::new(id, title, priority));
        self.next_id += 1;
        id
    }

    fn get_mut(&mut self, id: u32) -> Option<&mut Issue> {
        self.issues.iter_mut().find(|i| i.id == id)
    }

    // 泛型过滤：接受任意谓词，返回按优先级排序的结果
    fn filter<P: Fn(&Issue) -> bool>(&self, pred: P) -> Vec<&Issue> {
        let mut res: Vec<&Issue> = self.issues.iter().filter(|i| pred(i)).collect();
        res.sort_by(|a, b| b.priority.cmp(&a.priority)); // 高优先级排前面
        res
    }

    // 动态分发：打印任何实现了 Describable 的列表
    fn print_list(label: &str, items: &[&dyn Describable]) {
        println!("\n── {label} ───────────────────────────────────");
        for item in items {
            println!("  {}", item.one_line());
        }
    }
}

// ── main：场景演示 ───────────────────────────────────────
pub fn issue_test() {
    let mut t = Tracker::new();

    let id1 = t.add("登录页在 Safari 崩溃", Priority::Critical);
    let id2 = t.add("支持深色模式", Priority::High);
    let id3 = t.add("README 缺少安装说明", Priority::Medium);
    let id4 = t.add("暗黑主题（重复提交）", Priority::Low);

    // 状态流转
    t.get_mut(id1).unwrap().assign("alice").unwrap();
    t.get_mut(id1).unwrap().close(Resolution::Fixed).unwrap();
    t.get_mut(id2).unwrap().assign("bob").unwrap();
    t.get_mut(id3).unwrap().close(Resolution::WontFix).unwrap();
    t.get_mut(id4)
        .unwrap()
        .close(Resolution::Duplicate(id2))
        .unwrap();

    // 非法状态流转：演示 Result 错误处理
    let err = t.get_mut(id1).unwrap().assign("charlie").unwrap_err();
    println!("⚠️  非法操作：{err}");

    // 动态分发：Vec<&dyn Describable> 混合展示
    let all: Vec<&dyn Describable> = t.issues.iter().map(|i| i as &dyn Describable).collect();
    Tracker::print_list("全部 Issues", &all);

    // 泛型过滤 + matches! 宏：未关闭按优先级排序
    let open = t.filter(|i| !matches!(i.status, Status::Closed(_)));
    let open_dyn: Vec<&dyn Describable> = open.iter().map(|i| *i as &dyn Describable).collect();
    Tracker::print_list("未关闭（按优先级）", &open_dyn);
}
