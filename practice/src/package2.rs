use std::fmt;
use std::collections::HashSet;

// ==================== PackageSize ====================
#[derive(Debug, PartialEq)]
pub enum PackageSize {
    Small,   // ≤1kg
    Medium,  // 1-10kg
    Large,   // >10kg
}

impl fmt::Display for PackageSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackageSize::Small => write!(f, "小件"),
            PackageSize::Medium => write!(f, "中件"),
            PackageSize::Large => write!(f, "大件"),
        }
    }
}

// ==================== PackageStatus ====================
#[derive(Debug, PartialEq)]
pub enum PackageStatus {
    Received,
    Sorting { belt_id: u32 },
    Assigned { courier_name: String },
    InTransit { courier_name: String, eta_hours: u32 },
    Delivered,
    Failed { reason: String },
}

// ==================== DeliveryResult ====================
#[derive(Debug)]
pub enum DeliveryResult {
    Success,
    NoOneHome,
    AddressNotFound,
    Refused,
}

// ==================== Package ====================
#[derive(Debug)]
pub struct Package {
    id: u32,
    recipient: String,
    address: String,
    size: PackageSize,
    status: PackageStatus,
    fragile: bool,
}

impl Package {
    pub fn new(id: u32, recipient: &str, address: &str, size: PackageSize, fragile: bool) -> Self {
        Package {
            id,
            recipient: recipient.to_string(),
            address: address.to_string(),
            size,
            status: PackageStatus::Received,
            fragile,
        }
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match &self.status {
            PackageStatus::Received => "已入库".to_string(),
            PackageStatus::Sorting { belt_id } => format!("分拣中(传送带{})", belt_id),
            PackageStatus::Assigned { courier_name } => format!("已分配({})", courier_name),
            PackageStatus::InTransit { courier_name, eta_hours } => format!("派送中({}, 预计{}小时)", courier_name, eta_hours),
            PackageStatus::Delivered => "已签收".to_string(),
            PackageStatus::Failed { reason } => format!("失败: {}", reason),
        };
        
        let fragile_str = if self.fragile { "是" } else { "否" };
        
        write!(
            f,
            "[#{:04}][{}] {} | 地址: {} | 状态: {} | 易碎: {}",
            self.id, self.size, self.recipient, self.address, status_str, fragile_str
        )
    }
}

// ==================== Courier ====================
#[derive(Debug)]
pub struct Courier {
    name: String,
    max_capacity: u32,
    current_load: u32,
    accepts_large: bool,
}

impl Courier {
    pub fn new(name: &str, max_capacity: u32, accepts_large: bool) -> Self {
        Courier {
            name: name.to_string(),
            max_capacity,
            current_load: 0,
            accepts_large,
        }
    }
    
    fn has_capacity(&self) -> bool {
        self.current_load < self.max_capacity
    }
    
    fn can_carry(&self, package: &Package) -> bool {
        if let PackageSize::Large = package.size {
            self.accepts_large
        } else {
            true
        }
    }
    
    fn add_load(&mut self) -> Result<(), String> {
        if self.has_capacity() {
            self.current_load += 1;
            Ok(())
        } else {
            Err(format!("{} 已达到最大负载量", self.name))
        }
    }
    
    fn remove_load(&mut self) {
        if self.current_load > 0 {
            self.current_load -= 1;
        }
    }
}

impl fmt::Display for Courier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let accept_str = if self.accepts_large { "是" } else { "否" };
        write!(
            f,
            "[快递员] {} | 负载: {}/{} | 接受大件: {}",
            self.name, self.current_load, self.max_capacity, accept_str
        )
    }
}

// ==================== Warehouse ====================
pub struct Warehouse {
    name: String,
    packages: Vec<Package>,
    couriers: Vec<Courier>,
    belt_count: u32,
    active_belts: HashSet<u32>, // 正在使用的传送带
}

impl Warehouse {
    pub fn new(name: &str, belt_count: u32) -> Self {
        Warehouse {
            name: name.to_string(),
            packages: Vec::new(),
            couriers: Vec::new(),
            belt_count,
            active_belts: HashSet::new(),
        }
    }
    
    pub fn add_courier(&mut self, courier: Courier) {
        self.couriers.push(courier);
    }
    
    fn find_free_belt(&self) -> Option<u32> {
        for belt_id in 1..=self.belt_count {
            if !self.active_belts.contains(&belt_id) {
                return Some(belt_id);
            }
        }
        None
    }
    
    fn find_courier(&self, name: &str) -> Option<&Courier> {
        self.couriers.iter().find(|c| c.name == name)
    }
    
    fn find_courier_mut(&mut self, name: &str) -> Option<&mut Courier> {
        self.couriers.iter_mut().find(|c| c.name == name)
    }
}

// ==================== Dispatch Trait ====================
pub trait Dispatch {
    fn receive(&mut self, package: Package) -> Result<(), String>;
    fn sort(&mut self, package_id: u32) -> Result<(), String>;
    fn assign(&mut self, package_id: u32, courier_name: &str) -> Result<(), String>;
    fn dispatch(&mut self, package_id: u32, eta_hours: u32) -> Result<(), String>;
    fn deliver(&mut self, package_id: u32, result: DeliveryResult) -> Result<(), String>;
    fn query(&self, package_id: u32) -> Option<&Package>;
    
    fn failed_packages(&self) -> Vec<&Package> {
        self.query_all().into_iter()
            .filter(|p| matches!(p.status, PackageStatus::Failed { .. }))
            .collect()
    }
    
    // Helper method for failed_packages default implementation
    fn query_all(&self) -> Vec<&Package>;
}

impl Dispatch for Warehouse {
    fn receive(&mut self, package: Package) -> Result<(), String> {
        // 检查编号是否重复
        if self.packages.iter().any(|p| p.id == package.id) {
            return Err(format!("包裹编号 #{} 已存在", package.id));
        }
        
        println!("✅ 入库成功: {}", package);
        self.packages.push(package);
        Ok(())
    }
    
    fn sort(&mut self, package_id: u32) -> Result<(), String> {
        let package = self.packages.iter_mut()
            .find(|p| p.id == package_id)
            .ok_or_else(|| format!("未找到编号 #{} 的包裹", package_id))?;
        
        // 检查状态是否允许分拣
        if !matches!(package.status, PackageStatus::Received) {
            return Err(format!("包裹 #{} 当前状态无法分拣", package_id));
        }
        
        // 查找空闲传送带
        let belt_id = self.find_free_belt()
            .ok_or_else(|| format!("所有传送带都在使用中 (共{}条)", self.belt_count))?;
        
        // 更新状态
        package.status = PackageStatus::Sorting { belt_id };
        self.active_belts.insert(belt_id);
        
        println!("✅ 分拣成功: 包裹 #{} 已分配至传送带 {}", package_id, belt_id);
        Ok(())
    }
    
    fn assign(&mut self, package_id: u32, courier_name: &str) -> Result<(), String> {
        // 检查包裹是否存在且状态正确
        let package = self.packages.iter_mut()
            .find(|p| p.id == package_id)
            .ok_or_else(|| format!("未找到编号 #{} 的包裹", package_id))?;
        
        if !matches!(package.status, PackageStatus::Sorting { .. }) {
            return Err(format!("包裹 #{} 当前状态无法分配", package_id));
        }
        
        // 检查快递员是否存在
        let courier = self.find_courier(courier_name)
            .ok_or_else(|| format!("未找到快递员: {}", courier_name))?;
        
        // 检查快递员容量
        if !courier.has_capacity() {
            return Err(format!("快递员 {} 已满载", courier_name));
        }
        
        // 检查是否接受大件
        if !courier.can_carry(package) {
            return Err(format!("快递员 {} 不接受大件包裹", courier_name));
        }
        
        // 分配包裹
        let belt_id = match package.status {
            PackageStatus::Sorting { belt_id } => belt_id,
            _ => unreachable!(),
        };
        
        package.status = PackageStatus::Assigned { courier_name: courier_name.to_string() };
        self.active_belts.remove(&belt_id);
        
        // 增加快递员负载
        let courier = self.find_courier_mut(courier_name).unwrap();
        courier.add_load()?;
        
        println!("✅ 分配成功: 包裹 #{} 已分配给 {}", package_id, courier_name);
        Ok(())
    }
    
    fn dispatch(&mut self, package_id: u32, eta_hours: u32) -> Result<(), String> {
        let package = self.packages.iter_mut()
            .find(|p| p.id == package_id)
            .ok_or_else(|| format!("未找到编号 #{} 的包裹", package_id))?;
        
        let courier_name = match &package.status {
            PackageStatus::Assigned { courier_name } => courier_name.clone(),
            _ => return Err(format!("包裹 #{} 当前状态无法派送", package_id)),
        };
        
        package.status = PackageStatus::InTransit { courier_name, eta_hours };
        
        println!("✅ 派送成功: 包裹 #{} 已发出，预计{}小时内送达", package_id, eta_hours);
        Ok(())
    }
    
    fn deliver(&mut self, package_id: u32, result: DeliveryResult) -> Result<(), String> {
        let package = self.packages.iter_mut()
            .find(|p| p.id == package_id)
            .ok_or_else(|| format!("未找到编号 #{} 的包裹", package_id))?;
        
        // 获取快递员名称（用于减少负载）
        let courier_name = match &package.status {
            PackageStatus::InTransit { courier_name, .. } => courier_name.clone(),
            _ => return Err(format!("包裹 #{} 当前状态无法签收", package_id)),
        };
        
        // 更新包裹状态
        match result {
            DeliveryResult::Success => {
                package.status = PackageStatus::Delivered;
                println!("✅ 签收成功: 包裹 #{} 已送达", package_id);
            }
            DeliveryResult::NoOneHome => {
                package.status = PackageStatus::Failed { reason: "无人接收，已转存代收点".to_string() };
                println!("❌ 派送失败: 包裹 #{} 无人接收", package_id);
            }
            DeliveryResult::AddressNotFound => {
                package.status = PackageStatus::Failed { reason: "地址有误，无法送达".to_string() };
                println!("❌ 派送失败: 包裹 #{} 地址有误", package_id);
            }
            DeliveryResult::Refused => {
                package.status = PackageStatus::Failed { reason: "收件人拒收".to_string() };
                println!("❌ 派送失败: 包裹 #{} 被拒收", package_id);
            }
        }
        
        // 减少快递员负载
        if let Some(courier) = self.find_courier_mut(&courier_name) {
            courier.remove_load();
        }
        
        Ok(())
    }
    
    fn query(&self, package_id: u32) -> Option<&Package> {
        self.packages.iter().find(|p| p.id == package_id)
    }
    
    fn query_all(&self) -> Vec<&Package> {
        self.packages.iter().collect()
    }
}

impl fmt::Display for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 统计各状态包裹数量
        let mut received_count = 0;
        let mut sorting_count = 0;
        let mut assigned_count = 0;
        let mut intransit_count = 0;
        let mut delivered_count = 0;
        let mut failed_count = 0;
        
        for package in &self.packages {
            match package.status {
                PackageStatus::Received => received_count += 1,
                PackageStatus::Sorting { .. } => sorting_count += 1,
                PackageStatus::Assigned { .. } => assigned_count += 1,
                PackageStatus::InTransit { .. } => intransit_count += 1,
                PackageStatus::Delivered => delivered_count += 1,
                PackageStatus::Failed { .. } => failed_count += 1,
            }
        }
        
        writeln!(f, "\n🏭 {} 仓库报告", self.name)?;
        writeln!(f, "📦 包裹总数: {}", self.packages.len())?;
        writeln!(f, "   状态统计:")?;
        writeln!(f, "   - 已入库: {}", received_count)?;
        writeln!(f, "   - 分拣中: {}", sorting_count)?;
        writeln!(f, "   - 已分配: {}", assigned_count)?;
        writeln!(f, "   - 派送中: {}", intransit_count)?;
        writeln!(f, "   - 已签收: {}", delivered_count)?;
        writeln!(f, "   - 失败: {}", failed_count)?;
        writeln!(f, "\n🚚 快递员列表:")?;
        for courier in &self.couriers {
            writeln!(f, "   {}", courier)?;
        }
        Ok(())
    }
}

// ==================== main 函数 ====================
fn main() {
    println!("========== 快递仓库分拣系统 ==========\n");
    
    // 1. 创建仓库
    let mut warehouse = Warehouse::new("北京东城配送中心", 2);
    
    // 添加快递员
    let courier1 = Courier::new("李师傅", 3, true);
    let courier2 = Courier::new("王师傅", 2, false); // 不接受大件
    warehouse.add_courier(courier1);
    warehouse.add_courier(courier2);
    
    println!("✅ 仓库初始化完成");
    println!("{}", warehouse);
    
    // 2. 入库包裹
    println!("\n========== 开始入库 ==========");
    
    let p1 = Package::new(1001, "张伟", "北京市朝阳区xx路1号", PackageSize::Medium, true);
    let p2 = Package::new(1002, "李芳", "北京市海淀区yy路2号", PackageSize::Small, false);
    let p3 = Package::new(1003, "王强", "北京市东城区zz路3号", PackageSize::Large, false);
    let p4 = Package::new(1004, "刘娜", "北京市西城区ww路4号", PackageSize::Medium, true);
    
    warehouse.receive(p1).unwrap();
    warehouse.receive(p2).unwrap();
    warehouse.receive(p3).unwrap();
    warehouse.receive(p4).unwrap();
    
    // 3. 正常派送流程（包裹 1001）
    println!("\n========== 正常派送流程 (包裹 #1001) ==========");
    warehouse.sort(1001).unwrap();
    warehouse.assign(1001, "李师傅").unwrap();
    warehouse.dispatch(1001, 2).unwrap();
    warehouse.deliver(1001, DeliveryResult::Success).unwrap();
    
    // 4. 非法操作演示
    println!("\n========== 非法操作演示 ==========");
    
    // 对已分拣的包裹再次调用 sort()
    println!("\n❌ 测试: 对已分拣的包裹 #1002 再次调用 sort()");
    warehouse.sort(1002).unwrap(); // 第一次成功
    match warehouse.sort(1002) {
        Ok(_) => println!("意外成功"),
        Err(e) => println!("❌ 预期错误: {}", e),
    }
    
    // 将大件分配给不接受大件的快递员
    println!("\n❌ 测试: 将大件包裹 #1003 分配给王师傅（不接受大件）");
    warehouse.sort(1003).unwrap(); // 先分拣
    match warehouse.assign(1003, "王师傅") {
        Ok(_) => println!("意外成功"),
        Err(e) => println!("❌ 预期错误: {}", e),
    }
    
    // 5. 传送带满载演示
    println!("\n========== 传送带满载演示 ==========");
    println!("当前传送带数量: 2");
    
    // 分拣包裹 1002 和 1003（1001已完成，1004未分拣）
    println!("\n分拣包裹 #1002...");
    warehouse.sort(1002).unwrap(); // 已在上面分拣过，但状态是Sorting
    
    println!("\n分拣包裹 #1003...");
    warehouse.sort(1003).unwrap();
    
    println!("\n尝试分拣包裹 #1004（第3个）...");
    match warehouse.sort(1004) {
        Ok(_) => println!("意外成功"),
        Err(e) => println!("❌ 预期错误: {}", e),
    }
    
    // 6. 失败包裹演示
    println!("\n========== 失败包裹演示 ==========");
    
    // 创建一个新包裹测试失败场景
    let p5 = Package::new(1005, "赵明", "上海市浦东新区vv路5号", PackageSize::Small, false);
    warehouse.receive(p5).unwrap();
    warehouse.sort(1005).unwrap();
    warehouse.assign(1005, "李师傅").unwrap();
    warehouse.dispatch(1005, 3).unwrap();
    warehouse.deliver(1005, DeliveryResult::AddressNotFound).unwrap(); // 地址错误，派送失败
    
    let p6 = Package::new(1006, "周敏", "深圳市南山区ww路6号", PackageSize::Medium, false);
    warehouse.receive(p6).unwrap();
    warehouse.sort(1006).unwrap();
    warehouse.assign(1006, "李师傅").unwrap();
    warehouse.dispatch(1006, 2).unwrap();
    warehouse.deliver(1006, DeliveryResult::Refused).unwrap(); // 拒收，派送失败
    
    // 7. 打印失败包裹汇总
    println!("\n========== 失败包裹汇总 ==========");
    let failed = warehouse.failed_packages();
    if failed.is_empty() {
        println!("暂无失败包裹");
    } else {
        println!("共有 {} 个失败包裹:", failed.len());
        for package in failed {
            println!("  {}", package);
        }
    }
    
    // 打印最终仓库报告
    println!("\n{}", warehouse);
    
    // 打印所有包裹状态
    println!("\n========== 所有包裹详情 ==========");
    for package in warehouse.query_all() {
        println!("{}", package);
    }
}