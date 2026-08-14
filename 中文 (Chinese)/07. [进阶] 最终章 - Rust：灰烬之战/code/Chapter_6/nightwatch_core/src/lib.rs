pub mod args;
pub mod dict;
pub mod login;

// 把最常用的函数重新导出到 crate 顶层
pub use args::parse_args;
pub use dict::load_dict;
pub use login::try_login;