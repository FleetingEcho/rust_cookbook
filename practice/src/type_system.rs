use std::collections::HashMap;
use std::fmt;

// ══════════════════════════════════════════════
// 1. enum：表达"有限状态集合"
// ══════════════════════════════════════════════

/// Issue 的优先级
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Priority::Low      => "🟢 低",
            Priority::Medium   => "🟡 中",
            Priority::High     => "🟠 高",
            Priority::Critical => "🔴 紧急",
        };
        write!(f, "{s}")
    }
}

/// Issue 的当前状态（状态机）
#[derive(Debug, Clone, PartialEq)]
enum Status {
    Open,
    InProgress { assignee: String },    // 携带数据：谁在处理
    InReview   { pr_url: String },      // 携带数据：关联 PR
    Closed     { resolution: Resolution },
}

/// 关闭原因
#[derive(Debug, Clone, PartialEq)]
enum Resolution {
    Fixed,
    WontFix,
    Duplicate(u32),   // 携带数据：重复的是哪个 issue ID
    Invalid,
}

impl fmt::Display for Resolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Resolution::Fixed          => write!(f, "已修复"),
            Resolution::WontFix        => write!(f, "不予处理"),
            Resolution::Duplicate(id)  => write!(f, "重复 #{id}"),
            Resolution::Invalid        => write!(f, "无效"),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Open                        => write!(f, "[ 待处理 ]"),
            Status::InProgress { assignee }     => write!(f, "[ 处理中 → {assignee} ]"),
            Status::InReview   { pr_url }       => write!(f, "[ 审查中 → {pr_url} ]"),
            Status::Closed     { resolution }   => write!(f, "[ 已关闭：{resolution} ]"),
        }
    }
}

/// Issue 的类型标签
#[derive(Debug, Clone, PartialEq)]
enum Label {
    Bug,
    Feature,
    Docs,
    Performance,
    Security,
    Custom(String),   // 自定义标签
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Label::Bug         => write!(f, "#bug"),
            Label::Feature     => write!(f, "#feature"),
            Label::Docs        => write!(f, "#docs"),
            Label::Performance => write!(f, "#perf"),
            Label::Security    => write!(f, "#security"),
            Label::Custom(s)   => write!(f, "#{s}"),
        }
    }
}

// ══════════════════════════════════════════════
// 2. trait：定义行为契约
// ══════════════════════════════════════════════

/// 所有可以"被汇总展示"的对象都实现这个 trait
trait Summarize {
    fn summary(&self) -> String;
}

/// 支持状态流转的对象
trait Stateful {
    /// 返回当前状态描述
    fn current_status(&self) -> String;
    /// 是否已关闭（默认实现）
    fn is_closed(&self) -> bool {
        self.current_status().contains("已关闭")
    }
}

/// 可以被过滤/搜索的对象
trait Searchable {
    fn matches_query(&self, query: &str) -> bool;
}

// ══════════════════════════════════════════════
// 3. struct：定义数据结构
// ══════════════════════════════════════════════

/// 单条评论
#[derive(Debug, Clone)]
struct Comment {
    author: String,
    body: String,
    timestamp: String,  // 简化：用字符串表示时间
}

/// 核心数据：一个 Issue
#[derive(Debug, Clone)]
struct Issue {
    id:       u32,
    title:    String,
    body:     String,
    priority: Priority,
    status:   Status,
    labels:   Vec<Label>,
    author:   String,
    comments: Vec<Comment>,
}

/// Issue Tracker 本体
struct Tracker {
    issues:    HashMap<u32, Issue>,
    next_id:   u32,
    /// 事件日志：记录所有变更历史
    event_log: Vec<String>,
}

// ══════════════════════════════════════════════
// 4. impl：实现方法
// ══════════════════════════════════════════════

impl Comment {
    fn new(author: &str, body: &str, timestamp: &str) -> Self {
        Comment {
            author:    author.to_string(),
            body:      body.to_string(),
            timestamp: timestamp.to_string(),
        }
    }
}

impl Issue {
    /// 构造新 issue（Builder 风格的链式调用）
    fn new(id: u32, author: &str, title: &str, body: &str, priority: Priority) -> Self {
        Issue {
            id,
            title:    title.to_string(),
            body:     body.to_string(),
            priority,
            status:   Status::Open,
            labels:   vec![],
            author:   author.to_string(),
            comments: vec![],
        }
    }

    /// 链式添加标签
    fn with_label(mut self, label: Label) -> Self {
        self.labels.push(label);
        self
    }

    /// 状态流转：分配给某人处理
    fn assign(&mut self, assignee: &str) -> Result<(), String> {
        match &self.status {
            Status::Open => {
                self.status = Status::InProgress { assignee: assignee.to_string() };
                Ok(())
            }
            other => Err(format!("无法从 {other} 状态分配，必须为 Open")),
        }
    }

    /// 状态流转：提交 PR 进入 Review
    fn submit_for_review(&mut self, pr_url: &str) -> Result<(), String> {
        match &self.status {
            Status::InProgress { .. } => {
                self.status = Status::InReview { pr_url: pr_url.to_string() };
                Ok(())
            }
            other => Err(format!("无法从 {other} 状态提交 Review")),
        }
    }

    /// 状态流转：关闭 issue
    fn close(&mut self, resolution: Resolution) -> Result<(), String> {
        if self.status == (Status::Closed { resolution: resolution.clone() }) {
            return Err("Issue 已经是关闭状态".to_string());
        }
        self.status = Status::Closed { resolution };
        Ok(())
    }

    /// 添加评论
    fn add_comment(&mut self, comment: Comment) {
        self.comments.push(comment);
    }
}

// 为 Issue 实现各种 trait
impl Summarize for Issue {
    fn summary(&self) -> String {
        let labels: Vec<String> = self.labels.iter().map(|l| l.to_string()).collect();
        let label_str = if labels.is_empty() {
            String::new()
        } else {
            format!("  {}", labels.join(" "))
        };
        format!(
            "#{:<4} [{:>4}] {:40} {} {}{}",
            self.id,
            self.priority.to_string(),
            self.title,
            self.status,
            self.author,
            label_str,
        )
    }
}

impl Stateful for Issue {
    fn current_status(&self) -> String {
        self.status.to_string()
    }
}

impl Searchable for Issue {
    fn matches_query(&self, query: &str) -> bool {
        let q = query.to_lowercase();
        self.title.to_lowercase().contains(&q)
            || self.body.to_lowercase().contains(&q)
            || self.author.to_lowercase().contains(&q)
            || self.labels.iter().any(|l| l.to_string().contains(&q))
    }
}

impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "┌─────────────────────────────────────────")?;
        writeln!(f, "│ Issue #{} — {}", self.id, self.title)?;
        writeln!(f, "│ 作者：{}　优先级：{}　状态：{}", self.author, self.priority, self.status)?;
        if !self.labels.is_empty() {
            let ls: Vec<_> = self.labels.iter().map(|l| l.to_string()).collect();
            writeln!(f, "│ 标签：{}", ls.join(", "))?;
        }
        writeln!(f, "│")?;
        writeln!(f, "│ {}", self.body)?;
        if !self.comments.is_empty() {
            writeln!(f, "│")?;
            writeln!(f, "│ 💬 评论（{}条）：", self.comments.len())?;
            for c in &self.comments {
                writeln!(f, "│   [{}] {}: {}", c.timestamp, c.author, c.body)?;
            }
        }
        write!(f, "└─────────────────────────────────────────")
    }
}

// Tracker 的实现
impl Tracker {
    fn new() -> Self {
        Tracker {
            issues:    HashMap::new(),
            next_id:   1,
            event_log: vec![],
        }
    }

    /// 创建并登记 issue，返回分配的 ID
    fn create_issue(&mut self, issue: Issue) -> u32 {
        let id = issue.id;
        self.event_log.push(format!("CREATED  #{id} 「{}」by {}", issue.title, issue.author));
        self.issues.insert(id, issue);
        self.next_id = id + 1;
        id
    }

    /// 获取可变引用，执行闭包操作（统一记录日志）
    fn update_issue<F>(&mut self, id: u32, action: &str, f: F) -> Result<(), String>
    where
        F: FnOnce(&mut Issue) -> Result<(), String>,
    {
        let issue = self.issues.get_mut(&id).ok_or(format!("Issue #{id} 不存在"))?;
        f(issue)?;
        self.event_log.push(format!("{action:8} #{id} 「{}」", issue.title));
        Ok(())
    }

    /// 泛型过滤：接受任何谓词
    fn filter<P>(&self, predicate: P) -> Vec<&Issue>
    where
        P: Fn(&Issue) -> bool,
    {
        let mut results: Vec<&Issue> = self.issues.values().filter(|i| predicate(i)).collect();
        // 按优先级降序排列
        results.sort_by(|a, b| b.priority.cmp(&a.priority));
        results
    }

    /// 全文搜索（使用 Searchable trait 动态分发的平替：这里用 trait method）
    fn search(&self, query: &str) -> Vec<&Issue> {
        self.filter(|issue| issue.matches_query(query))
    }

    /// 统计报表
    fn report(&self) {
        let total   = self.issues.len();
        let open    = self.filter(|i| i.status == Status::Open).len();
        let closed  = self.filter(|i| matches!(i.status, Status::Closed { .. })).len();
        let in_prog = self.filter(|i| matches!(i.status, Status::InProgress { .. })).len();

        println!("\n📊 项目统计");
        println!("  总计: {total}  待处理: {open}  处理中: {in_prog}  已关闭: {closed}");

        // 按优先级分组统计
        for p in [Priority::Critical, Priority::High, Priority::Medium, Priority::Low] {
            let count = self.filter(|i| i.priority == p && i.status != (Status::Closed {
                resolution: Resolution::Fixed  // 只是占位，用 matches! 更好
            })).len();
            // 用 matches! 更精准
            let open_count = self.issues.values()
                .filter(|i| i.priority == p && !matches!(i.status, Status::Closed { .. }))
                .count();
            println!("  {p}: {open_count} 个未关闭");
        }
    }

    /// 打印事件日志
    fn print_log(&self) {
        println!("\n📋 变更历史");
        for entry in &self.event_log {
            println!("  {entry}");
        }
    }
}

// ══════════════════════════════════════════════
// 5. 泛型工具函数：接受 dyn Summarize
// ══════════════════════════════════════════════

/// 打印任意实现了 Summarize 的列表（动态分发）
fn print_list(title: &str, items: &[&dyn Summarize]) {
    println!("\n── {title} ──────────────────────────");
    if items.is_empty() {
        println!("  （空）");
    } else {
        for item in items {
            println!("  {}", item.summary());
        }
    }
}

// ══════════════════════════════════════════════
// 6. main：场景演示
// ══════════════════════════════════════════════

fn main() {
    let mut tracker = Tracker::new();

    // ── 创建 Issues ──────────────────────────
    let id1 = tracker.create_issue(
        Issue::new(1, "alice", "登录页面在 Safari 上崩溃", "点击登录按钮后白屏，复现率 100%", Priority::Critical)
            .with_label(Label::Bug)
            .with_label(Label::Security),
    );

    let id2 = tracker.create_issue(
        Issue::new(2, "bob", "支持深色模式", "用户反馈强烈，需支持 prefers-color-scheme", Priority::High)
            .with_label(Label::Feature),
    );

    let id3 = tracker.create_issue(
        Issue::new(3, "alice", "README 缺少安装说明", "新人不知道如何本地启动项目", Priority::Medium)
            .with_label(Label::Docs),
    );

    let id4 = tracker.create_issue(
        Issue::new(4, "charlie", "首页加载超过 5 秒", "Lighthouse 评分 32，需优化图片和 JS bundle", Priority::High)
            .with_label(Label::Performance)
            .with_label(Label::Custom("sprint-3".to_string())),
    );

    let id5 = tracker.create_issue(
        Issue::new(5, "bob", "升级 webpack 版本", "当前版本有安全漏洞 CVE-2024-XXXX", Priority::Critical)
            .with_label(Label::Security),
    );

    // ── 状态流转演示 ──────────────────────────
    tracker.update_issue(id1, "ASSIGNED", |issue| {
        issue.assign("charlie")
    }).unwrap();

    tracker.update_issue(id1, "REVIEWED", |issue| {
        issue.submit_for_review("https://github.com/org/repo/pull/42")
    }).unwrap();

    tracker.update_issue(id1, "CLOSED  ", |issue| {
        issue.add_comment(Comment::new("charlie", "根本原因：Safari 不支持可选链操作符，已 polyfill", "2024-06-01 14:32"));
        issue.close(Resolution::Fixed)
    }).unwrap();

    tracker.update_issue(id3, "CLOSED  ", |issue| {
        issue.close(Resolution::Fixed)
    }).unwrap();

    // 演示 Duplicate 关闭
    tracker.create_issue(
        Issue::new(6, "dave", "暗黑主题", "和 #2 重复了", Priority::Low)
            .with_label(Label::Feature),
    );
    tracker.update_issue(6, "CLOSED  ", |issue| {
        issue.close(Resolution::Duplicate(2))
    }).unwrap();

    tracker.update_issue(id4, "ASSIGNED", |issue| {
        issue.assign("alice")
    }).unwrap();

    // ── 全部 issue 列表 ──────────────────────
    let all: Vec<&dyn Summarize> = tracker.issues.values()
        .collect::<Vec<_>>()  // 先收集 &Issue
        .into_iter()
        .map(|i| i as &dyn Summarize)
        .collect();
    print_list("所有 Issues", &all);

    // ── 过滤：只看未关闭的 ───────────────────
    let open_issues = tracker.filter(|i| !matches!(i.status, Status::Closed { .. }));
    let open_display: Vec<&dyn Summarize> = open_issues.iter().map(|i| *i as &dyn Summarize).collect();
    print_list("未关闭的 Issues（按优先级排序）", &open_display);

    // ── 搜索 ─────────────────────────────────
    let results = tracker.search("安全");
    println!("\n🔍 搜索「安全」：");
    for issue in &results {
        println!("  {}", issue.summary());
    }

    // ── 查看某个 issue 的详情 ─────────────────
    println!("\n{}", tracker.issues[&id1]);

    // ── 统计报表 ──────────────────────────────
    tracker.report();

    // ── 事件日志 ──────────────────────────────
    tracker.print_log();

    // ── 演示错误处理（非法状态流转）────────────
    println!("\n⚠️  尝试非法状态流转：");
    let err = tracker.update_issue(id1, "ASSIGN  ", |issue| {
        issue.assign("dave")  // 已关闭的 issue 不能再分配
    });
    println!("  结果：{}", err.unwrap_err());
}