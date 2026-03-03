# md-bider

<div align="center">
  <img src="docs/hero.svg" alt="md-bider" width="100%" />

  <p>
    <strong>面向中文创作场景的桌面级 Markdown 编辑与审阅工作台</strong><br/>
    默认所见即所得，离线可用，支持多模式切换与本地文件工作流。
  </p>

  <p>
    <img src="https://img.shields.io/badge/Rust-Edition%202024-black?logo=rust" alt="Rust" />
    <img src="https://img.shields.io/badge/Platform-Windows-0078D4" alt="Platform" />
    <img src="https://img.shields.io/badge/Mode-IR%20%7C%20SV%20%7C%20WYSIWYG-2A5CAA" alt="Modes" />
    <img src="https://img.shields.io/badge/Network-Offline%20First-1D7F5F" alt="Offline" />
    <img src="https://github.com/cloveric/md-bider/actions/workflows/ci.yml/badge.svg" alt="CI" />
    <img src="https://github.com/cloveric/md-bider/actions/workflows/release.yml/badge.svg" alt="Release" />
    <img src="https://img.shields.io/github/v/release/cloveric/md-bider?display_name=tag" alt="Latest Release" />
    <img src="https://img.shields.io/github/downloads/cloveric/md-bider/total" alt="Downloads" />
  </p>
</div>

## 为什么是它

md-bider 不是“在线文档工具”的桌面壳，而是一套针对本地 Markdown 使用习惯优化的工作台：

- 默认进入 `IR` 所见即所得模式，打开即写
- `SV / IR / WYSIWYG` 三模式一键切换，兼顾结构化与可视化
- 核心编辑资源内嵌，离线可用，不依赖外网
- 面向中文排版优化，文件编码兼容处理更稳
- 本地文件流顺畅：新建、打开、保存、另存为、命令行传参启动

## 功能总览

| 能力 | 说明 |
| --- | --- |
| 三种编辑模式 | `SV` 分栏、`IR` 所见即所得、`WYSIWYG` 富文本 |
| 默认编辑体验 | 启动默认 `IR`，更贴近日常写作与审阅 |
| 文件操作 | 新建、打开、保存、另存为 |
| 快捷键 | `Ctrl+N/O/S/Shift+S` |
| 离线能力 | 编辑引擎、样式、语言包均内嵌到可执行文件 |
| 命令行启动 | 支持 `md-bider.exe <file.md>` 直接打开文件 |

## 下载与发布

- 最新稳定版：<https://github.com/cloveric/md-bider/releases/latest>
- Windows 用户下载 `md-bider-vX.Y.Z-windows-x64.zip`，解压后运行 `md-bider.exe`
- macOS 用户下载 `md-bider-vX.Y.Z-macos-*.zip`，解压后将 `md-bider.app` 拖入 `Applications`

## 快速开始

### 1. 运行发布版

```powershell
cd C:\Users\hangw\md-bider
cargo build --release
.\target\release\md-bider.exe
```

### 2. 带文件启动

```powershell
.\target\release\md-bider.exe C:\path\to\README.md
```

## 键盘快捷键

| 快捷键 | 动作 |
| --- | --- |
| `Ctrl + N` | 新建空白文档 |
| `Ctrl + O` | 打开文件 |
| `Ctrl + S` | 保存 |
| `Ctrl + Shift + S` | 另存为 |

## 技术架构

```text
Rust (tao + wry) Desktop Shell
        |
        | IPC (JSON commands/events)
        v
Embedded Editor Shell (HTML/CSS/JS)
        |
        +-- local file IO (UTF-8 + fallback)
        +-- offline embedded assets
```

关键模块：

- `src/main.rs`：应用入口、窗口生命周期、IPC 路由
- `src/desktop.rs`：IPC 协议定义（命令/事件）
- `src/io.rs`：文件读写与编码回退
- `assets/editor_shell.html`：编辑器 UI 与交互逻辑
- `assets/vendor/*.b64`：内嵌资源（脚本/样式/语言包）

## 开发与测试

```powershell
cd C:\Users\hangw\md-bider
cargo test
cargo build --release
```

## 路线图

- 文档层：补充使用手册与常见问题
- 产品层：更多阅读/排版预设
- 工程层：打包流程与版本发布自动化

详见 [CHANGELOG.md](CHANGELOG.md) 与 [CONTRIBUTING.md](CONTRIBUTING.md)。

## License

MIT

