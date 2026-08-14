use std::fs;
use std::io::{self, BufRead};

/// 从文件读取字典，过滤掉无效行，返回有效 PIN 码列表
pub fn load_dict(path: &str) -> Vec<String> {
    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("字典呢？！文件 '{}' 打不开！", path);
            std::process::exit(1);
        }
    };

    let reader = io::BufReader::new(file);

    let mut pins = Vec::new();
    let mut total_lines = 0;
    let mut empty_lines = 0;
    let mut bad_lines = 0;
    let mut bad_examples: Vec<String> = Vec::new();

    for line in reader.lines() {
        total_lines += 1;

        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("警告：第 {} 行读不了，跳过。原因: {}", total_lines, e);
                continue;
            }
        };

        let pin = line.trim().to_string();

        // 过滤：空行
        if pin.is_empty() {
            empty_lines += 1;
            continue;
        }

        // 过滤：长度不是4
        if pin.len() != 4 {
            bad_lines += 1;
            if bad_examples.len() < 5 {
                bad_examples.push(format!("'{}' ({}位)", pin, pin.len()));
            }
            continue;
        }

        // 过滤：含非数字字符
        if !pin.chars().all(|c| c.is_digit(10)) {
            bad_lines += 1;
            if bad_examples.len() < 5 {
                let non_digit: String = pin.chars().filter(|c| !c.is_digit(10)).collect();
                bad_examples.push(format!("'{}' (含非数字字符: '{}')", pin, non_digit));
            }
            continue;
        }

        pins.push(pin);
    }

    // 检查结果是否为空
    if pins.is_empty() {
        eprintln!("错误：文件 '{}' 里一个有效 PIN 码都没有！", path);
        eprintln!("请检查文件内容——每行一个 4 位数字密码。");
        std::process::exit(1);
    }

    // 打印加载报告
    eprintln!("--- 字典加载报告 ---");
    eprintln!(
        "文件: {} | 总行: {} | 有效: {} | 空行: {} | 无效: {}",
        path, total_lines, pins.len(), empty_lines, bad_lines
    );
    if bad_lines > 0 {
        for example in &bad_examples {
            eprintln!("  例: {}", example);
        }
    }
    eprintln!("---");

    pins
}