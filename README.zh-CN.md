# md-bider（马得笔）

[English](README.md) | 简体中文

<div align="center">
  <img src="docs/hero.svg" alt="md-bider hero" width="100%" />

  <p>
    <strong>让人打开就想点星的 Rust 原生 Markdown 桌面应用。</strong><br/>
    启动快、上手快、本地多文件顺滑、离线稳、中文文档更安心。
  </p>

  <p>
    <img src="https://img.shields.io/badge/Engine-Rust%20Native-C26A16?logo=rust" alt="Rust Native" />
    <img src="https://img.shields.io/badge/Workflow-Chrome--Style%20Tabs-0A7CFF" alt="Chrome Style Tabs" />
    <img src="https://img.shields.io/badge/Editing-IR%20%7C%20SV%20%7C%20WYSIWYG-1E5CB3" alt="Editing Modes" />
    <img src="https://img.shields.io/badge/Architecture-Offline%20First-0E8A5A" alt="Offline First" />
    <img src="https://img.shields.io/badge/Encoding-CJK%20Safe-E76F51" alt="CJK Safe" />
    <img src="https://img.shields.io/github/stars/cloveric/md-bider?style=flat&logo=github" alt="GitHub Stars" />
    <img src="https://github.com/cloveric/md-bider/actions/workflows/ci.yml/badge.svg" alt="CI" />
    <img src="https://github.com/cloveric/md-bider/actions/workflows/release.yml/badge.svg" alt="Release" />
  </p>

  <p>
    <a href="https://github.com/cloveric/md-bider/stargazers"><strong>点亮 Star</strong></a> ·
    <a href="https://github.com/cloveric/md-bider/releases/latest"><strong>下载最新版</strong></a>
  </p>
</div>

## 为什么 md-bider 更容易被点星

- **打开就写**：默认进入 `IR` 模式，零准备、零等待。
- **标签页像浏览器一样顺手**：多文件并行编辑更接近日常桌面习惯。
- **中文/旧编码更稳**：支持 `UTF-8`、`UTF-16 BOM`、探测与 `GBK` 回退。
- **Rust 原生编译**：运行行为干净，性能和稳定性更可控。
- **离线优先**：核心编辑资源内嵌，断网照样工作。

## 软件截图

<div align="center">
  <img src="docs/screenshots/app-main.png" alt="md-bider app screenshot" width="100%" />
</div>

## 对比常见 Markdown 编辑器

| 维度 | 常见体验 | md-bider |
| --- | --- | --- |
| 打开到开写 | 先设置再输入 | 打开即写 |
| 本地多文件 | 偏单文档模式 | 原生标签页并行 |
| 中文编码兼容 | 多数 UTF-8 优先 | UTF-16 BOM + 探测 + GBK 回退 |
| 离线可靠性 | 受插件或外网影响 | 核心资源内嵌，离线可用 |
| 桌面技术路线 | 浏览器壳优先 | Rust 原生壳 + 本地 IO 优先 |

## Rust 带来的不是口号，而是体验差异

md-bider 采用 Rust 桌面壳（`tao + wry`）和本地文件优先的架构，不是单纯网页套壳。实际收益是：启动和运行路径更清晰、打包更直接、在真实本地工作流下更稳定。

## 功能总览

| 能力 | 说明 |
| --- | --- |
| 编辑模式 | `IR`、`SV`、`WYSIWYG` |
| 标签页工作流 | 新建、切换、关闭多个本地 Markdown 文件 |
| 文件操作 | 新建、打开、保存、另存为 |
| 快捷键 | `Ctrl+N / Ctrl+O / Ctrl+S / Ctrl+Shift+S / Ctrl+W` |
| 命令行打开 | `md-bider.exe <file.md>` |
| 离线运行 | JS/CSS/i18n 资源内嵌 |

## 获取 md-bider

- 发布页：<https://github.com/cloveric/md-bider/releases/latest>
- Windows：`md-bider-vX.Y.Z-windows-x64.zip` -> 运行 `md-bider.exe`
- macOS：`md-bider-vX.Y.Z-macos-*.zip` -> 将 `md-bider.app` 拖入 `Applications`

## 源码构建

```powershell
git clone https://github.com/cloveric/md-bider.git
cd md-bider
cargo build --release
```

- Windows：`./target/release/md-bider.exe`
- macOS：`./target/release/md-bider`

## 项目定位

md-bider 的目标非常明确：让本地 Markdown 工作流“快到不打断思路，稳到可以长期依赖”。如果你重视本地文件、离线能力和写作节奏，这就是为你准备的桌面应用。

## 参与贡献

欢迎 Issue 和 PR。详见 [CONTRIBUTING.md](CONTRIBUTING.md) 与 [CHANGELOG.md](CHANGELOG.md)。

## License

MIT
