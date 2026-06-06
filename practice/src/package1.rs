use std::fmt;

#[derive(Debug, PartialEq)]
enum PackageStatus {
    Received,
    InTransit { courier: String },
    Delivered,
    Failed { reason: String },
}

struct Package {
    id: u32,
    recipient: String,
    status: PackageStatus,
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match &self.status {
            PackageStatus::Received => "已入库".to_string(),
            PackageStatus::InTransit { courier } => format!("派送中({})", courier),
            PackageStatus::Delivered => "已签收".to_string(),
            PackageStatus::Failed { reason } => format!("派送失败({})", reason),
        };
        write!(f, "[#{:03}] {} | 状态: {}", self.id, self.recipient, status_str)
    }
}

trait Dispatch {
    fn assign(&mut self, id: u32, courier: &str) -> Result<(), String>;
    fn deliver(&mut self, id: u32) -> Result<(), String>;
    fn fail(&mut self, id: u32, reason: &str) -> Result<(), String>;
}

impl Dispatch for Vec<Package> {
    fn assign(&mut self, id: u32, courier: &str) -> Result<(), String> {
        for package in self.iter_mut() {
            if package.id == id {
                match package.status {
                    PackageStatus::Received => {
                        package.status = PackageStatus::InTransit {
                            courier: courier.to_string(),
                        };
                        return Ok(());
                    }
                    _ => {
                        return Err(format!(
                            "包裹 #{} 当前状态无法分配快递员（状态：{}）",
                            id,
                            match &package.status {
                                PackageStatus::Received => "已入库",
                                PackageStatus::InTransit { .. } => "派送中",
                                PackageStatus::Delivered => "已签收",
                                PackageStatus::Failed { .. } => "派送失败",
                            }
                        ));
                    }
                }
            }
        }
        Err(format!("未找到 ID 为 {} 的包裹", id))
    }

    fn deliver(&mut self, id: u32) -> Result<(), String> {
        for package in self.iter_mut() {
            if package.id == id {
                match package.status {
                    PackageStatus::InTransit { .. } => {
                        package.status = PackageStatus::Delivered;
                        return Ok(());
                    }
                    _ => {
                        return Err(format!(
                            "包裹 #{} 当前状态无法标记为已签收（状态：{}）",
                            id,
                            match &package.status {
                                PackageStatus::Received => "已入库",
                                PackageStatus::InTransit { .. } => "派送中",
                                PackageStatus::Delivered => "已签收",
                                PackageStatus::Failed { .. } => "派送失败",
                            }
                        ));
                    }
                }
            }
        }
        Err(format!("未找到 ID 为 {} 的包裹", id))
    }

    fn fail(&mut self, id: u32, reason: &str) -> Result<(), String> {
        for package in self.iter_mut() {
            if package.id == id {
                match package.status {
                    PackageStatus::InTransit { .. } => {
                        package.status = PackageStatus::Failed {
                            reason: reason.to_string(),
                        };
                        return Ok(());
                    }
                    _ => {
                        return Err(format!(
                            "包裹 #{} 当前状态无法标记为派送失败（状态：{}）",
                            id,
                            match &package.status {
                                PackageStatus::Received => "已入库",
                                PackageStatus::InTransit { .. } => "派送中",
                                PackageStatus::Delivered => "已签收",
                                PackageStatus::Failed { .. } => "派送失败",
                            }
                        ));
                    }
                }
            }
        }
        Err(format!("未找到 ID 为 {} 的包裹", id))
    }
}

fn main() {
    let mut packages = vec![
        Package {
            id: 1,
            recipient: "张伟".to_string(),
            status: PackageStatus::Received,
        },
        Package {
            id: 2,
            recipient: "李芳".to_string(),
            status: PackageStatus::Received,
        },
        Package {
            id: 3,
            recipient: "王强".to_string(),
            status: PackageStatus::Received,
        },
    ];

    println!("=== 初始包裹状态 ===");
    for pkg in &packages {
        println!("{}", pkg);
    }

    println!("\n=== 正常流程：包裹 #1 派送签收 ===");
    match packages.assign(1, "李师傅") {
        Ok(_) => println!("包裹 #1 分配快递员成功"),
        Err(e) => println!("错误：{}", e),
    }
    println!("{}", packages[0]);

    match packages.deliver(1) {
        Ok(_) => println!("包裹 #1 签收成功"),
        Err(e) => println!("错误：{}", e),
    }
    println!("{}", packages[0]);

    println!("\n=== 派送失败：包裹 #2 ===");
    match packages.assign(2, "王师傅") {
        Ok(_) => println!("包裹 #2 分配快递员成功"),
        Err(e) => println!("错误：{}", e),
    }
    println!("{}", packages[1]);

    match packages.fail(2, "地址错误") {
        Ok(_) => println!("包裹 #2 派送失败标记成功"),
        Err(e) => println!("错误：{}", e),
    }
    println!("{}", packages[1]);

    println!("\n=== 非法操作：对已签收的包裹 #1 调用 assign ===");
    match packages.assign(1, "赵师傅") {
        Ok(_) => println!("包裹 #1 分配快递员成功"),
        Err(e) => println!("错误：{}", e),
    }

    println!("\n=== 最终所有包裹状态 ===");
    for pkg in &packages {
        println!("{}", pkg);
    }
}