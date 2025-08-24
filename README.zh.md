# CCometixLine - PackyCode

[English](README.md) | [中文](README.zh.md)

基于 Rust 的高性能 Claude Code 状态栏工具，集成 Git 信息、实时使用量跟踪和智能 API 配额监控。

> 这是专为 PackyCode 服务特别适配的 CCometixLine 版本。原版 CCometixLine 由 [Haleclipse](https://github.com/Haleclipse/CCometixLine) 基于 MIT License 创建。本项目也基于 MIT License 发布。
>
> PackyCode 是一个第三方的 Claude Code 中转服务。本项目为第三方自发适配，与 Anthropic 和 PackyCode 均无关。PackyCode 官网：[packycode.com](https://www.packycode.com/) 和 [share.packycode.com](https://share.packycode.com/)。本项目实现了对两个端点的自动适配。

![Language:Rust](https://img.shields.io/static/v1?label=Language&message=Rust&color=orange&style=flat-square)
![License:MIT](https://img.shields.io/static/v1?label=License&message=MIT&color=blue&style=flat-square)
[![CI](https://github.com/ding113/ccline-packycc/actions/workflows/ci.yml/badge.svg)](https://github.com/ding113/ccline-packycc/actions/workflows/ci.yml)

## 截图

![CCometixLine](assets/img1.png)

状态栏显示：模型 | 目录 | Git 分支状态 | 上下文窗口 | API 配额

## 特性

- **高性能** Rust 原生速度
- **Git 集成** 显示分支、状态和跟踪信息
- **模型显示** 简化的 Claude 模型名称
- **使用量跟踪** 基于转录文件分析  
- **智能 API 配额监控** 支持双端点智能检测
- **目录显示** 显示当前工作空间
- **简洁设计** 使用 Nerd Font 图标
- **简单配置** 通过命令行选项配置

## 安装

### 快速安装（推荐）

通过 npm 安装（适用于所有平台）：

```bash
# 全局安装
npm install -g @ding113/ccline-packycc

# 或使用 yarn
yarn global add @ding113/ccline-packycc

# 或使用 pnpm
pnpm add -g @ding113/ccline-packycc
```

使用镜像源加速下载：
```bash
npm install -g @ding113/ccline-packycc --registry https://registry.npmmirror.com
```

安装后：
- ✅ 全局命令 `ccline` 可在任何地方使用
- ✅ 自动配置 Claude Code 到 `~/.claude/ccline/ccline`
- ✅ 立即可用！

### 更新

```bash
npm update -g @ding113/ccline-packycc
```

### 手动安装

或者从 [Releases](https://github.com/ding113/ccline-packycc/releases) 手动下载：

#### Linux

#### 选项 1: 动态链接版本（推荐）
```bash
mkdir -p ~/.claude/ccline
wget https://github.com/ding113/ccline-packycc/releases/latest/download/ccline-linux-x64.tar.gz
tar -xzf ccline-linux-x64.tar.gz
cp ccline ~/.claude/ccline/
chmod +x ~/.claude/ccline/ccline
```
*系统要求: Ubuntu 22.04+, CentOS 9+, Debian 11+, RHEL 9+ (glibc 2.35+)*

#### 选项 2: 静态链接版本（通用兼容）
```bash
mkdir -p ~/.claude/ccline
wget https://github.com/ding113/ccline-packycc/releases/latest/download/ccline-linux-x64-static.tar.gz
tar -xzf ccline-linux-x64-static.tar.gz
cp ccline ~/.claude/ccline/
chmod +x ~/.claude/ccline/ccline
```
*适用于任何 Linux 发行版（静态链接，无依赖）*

### macOS (Intel)

```bash  
mkdir -p ~/.claude/ccline
wget https://github.com/ding113/ccline-packycc/releases/latest/download/ccline-macos-x64.tar.gz
tar -xzf ccline-macos-x64.tar.gz
cp ccline ~/.claude/ccline/
chmod +x ~/.claude/ccline/ccline
```

### macOS (Apple Silicon)

```bash
mkdir -p ~/.claude/ccline  
wget https://github.com/ding113/ccline-packycc/releases/latest/download/ccline-macos-arm64.tar.gz
tar -xzf ccline-macos-arm64.tar.gz
cp ccline ~/.claude/ccline/
chmod +x ~/.claude/ccline/ccline
```

### Windows

```powershell
# 创建目录并下载
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.claude\ccline"
Invoke-WebRequest -Uri "https://github.com/ding113/ccline-packycc/releases/latest/download/ccline-windows-x64.zip" -OutFile "ccline-windows-x64.zip"
Expand-Archive -Path "ccline-windows-x64.zip" -DestinationPath "."
Move-Item "ccline.exe" "$env:USERPROFILE\.claude\ccline\"
```

### 从源码构建

```bash
git clone https://github.com/ding113/ccline-packycc.git
cd CCometixLine
cargo build --release
cp target/release/ccometixline ~/.claude/ccline/ccline
```

### Claude Code 配置

添加到 Claude Code `settings.json`：

**Linux/macOS:**
```json
{
  "statusLine": {
    "type": "command", 
    "command": "~/.claude/ccline/ccline",
    "padding": 0
  }
}
```

**Windows:**
```json
{
  "statusLine": {
    "type": "command", 
    "command": "%USERPROFILE%\\.claude\\ccline\\ccline.exe",
    "padding": 0
  }
}
```

## 使用

```bash
# 基础使用 (显示所有启用的段落)
ccline

# 显示帮助
ccline --help

# 打印默认配置
ccline --print-config

# TUI 配置模式 (计划中)
ccline --configure
```

## 默认段落

显示：`目录 | Git 分支状态 | 模型 | 上下文窗口 | API 配额`

### Git 状态指示器

- 带 Nerd Font 图标的分支名
- 状态：`✓` 清洁，`●` 有更改，`⚠` 冲突
- 远程跟踪：`↑n` 领先，`↓n` 落后

### 模型显示

显示简化的 Claude 模型名称：
- `claude-3-5-sonnet` → `Sonnet 3.5`
- `claude-4-sonnet` → `Sonnet 4`

### 上下文窗口显示

基于转录文件分析的令牌使用百分比，包含上下文限制跟踪。

### API 配额显示

智能 API 使用量监控，支持双端点：
- **每日花费**：精确的每日使用金额（如 `$88.48`）
- **Opus 权限**：显示 Opus 模型可用性（`Opus✓` / `Opus✗`）
- **自动检测**：自动检测正确的 API 端点
- **故障容错**：端点故障时无缝切换
- **零配置**：只需提供 API key，其他全自动


## 配置

计划在未来版本中支持配置。当前为所有段落使用合理的默认值。

## 性能

- **启动时间**：< 50ms（TypeScript 版本约 200ms）
- **内存使用**：< 10MB（Node.js 工具约 25MB）
- **二进制大小**：约 2MB 优化版本

## 系统要求

- **Git**: 版本 1.5+ (推荐 Git 2.22+ 以获得更好的分支检测)
- **终端**: 必须支持 Nerd Font 图标正常显示
  - 安装 [Nerd Font](https://www.nerdfonts.com/) 字体
  - 中文用户推荐: [Maple Font](https://github.com/subframe7536/maple-font) (支持中文的 Nerd Font)
  - 在终端中配置使用该字体
- **Claude Code**: 用于状态栏集成

## 开发

```bash
# 构建开发版本
cargo build

# 运行测试
cargo test

# 构建优化版本
cargo build --release
```

## 路线图

- [ ] TOML 配置文件支持
- [ ] TUI 配置界面
- [ ] 自定义主题
- [ ] 插件系统
- [ ] 跨平台二进制文件

## 贡献

欢迎贡献！请随时提交 issue 或 pull request。

## 许可证

本项目采用 [MIT 许可证](LICENSE)。

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=ding113/CCometixLine&type=Date)](https://star-history.com/#ding113/CCometixLine&Date)