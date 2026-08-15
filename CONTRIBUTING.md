# 贡献指南 / Contributing Guide<!-- omit in toc -->

感谢你愿意为《守夜者之书》贡献内容！这份指南告诉你如何参与。

*Thank you for your interest in contributing to The Night Keeper's Book! This guide tells you how to get involved.*

---

## 目录 / Table of Contents<!-- omit in toc -->

- [中文版](#中文版)
  - [如何贡献](#如何贡献)
  - [提交信息规范](#提交信息规范)
  - [代码规范](#代码规范)
  - [你可以贡献什么](#你可以贡献什么)
  - [讨论](#讨论)
  - [许可证](#许可证)
- [English Version](#english-version)
  - [How to Contribute](#how-to-contribute)
  - [Commit Message Convention](#commit-message-convention)
  - [Code Style](#code-style)
  - [What You Can Contribute](#what-you-can-contribute)
  - [Discussion](#discussion)
  - [License](#license)

---

## 中文版

### 如何贡献

1. **Fork 本仓库**：点击右上角的 Fork 按钮，把仓库复制到你的 GitHub 账号下。
2. **Clone 到本地**：

   ```bash
   git clone https://github.com/TheSilentOne-creator/The-Night-Keeper-s-Book.git
   ```

3. **创建分支**：

   ```bash
   git checkout -b 你的分支名
   ```

   分支名用英文，简短描述你要做什么：

   - `Fix-typo-Chapter3`：修复第三章的错别字
   - `Add-vim-Cheatsheet`：添加 Vim 速查表
   - `Translate-en-README`：翻译 README 英文版
   - `Update-install-steps`：更新安装步骤截图

4. **修改内容**：修改你 Fork 下来的文件。保存后提交。
5. **Push 到你的仓库**：

   ```bash
   git push origin 你的分支名
   ```

6. **提交 Pull Request**：回到原仓库页面，点击 Pull Requests → New Pull Request，选择你的分支，填写修改说明，提交。

### 提交信息规范

本项目使用简洁的提交信息格式：`状态 语言-教程编号-章节编号`

| 状态 | 含义 | 示例 |
| :--- | :--- | :--- |
| `Finish` | 写完了一节 | `Finish zh-01-3.6` |
| `Update` | 修改了已有内容 | `Update zh-01-3.5` |
| `Fix` | 修复了错误 | `Fix en-05-2.1` |
| `Add` | 新增了内容 | `Add zh-CONTRIBUTING` |
| `Translate` | 翻译了内容 | `Translate zh-README` |

| 语言 | 含义 |
| :--- | :--- |
| `zh` | 中文 |
| `en` | 英文 |

| 教程编号 | 教程名称 |
| :--- | :--- |
| `01` | 玩转 Markdown |
| `02` | 玩转 VS Code |
| `03` | Vim 从入门到得体 |
| `04` | 拿捏 Git 与 GitHub |
| `05` | 这才是网络安全 |
| `06` | 这才是 Python |
| `07` | Rust：灰烬之战 |
| `08` | 详解计算机科学丛书 |

### 代码规范

**Markdown 规范**  

- **标题层级**：`#` 后面有一个空格。标题层级不跳级（`##` 下面不能直接出现 `####`）。
- **图片路径**：使用相对路径，格式为 `![英文描述](<./Images/Chapter-X/X.X/xxx.png>)`。
- **内部链接**：教程内部引用其他章节时使用锚点链接。

**中英文排版规范**  

- **中文段落**：使用全角标点符号（`，。！？`）。
- **Markdown 语法符号**：`#`、`-`、`*`、`>`、`[`、`(` 等语法符号永远使用半角。
- **专有名词**：首字母大写。如 GitHub、Python、VS Code、Burp Suite。
- **英文单词前后有空格**：中文段落中嵌入的英文单词，前后各留一个半角空格。

**教程风格规范**  

- **守夜者风格**：保持幽默风趣、面向零基础的写作风格。用生活中的比喻解释技术概念。
- **Kate 的错误示范**：所有错误示范都是 Kate 写的。遵循 **“正确写法 → Kate 的翻车现场 → 原因分析 → 避坑指南”** 的四步结构。
- **零基础视角**：不要假设读者已经知道某个术语。每个新概念出现时，先用通俗语言解释它是什么，再教怎么用。

### 你可以贡献什么

| 贡献类型 | 说明 | 适合新手？ |
| :--- | :--- | :--- |
| **修正错别字、语法错误、链接失效** | 阅读教程时发现的小问题，直接修正 | ✅ 是 |
| **补充图片** | 教程中有 `![图片](<路径>)` 标记但还没有图片的地方 | ✅ 是 |
| **翻译** | 将中文教程翻译成英文，或将英文教程翻译成中文 | 🟡 需要双语能力 |
| **补充示例代码** | 在安全教程中添加完整的攻击和防御示例代码 | 🟡 需要安全知识 |
| **提出改进建议** | 在 Issues 中提出你对教程内容的改进想法 | ✅ 是 |
| **修复代码错误** | 安全教程中的 Python、Rust、Shell 代码错误 | 🟡 需要编程能力 |

### 讨论

如果你有任何问题或建议，请在 [Issues](https://github.com/TheSilentOne-creator/The-Night-Keeper-s-Book/issues) 中提出。

### 许可证

本仓库内容采用 [CC BY-NC-SA 4.0](./LICENSE) 协议。你对本仓库的任何贡献，都将以相同的许可证发布。

---

## English Version

### How to Contribute

1. **Fork this repository**: Click the Fork button in the top-right corner to copy this repository to your own GitHub account.
2. **Clone to your local machine**:

   ```bash
   git clone https://github.com/TheSilentOne-creator/The-Night-Keeper-s-Book.git
   ```

3. **Create a branch**:

   ```bash
   git checkout -b your-branch-name
   ```

   Use a short English name that describes what you're doing:

   - `Fix-typo-chapter3`: Fix typos in Chapter 3
   - `Add-vim-cheatsheet`: Add a Vim cheatsheet
   - `Translate-en-README`: Translate README to English
   - `Update-install-steps`: Update installation step screenshots

4. **Make your changes**: Edit the files you've forked. Save and commit.
5. **Push to your repository**:

   ```bash
   git push origin your-branch-name
   ```

6. **Submit a Pull Request**: Go back to the original repository page, click Pull Requests → New Pull Request, select your branch, fill in the description, and submit.

### Commit Message Convention

This project uses a concise commit message format: `status language-tutorial-section`

| Status | Meaning | Example |
| :--- | :--- | :--- |
| `Finish` | Completed a section | `Finish zh-01-3.6` |
| `Update` | Modified existing content | `Update zh-01-3.5` |
| `Fix` | Fixed an error | `Fix en-05-2.1` |
| `Add` | Added new content | `Add zh-CONTRIBUTING` |
| `Translate` | Translated content | `Translate zh-README` |

| Language | Meaning |
| :--- | :--- |
| `zh` | Chinese |
| `en` | English |

| Tutorial Number | Tutorial Name |
| :--- | :--- |
| `01` | Mastering Markdown |
| `02` | Mastering VS Code |
| `03` | Vim: From Entry to Decency |
| `04` | Git & GitHub by the Horns |
| `05` | This Is Cybersecurity |
| `06` | This Is Python |
| `07` | Rust: The Ash War |
| `08` | The Computer Science Canon Explained |

### Code Style

**Markdown Style**  

- **Heading levels**: There is a space after `#`. Heading levels should not skip (no `####` directly under `##`).
- **Image paths**: Use relative paths in the format `![description](<./Images/Chapter-X/X.X/xxx.png>)`.
- **Internal links**: Use anchor links when referencing other sections within the tutorial.

**Chinese-English Formatting Style**  

- **Chinese paragraphs**: Use full-width punctuation (`，。！？`).
- **Markdown syntax symbols**: `#`, `-`, `*`, `>`, `[`, `(` should always be half-width.
- **Proper nouns**: Capitalize the first letter. e.g., GitHub, Python, VS Code, Burp Suite.
- **Spacing around English words**: English words embedded in Chinese paragraphs should have a half-width space before and after.

**Tutorial Style Guide**  

- **Night Keeper's style**: Keep it humorous and beginner-friendly. Use real-life analogies to explain technical concepts.
- **Kate's error demonstrations**: All error demonstrations are written by Kate. Follow the four-step structure: correct approach → Kate's crash scene → root cause analysis → pitfall guide.
- **Zero-baseline perspective**: Don't assume readers already know a term. When introducing a new concept, first explain what it is in plain language, then teach how to use it.

### What You Can Contribute

| Contribution Type | Description | Beginner-Friendly? |
| :--- | :--- | :--- |
| **Fix typos, grammar errors, broken links** | Small issues found while reading the tutorial, fix directly | ✅ Yes |
| **Add missing images** | Places in the tutorial marked with `![image](<path>)` that don't have images yet | ✅ Yes |
| **Translation** | Translate Chinese tutorials to English, or English tutorials to Chinese | 🟡 Requires bilingual skills |
| **Add example code** | Add complete attack and defense example code in security tutorials | 🟡 Requires security knowledge |
| **Suggest improvements** | Post your ideas for improving tutorial content in Issues | ✅ Yes |
| **Fix code errors** | Python, Rust, Shell code errors in security tutorials | 🟡 Requires programming skills |

### Discussion

If you have any questions or suggestions, please post them in [Issues](https://github.com/TheSilentOne-creator/The-Night-Keeper-s-Book/issues).

### License

This repository is licensed under the [CC BY-NC-SA 4.0](./LICENSE) license. Any contributions you make to this repository will be released under the same license.
