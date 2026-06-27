# Rust是怎样炼成的

## 目录

- [Rust是怎样炼成的](#rust是怎样炼成的)
  - [目录](#目录)
  - [第三章-Cargo：你的 Rust 开发“瑞士军刀”🔧](#第三章-cargo你的-rust-开发瑞士军刀)
    - [3.1 Cargo 是什么？](#31-cargo-是什么)
    - [3.2 Cargo 的核心特性](#32-cargo-的核心特性)
    - [3.3 Cargo 常用命令速查表](#33-cargo-常用命令速查表)
      - [3.3.1 💻 实际演练一下](#331--实际演练一下)
      - [3.3.2 🎯 Cargo 的设计哲学](#332--cargo-的设计哲学)
    - [3.4 在 VSCode 中配置 Rust 工程](#34-在-vscode-中配置-rust-工程)

---

## 第三章-Cargo：你的 Rust 开发“瑞士军刀”🔧

欢迎来到 Cargo 的世界！如果说 Rust 编译器是你的“发动机”，那么 Cargo 就是整辆车的“控制系统”——它让你开得更稳、更快，还帮你管理“燃料”（依赖库）。

### 3.1 Cargo 是什么？

简单来说，Cargo 是 Rust 的**官方构建系统** + **包管理器**二合一工具。它就像 Rust 项目的“超级管家”：

| 角色 | 具体职责 |
| ------ | -------------------- |
| 🎯 项目管理员 | 创建项目结构、管理配置文件、组织代码 |
| 📦 包管理器 | 下载依赖、版本控制、解决依赖冲突 |
| 🔨 构建工程师 | 编译代码、链接库、优化二进制文件 |
| 🧪 质量检测员 | 运行测试、检查代码质量 |

想象一下：没有 Cargo，你得手动下载依赖、管理版本、配置编译器……有了 Cargo，你只需要几个简单命令，它帮你搞定一切！

### 3.2 Cargo 的核心特性

为什么Cargo它这么香？

Cargo 的强大体现在这些方面：

1. 📁 智能项目管理

   - 通过 `cargo new` 一键生成标准项目结构
   - `Cargo.toml` 文件管理所有配置和依赖（就像 Node.js 的 `package.json`）

2. 📦 强大的依赖管理

   - 自动从 crates.io（Rust 的官方包仓库）下载依赖
   - 版本锁定机制（`Cargo.lock` 文件）确保每次构建结果一致
   - 支持 Git 仓库、本地路径等多种依赖来源

3. ⚡ 高效的构建系统

   - **增量编译：** 只重新编译修改过的文件，大幅提升构建速度
   - **并行编译：** 利用多核 CPU 加速编译过程
   - **构建缓存：** 缓存已编译的依赖，避免重复工作

4. 🔍 全方位的质量保障

   - `cargo check`：快速语法检查（比完整编译快 10 倍！）
   - `cargo test`：运行单元测试和集成测试
   - `cargo clippy`：代码质量检查（像一位严格的代码审查员）

5. 🌍 跨平台支持

   - 一套命令通吃 Windows、macOS、Linux
   - 轻松交叉编译到其他平台（如嵌入式设备）

6. 🔧 丰富的扩展能力

   - 自定义构建脚本（`build.rs`）
   - 条件编译和特性标志（features）
   - 插件系统（可通过 `cargo install` 安装各种工具）

7. 📊 专业级工具链

   - `cargo bench`：基准测试，测量代码性能
   - `cargo doc`：生成漂亮的 API 文档（自动托管到 [docs.rs](https://docs.rs/)）
   - `cargo publish`：一键发布到 [crates.io](https://crates.io/)

8. 💡 开发者友好设计

   - **离线支持：** 断网也能正常开发（依赖已缓存）
   - **环境变量覆盖：** 灵活调整构建行为
   - **插件生态：** 数百个第三方工具扩展 Cargo 功能

### 3.3 Cargo 常用命令速查表

以下是你每天都会用到的 Cargo 命令，建议收藏：

| 命令 | 作用 | 使用频率 |
| ------ | ------ | ------ |
| `cargo new <项目名>` | 创建新项目 | ⭐⭐⭐⭐⭐（一次） |
| `cargo build` | 编译项目（生成可执行文件） | ⭐⭐⭐⭐ |
| `cargo run` | 编译并运行项目 | ⭐⭐⭐⭐⭐ |
| `cargo check` | 快速检查语法错误（不生成二进制文件） | ⭐⭐⭐⭐⭐ |
| `cargo test` | 运行所有测试 | ⭐⭐⭐⭐ |
| `cargo clean` | 清理构建产物（释放磁盘空间） | ⭐⭐⭐ |
| `cargo update` | 更新依赖到最新兼容版本 | ⭐⭐⭐ |
| `cargo add <包名>` | 添加新依赖（最方便的方式！） | ⭐⭐⭐⭐ |
| `cargo doc --open` | 生成并打开项目文档 | ⭐⭐ |
| `cargo clippy` | 代码 lint 检查（推荐安装） | ⭐⭐⭐ |
| `cargo fmt` | 自动格式化代码（保持代码风格统一） | ⭐⭐⭐⭐ |

#### 3.3.1 💻 实际演练一下

让我们重温上一节的操作，看看 Cargo 的实际工作流程：

```bash
# 1. 创建新项目
cargo new my_awesome_project
cd my_awesome_project

# 2. 快速检查代码（0.5秒完成）
cargo check

# 3. 编译项目
cargo build

# 4. 运行项目
cargo run

# 5. 添加一个依赖（比如用于处理时间的库）
cargo add chrono

# 6. 查看项目结构
tree .  # Windows 可用 `dir /s`
```

你会看到 Cargo 创建的标准结构：

```text
my_awesome_project/
├── Cargo.toml      # 项目配置和依赖
├── Cargo.lock      # 依赖版本锁定（自动生成）
├── src/
│   └── main.rs     # 程序入口
└── target/         # 构建产物（编译后生成）
```

#### 3.3.2 🎯 Cargo 的设计哲学

Cargo 的核心设计原则是 **“约定优于配置”**。这意味着：

1. **标准化：** 所有 Rust 项目都有相似的结构
2. **自动化：** 常见操作都有默认行为
3. **可重现：** 在任何机器上构建结果都一致
4. **社区友好：** 统一的工作流方便协作

> 🦀 Rustacean 小贴士：
刚开始学习时，重点关注 `cargo run`、`cargo check`、`cargo add` 这几个命令
养成使用 `cargo check` 的习惯，它比 `cargo build` 快得多
不要手动编辑 `Cargo.lock` 文件！它由 Cargo 自动管理
使用 `cargo --help` 查看所有命令，`cargo <命令> --help` 查看具体用法

### 3.4 在 VSCode 中配置 Rust 工程

Cargo 是一个不错的构建工具，如果使 VSCode 与它相配合那么 VSCode 将会是一个十分便捷的开发环境。

在上一章中我们建立了 greeting 工程，现在我们用 VSCode 打开 `learn_rust` 文件夹（**注意不是 `first`**）。

打开 learn_rust 之后，在里面新建一个新的文件夹 **`.vscode`** （注意 vscode 前面的点，如果有这个文件夹就不需要新建了）。在新建的 .vscode 文件夹里新建两个文件 `tasks.json` 和 `launch.json`，文件内容如下：

tasks.json 文件

```json
{ 
    "version":"2.0.0", 
    "tasks":[ 
        { 
            "label":"build", 
            "type":"shell", 
            "command":"cargo", 
            "args":["build"] 
        } 
    ] 
}
```

launch.json 文件（适用在 Windows 系统上）

```json
{ 
    "version":"0.2.0", 
    "configurations":[ 
        { 
            "name":"(Windows)启动", 
            "preLaunchTask":"build", 
            "type":"cppvsdbg", 
            "request":"launch", 
            "program":"${workspaceFolder}/target/debug/${workspaceFolderBasename}.exe", 
            "args":[], 
            "stopAtEntry":false, 
            "cwd":"${workspaceFolder}", 
            "environment":[], 
            "externalConsole":false 
        }, 
        { 
            "name":"(gdb)启动", 
            "type":"cppdbg", 
            "request":"launch", 
            "program":"${workspaceFolder}/target/debug/${workspaceFolderBasename}.exe", 
            "args":[], 
            "stopAtEntry":false, 
            "cwd":"${workspaceFolder}", 
            "environment":[], 
            "externalConsole":false, 
            "MIMode":"gdb", 
            "miDebuggerPath":"这里填GDB所在的目录", 
            "setupCommands":[ 
                { 
                    "description":"为 gdb 启用整齐打印", 
                    "text":"-enable-pretty-printing", 
                    "ignoreFailures":true 
                } 
            ] 
        } 
    ] 
}
```

launch.json 文件（适用在 Linux 系统上）

```json
{
    "version":"0.2.0",
    "configurations":[
        {
            "name":"Debug",
            "type":"gdb",
            "preLaunchTask":"build",
            "request":"launch",
            "target":"${workspaceFolder}/target/debug/${workspaceFolderBasename}",
            "cwd":"${workspaceFolder}"
        }
    ]
}
```

launch.json 文件（适用在 Mac OS 系统上）

```json
{
    "version":"0.2.0",
    "configurations":[
        {
            "name":"(lldb) 启动",
            "type":"cppdbg",
            "preLaunchTask":"build",
            "request":"launch",
            "program":"${workspaceFolder}/target/debug/${workspaceFolderBasename}",
            "args":[],
            "stopAtEntry":false,
            "cwd":"${workspaceFolder}",
            "environment":[],
            "externalConsole":false,
            "MIMode":"lldb"
        }
    ]
}
```

然后点击 VSCode 左栏的 "运行"。

如果你使用的是 MSVC 选择 "(Windows) 启动"。

如果使用的是 MinGW 且安装了 GDB 选择"(gdb)启动"，gdb 启动前请注意填写 launch.json 中的 "miDebuggerPath"。

![alt text](配置Cargo.jpg)

程序就会开始调试运行了。运行输出将出现在"调试控制台"中：

![alt text](cargo2.png)

在 VSCode 中调试 Rust
调试程序的方法与其它环境相似，只需要在行号的左侧点击红点就可以设置断点，在运行中遇到断点会暂停，以供开发者监视实时变量的值。

![alt text](cargo3.png)

> Cargo 可能是你遇到过最贴心的构建工具。下一章，我们将深入 Rust 语法，开始真正的编程之旅！准备好敲代码了吗？🚀
