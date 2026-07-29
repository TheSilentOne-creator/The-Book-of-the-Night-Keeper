// ===== 预演程序：模拟字典爆破 =====

use std::fs;
use std::io::{self, BufRead};

const DICT_PATH: &str = "pin_dict.txt";

// 主函数：程序的入口
fn main() {
    // 1. 模拟读字典（真实情况是从文件读）
    let pins = read_demo_dict();

    // 2. 遍历每个密码
    for (i, pin) in pins.iter().enumerate() {
        print!("[{}] 正在检查: {} ... ", i + 1, pin);

        // 3. 验证：长度必须是4
        if pin.len() != 4 {
            println!("长度不对，跳过");
            continue;
        }

        // 4. 验证：必须全是数字
        if !pin.chars().all(|c| c.is_digit(10)) {
            println!("含非数字字符，跳过");
            continue;
        }

        // 5. 验证：是不是正确答案
        if pin == "4247" {
            println!(">>> 找到了！就是它！ <<<");
            break;
        }

        println!("不对，继续。");
    }
}

// 模拟字典数据（返回一个 String 数组）
fn read_demo_dict() -> Vec<String> {
    let mut list = Vec::new();
    list.push(String::from("1234"));
    list.push(String::from("abc"));
    list.push(String::from("0000"));
    list.push(String::from("4247"));
    list  // 返回数组
}
