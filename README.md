# CCometixLine - PackyCode

[English](README.md) | [中文](README.zh.md)

A high-performance Claude Code statusline tool written in Rust with Git integration, real-time usage tracking, and intelligent API quota monitoring.

> This is a specially adapted version of CCometixLine for PackyCode service. The original CCometixLine was created by [Haleclipse](https://github.com/Haleclipse/CCometixLine) under MIT License. This project is also released under MIT License.
> 
> PackyCode is a third-party Claude Code proxy service. This project is a voluntary third-party adaptation and is not affiliated with Anthropic or PackyCode. PackyCode websites: [packycode.com](https://www.packycode.com/) and [share.packycode.com](https://share.packycode.com/). This project implements automatic adaptation for both endpoints.

![Language:Rust](https://img.shields.io/static/v1?label=Language&message=Rust&color=orange&style=flat-square)
![License:MIT](https://img.shields.io/static/v1?label=License&message=MIT&color=blue&style=flat-square)
[![CI](https://github.com/ding113/ccline-packycc/actions/workflows/ci.yml/badge.svg)](https://github.com/ding113/ccline-packycc/actions/workflows/ci.yml)

## Screenshots

![CCometixLine](assets/img1.png)

The statusline shows: Model | Directory | Git Branch Status | Context Window | API Quota

## Features

- **High performance** with Rust native speed
- **Git integration** with branch, status, and tracking info  
- **Model display** with simplified Claude model names
- **Usage tracking** based on transcript analysis
- **Smart API quota monitoring** with intelligent dual-endpoint detection
- **Directory display** showing current workspace
- **Minimal design** using Nerd Font icons
- **Simple configuration** via command line options

## Installation

### Quick Install (Recommended)

Install via npm (works on all platforms):

```bash
# Install globally
npm install -g @ding113/ccline-packycc

# Or using yarn
yarn global add @ding113/ccline-packycc

# Or using pnpm
pnpm add -g @ding113/ccline-packycc
```

Use npm mirror for faster download:
```bash
npm install -g @ding113/ccline-packycc --registry https://registry.npmmirror.com
```

After installation:
- ✅ Global command `ccline` is available everywhere  
- ✅ Automatically configured for Claude Code at `~/.claude/ccline/ccline`
- ✅ Ready to use immediately!

### Update

```bash
npm update -g @ding113/ccline-packycc
```

### Manual Installation

Alternatively, download from [Releases](https://github.com/ding113/ccline-packycc/releases):

#### Linux

#### Option 1: Dynamic Binary (Recommended)
```bash
mkdir -p ~/.claude/ccline
wget https://github.com/ding113/ccline-packycc/releases/latest/download/ccline-linux-x64.tar.gz
tar -xzf ccline-linux-x64.tar.gz
cp ccline ~/.claude/ccline/
chmod +x ~/.claude/ccline/ccline
```
*Requires: Ubuntu 22.04+, CentOS 9+, Debian 11+, RHEL 9+ (glibc 2.35+)*

#### Option 2: Static Binary (Universal Compatibility)
```bash
mkdir -p ~/.claude/ccline
wget https://github.com/ding113/ccline-packycc/releases/latest/download/ccline-linux-x64-static.tar.gz
tar -xzf ccline-linux-x64-static.tar.gz
cp ccline ~/.claude/ccline/
chmod +x ~/.claude/ccline/ccline
```
*Works on any Linux distribution (static, no dependencies)*

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
# Create directory and download
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.claude\ccline"
Invoke-WebRequest -Uri "https://github.com/ding113/ccline-packycc/releases/latest/download/ccline-windows-x64.zip" -OutFile "ccline-windows-x64.zip"
Expand-Archive -Path "ccline-windows-x64.zip" -DestinationPath "."
Move-Item "ccline.exe" "$env:USERPROFILE\.claude\ccline\"
```

### Claude Code Configuration

Add to your Claude Code `settings.json`:

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

### Build from Source

```bash
git clone https://github.com/ding113/ccline-packycc.git
cd CCometixLine
cargo build --release

# Linux/macOS
mkdir -p ~/.claude/ccline
cp target/release/ccometixline ~/.claude/ccline/ccline
chmod +x ~/.claude/ccline/ccline

# Windows (PowerShell)
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.claude\ccline"
copy target\release\ccometixline.exe "$env:USERPROFILE\.claude\ccline\ccline.exe"
```

## Usage

```bash
# Basic usage (displays all enabled segments)
ccline

# Show help
ccline --help

# Print default configuration  
ccline --print-config

# TUI configuration mode (planned)
ccline --configure
```

## Default Segments

Displays: `Directory | Git Branch Status | Model | Context Window | API Quota`

### Git Status Indicators

- Branch name with Nerd Font icon
- Status: `✓` Clean, `●` Dirty, `⚠` Conflicts  
- Remote tracking: `↑n` Ahead, `↓n` Behind

### Model Display

Shows simplified Claude model names:
- `claude-3-5-sonnet` → `Sonnet 3.5`
- `claude-4-sonnet` → `Sonnet 4`

### Context Window Display

Token usage percentage based on transcript analysis with context limit tracking.

### API Quota Display

Smart monitoring of API usage with dual-endpoint support:
- **Daily spent**: Precise daily usage amount (e.g., `$88.48`)
- **Opus access**: Shows Opus model availability (`Opus✓` / `Opus✗`)
- **Auto-detection**: Automatically detects the correct API endpoint
- **Fault tolerance**: Seamless switching between endpoints on failure
- **Zero configuration**: Just provide your API key, everything else is automatic

Supports multiple API key sources:
- Environment variables: `PACKYCODE_API_KEY`, `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`
- Claude Code settings.json
- Local API key file: `~/.claude/api_key`

## Configuration

Configuration support is planned for future releases. Currently uses sensible defaults for all segments.

## Performance

- **Startup time**: < 50ms (vs ~200ms for TypeScript equivalents)
- **Memory usage**: < 10MB (vs ~25MB for Node.js tools)
- **Binary size**: ~2MB optimized release build

## Requirements

- **Git**: Version 1.5+ (Git 2.22+ recommended for better branch detection)
- **Terminal**: Must support Nerd Fonts for proper icon display
  - Install a [Nerd Font](https://www.nerdfonts.com/) (e.g., FiraCode Nerd Font, JetBrains Mono Nerd Font)
  - Configure your terminal to use the Nerd Font
- **Claude Code**: For statusline integration

## Development

```bash
# Build development version
cargo build

# Run tests
cargo test

# Build optimized release
cargo build --release
```

## Roadmap

- [ ] TOML configuration file support
- [ ] TUI configuration interface
- [ ] Custom themes
- [ ] Plugin system
- [ ] Cross-platform binaries

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

This project is licensed under the [MIT License](LICENSE).

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=ding113/ccline-packycc&type=Date)](https://star-history.com/#ding113/ccline-packycc&Date)
