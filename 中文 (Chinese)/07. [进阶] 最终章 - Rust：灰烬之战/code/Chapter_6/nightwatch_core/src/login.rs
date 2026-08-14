use reqwest::blocking::Client;

/// 尝试用 PIN 码登录目标地址
/// 返回 Ok(true) 表示密码正确，Ok(false) 表示密码错误，Err 表示网络错误
pub fn try_login(client: &Client, pin: &str, target: &str) -> Result<bool, reqwest::Error> {
    let response = client
        .post(target)
        .header("Content-Type", "application/json")
        .body(serde_json::json!({"pin": pin}).to_string())
        .send()?;

    Ok(response.status().is_success())
}