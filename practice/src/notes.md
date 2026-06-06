## 使用New 

❌ 不需要 new 的情况
直接在创建时写字段就行：

```rs
// 1. 同一模块内使用
struct Package { id: u32, name: String }
let p = Package { id: 1, name: "张三".to_string() };

// 2. 所有字段都公开（pub）
pub struct Point { pub x: i32, pub y: i32 }
let p = Point { x: 10, y: 20 };
```


✅ 必须用 new 的情况
只有一种情况必须：跨模块使用 + 字段私有
```rs
mod secret {
    pub struct Wallet {
        balance: u32,  // 私有字段
    }
    
    impl Wallet {
        pub fn new(balance: u32) -> Self {  // 必须提供公开构造方法
            Wallet { balance }
        }
    }
}

// 外部模块无法直接构造，只能用 new
let w = secret::Wallet::new(100);  // ✅
// let w = secret::Wallet { balance: 100 };  // ❌ 编译错误
```

💡 推荐用 new 的情况（非必须，但推荐）
字段多，手动写太麻烦

需要默认值（如 current_load: 0）

需要在创建时验证参数

想让 API 更简洁友好

一句话记忆法
只有字段私有且要给别人用时，才必须写 new；其他情况都可以不写，但写了更舒服。



## 代码中的 Rust 语法点总结 🦀

---

### 1. **枚举与带字段的枚举**

pub enum PackageStatus {
    Received,                                          // 无字段
    Sorting { belt_id: u32 },                         // 结构体风格
    Assigned { courier_name: String },                // 结构体风格
    InTransit { courier_name: String, eta_hours: u32 }, // 多字段
    Delivered,
    Failed { reason: String },
}

---

### 2. **模式匹配的多种用法**

#### 基础匹配

match &self.status {
    PackageStatus::Received => "已入库".to_string(),
    PackageStatus::Sorting { belt_id } => format!("分拣中(传送带{})", belt_id),
    // ...
}

#### `if let` 简洁匹配

if let PackageSize::Large = package.size {
    self.accepts_large
}

if let Some(courier) = self.find_courier_mut(courier_name) {
    courier.remove_load();
}

#### `matches!` 宏

if !matches!(package.status, PackageStatus::Received)  // 只关心变体，不关心数据
if matches!(p.status, PackageStatus::Failed { .. })    // .. 忽略所有字段

---

### 3. **`Result` 与 `Option` 的优雅处理**

#### `ok_or_else` 转换

let package = self.packages.iter_mut()
    .find(|p| p.id == package_id)
    .ok_or_else(|| format!("未找到编号 #{} 的包裹", package_id))?;

#### `?` 操作符

courier.add_load()?;  // 自动传播错误

---

### 4. **迭代器与闭包**

// 查找
self.packages.iter().find(|p| p.id == package_id)

// 可变查找
self.packages.iter_mut().find(|p| p.id == package_id)

// 过滤收集
self.packages.iter()
    .filter(|p| matches!(p.status, PackageStatus::Failed { .. }))
    .collect()

// 遍历统计
for package in &self.packages {
    match package.status { ... }
}

---

### 5. **Trait 实现**

#### `Display` trait

impl fmt::Display for PackageSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackageSize::Small => write!(f, "小件"),
            // ...
        }
    }
}

#### 自定义 Trait 与默认实现

trait Dispatch {
    fn receive(&mut self, package: Package) -> Result<(), String>;
    // 默认实现
    fn failed_packages(&self) -> Vec<&Package> {
        self.query_all().into_iter()
            .filter(|p| matches!(p.status, PackageStatus::Failed { .. }))
            .collect()
    }
}

---

### 6. **格式化输出**

// 补零到4位
write!(f, "[#{:04}]", self.id)  // 42 -> "0042"

// 多个参数
write!(f, "{} | {} | {}", name, address, status)

// 构建字符串
format!("包裹 #{} 已分配至传送带 {}", package_id, belt_id)

---

### 7. **HashSet 的使用**

use std::collections::HashSet;

active_belts: HashSet<u32>,           // 声明
self.active_belts.contains(&belt_id)  // 检查存在
self.active_belts.insert(belt_id);    // 插入
self.active_belts.remove(&belt_id);   // 删除

---

### 8. **生命周期与引用**

#### 不可变引用
fn query(&self, package_id: u32) -> Option<&Package>

#### 可变引用
fn find_courier_mut(&mut self, name: &str) -> Option<&mut Courier>

#### 返回引用的生命周期（省略规则）
fn failed_packages(&self) -> Vec<&Package>  // 生命周期自动推断为 &self 的

---

### 9. **结构体方法定义**

#### 关联函数（静态方法）
impl Courier {
    pub fn new(name: &str, max_capacity: u32, accepts_large: bool) -> Self { ... }
}

#### 实例方法
impl Courier {
    fn has_capacity(&self) -> bool { ... }           // 只读
    fn add_load(&mut self) -> Result<(), String> { ... }  // 可修改
}

---

### 10. **字符串处理**

// &str -> String
name.to_string()
"hello".to_string()

// 格式化构建
format!("错误: {}", reason)

// 克隆
courier_name.clone()

---

### 11. **`unwrap` 与错误处理**

warehouse.receive(p1).unwrap();  // 测试用，失败会 panic

// 生产代码应该用
warehouse.receive(p1)?;  // 或 match 处理

---

## 语法点速查表

| 语法点 | 示例 |
|--------|------|
| 带字段枚举 | Sorting { belt_id: u32 } |
| matches! 宏 | matches!(status, Received) |
| if let | if let PackageSize::Large = ... |
| ok_or_else | .ok_or_else(|| format!())? |
| 迭代器链 | .iter().filter().collect() |
| Display trait | impl fmt::Display for ... |
| HashSet | active_belts.contains() |
| 格式化补零 | #{:04} |
| ? 操作符 | courier.add_load()? |
| 默认 trait 方法 | fn failed_packages(&self) { ... } |

---

以上都是 **Rust 开发中最常用的语法点**，掌握它们就能应对大部分实际项目！🎯


## {..}用法


```rs
// 只关心是 Failed，不关心里面的 reason 是什么
if matches!(package.status, PackageStatus::Failed { .. }) {
    println!("派送失败");
}

// 匹配 InTransit，忽略所有字段
if matches!(package.status, PackageStatus::InTransit { .. }) {
    println!("派送中");
}

// 匹配 Sorting，不管 belt_id 是多少
match package.status {
    PackageStatus::Sorting { .. } => println!("正在分拣"),
    PackageStatus::Delivered => println!("已签收"),
    _ => println!("其他状态"),
}


/////////////////////////
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let p1 = Point { x: 1, y: 2, z: 3 };

// 基于 p1 创建 p2，只修改 x，其他字段用 p1 的
let p2 = Point { x: 10, ..p1 };  // p2: x=10, y=2, z=3

// ..p1 表示"剩下的字段从 p1 复制"


////////////////////

let tuple = (1, 2, 3, 4, 5);

// 匹配前两个，忽略后面的
match tuple {
    (a, b, ..) => println!("a={}, b={}", a, b),
}

// 匹配第一个和最后一个
match tuple {
    (a, .., z) => println!("a={}, z={}", a, z),
}

// 匹配中间某个位置
match tuple {
    (.., x, y) => println!("最后两个: {}, {}", x, y),
}

```


场景	语法	含义
枚举匹配	Failed { .. }	匹配 Failed，忽略所有字段
结构体更新	Point { x: 10, ..p1 }	基于 p1 复制其余字段
元组匹配	(a, b, ..)	匹配前两个，忽略后面的
解构结构体	Person { name, .. }	只取 name，忽略其他
闭包参数	|_, y| y + 1	忽略第一个参数
match 通配	Err(..)	匹配 Err，不关心内部
