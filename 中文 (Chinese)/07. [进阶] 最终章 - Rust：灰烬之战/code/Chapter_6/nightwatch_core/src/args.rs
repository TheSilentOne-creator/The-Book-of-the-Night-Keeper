use std::env;

/// 解析命令行参数，返回 (目标地址, 字典路径)
pub fn parse_args() -> (String, String) {
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