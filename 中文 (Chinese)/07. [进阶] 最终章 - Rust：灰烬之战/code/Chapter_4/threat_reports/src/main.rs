#[derive(Debug)]
enum AttackType {
    PortScan,
    BruteForce,
    DDoS(u32),
    Malware(String),
    Unknown,
}

#[derive(Debug)]
struct ThreatReport {
    ip: Option<String>,
    attack_type: AttackType,
    level: u32,
    timestamp: String,
}

impl ThreatReport {
    fn print(&self) {
        println!("=== 威胁报告 ===");
        match &self.ip {
            Some(ip) => println!("来源: {}", ip),
            None => println!("来源: 未知"),
        }
        match &self.attack_type {
            AttackType::PortScan => println!("攻击类型: 端口扫描"),
            AttackType::BruteForce => println!("攻击类型: 爆破攻击"),
            AttackType::DDoS(peak) => println!("攻击类型: DDoS（峰值 {} Gbps）", peak),
            AttackType::Malware(name) => println!("攻击类型: 恶意软件 ({})", name),
            AttackType::Unknown => println!("攻击类型: 未知"),
        }
        println!("威胁等级: {}", self.level);
        println!("时间戳: {}", self.timestamp);
        println!("=================");
    }

    fn is_high_risk(&self) -> bool {
        self.level >= 4
    }

    fn escalate(&mut self) {
        self.level += 1;
    }
}

fn alert(report: &ThreatReport) {
    match report.attack_type {
        AttackType::PortScan => println!("⚠️ 端口扫描检测！来源: {:?}", report.ip),
        AttackType::BruteForce => println!("🔴 爆破攻击！"),
        AttackType::DDoS(peak) => println!("🔥 DDoS攻击！峰值流量: {} Gbps", peak),
        AttackType::Malware(ref name) => println!("💀 恶意软件: {}", name),
        AttackType::Unknown => println!("❓ 未知攻击类型"),
    }
}

fn main() {
    let mut report1 = ThreatReport {
        ip: Some(String::from("10.0.0.5")),
        attack_type: AttackType::PortScan,
        level: 2,
        timestamp: String::from("2025-01-21T14:32:17"),
    };

    let report2 = ThreatReport {
        ip: None,
        attack_type: AttackType::DDoS(850),
        level: 5,
        timestamp: String::from("2025-01-21T14:33:42"),
    };

    report1.print();
    report2.print();

    alert(&report1);
    alert(&report2);

    println!("\n--- 升级 report1 威胁等级 ---");
    report1.escalate();
    report1.print();

    println!("report1 是否高危: {}", report1.is_high_risk());
    println!("report2 是否高危: {}", report2.is_high_risk());

    println!("\n--- 使用 if let ---");
    if let Some(ip) = &report1.ip {
        println!("report1 的 IP 是: {}", ip);
    }
    if let Some(ip) = &report2.ip {
        println!("report2 的 IP 是: {}", ip);
    } else {
        println!("report2 没有 IP");
    }
}