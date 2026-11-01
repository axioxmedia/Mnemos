<div align="center">

# Mnemos · 忆灵

**本地优先的 Windows 长期记忆数字人 —— 记得久，上下文成本不膨胀。**

`Tauri 2 · Python sidecar · Ollama / OpenAI 兼容接口`

<br/>

[![English](https://img.shields.io/badge/English-README-blue?style=for-the-badge)](../README.md)
[![Download](https://img.shields.io/badge/Download-Releases-2ea44f?style=for-the-badge&logo=github)](https://github.com/axioxmedia/Mnemos/releases)
[![Star](https://img.shields.io/github/stars/axioxmedia/Mnemos?style=for-the-badge)](https://github.com/axioxmedia/Mnemos)

[安装](#安装) · [功能](#功能亮点) · [架构](#架构) · [文档](#文档)

<br/>

![Windows](https://img.shields.io/badge/Windows-10%2F11-0078D6?style=flat-square&logo=windows&logoColor=white)
![Tauri](https://img.shields.io/badge/Tauri-2-FFC131?style=flat-square&logo=tauri&logoColor=black)
![Python](https://img.shields.io/badge/Python-3.10%2B-3776AB?style=flat-square&logo=python&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-e7c07a?style=flat-square)
![Status](https://img.shields.io/badge/Status-MVP%200.6.9-orange?style=flat-square)

<br/>

<!-- Hero 预览图：发布前请替换为最新截图 -->
<img src="./APPCap.png" alt="忆灵应用预览" width="880" />

</div>

> **说明** — Windows 安装包发布在 **GitHub Releases**，不进入本仓库 git 树。  
> 首次启动可能需要拉取本地大模型（Ollama）及可选 TTS 资源。未签名安装包可能触发 SmartScreen：若你信任所下载的 Release，可选「更多信息 → 仍要运行」。

---

## 一眼看懂

| | |
|:--|:--|
| **是什么** | 本地优先、可长期陪伴的 Windows 桌面数字人 |
| **给谁用** | 重视隐私的陪伴用户、知识工作者、本地 AI 玩家、VRM/Live2D 创作者 |
| **不是什么** | 不是「带搜索框的聊天记录」，而是会整理、会遗忘、会归纳的记忆体 |
| **两大硬约束** | ① 长期记忆成本可控 ② 插件可扩展且有权限边界 |

---

## 解决的痛点

多数「AI 陪伴」用几周就会塌：

1. **上下文膨胀** — 把数年聊天全塞进 prompt，又贵又慢。  
2. **记忆不透明** — 看不到、改不了、钉不住、也忘不干净。  
3. **默认上云** — 私密生活离开本机。  
4. **插件两极** — 要么什么都干不了，要么权限过大。

忆灵按正典两大约束设计：

- **成本恒定**：每轮经固定槽位（S1–S7）注入记忆，第 90 天仍接近第 1 天。  
- **安全可扩展**：Process JSON-RPC 插件（后续 WASM / MCP），联网与写记忆需显式授权。

---

## 功能亮点

- **对话 +「本轮记忆」面板** — 回答前先看见用了哪些记忆  
- **分层记忆（L0–L3）** — 原始日志 → 工作记忆 → 情景 → 档案 / 事实 / 图谱  
- **记忆中枢** — 档案、时间线、知识图谱、遗忘中心  
- **本地 / 云端 LLM** — 本机 Ollama；可选 OpenAI 兼容接口（需你允许联网）  
- **形象窗** — VRM（Three.js + `@pixiv/three-vrm`）闲置动画、眨眼/眼神、唇形；Live2D 路径可用  
- **插件** — 进程隔离 JSON-RPC v0；联网默认关闭  
- **Windows 安装包** — Tauri 2 NSIS（`Mnemos_*_x64-setup.exe`）

> **模型权重**（`.vrm` / LLM / TTS）**不入库**。请将自有 VRM 放到 `public/avatar/models/`（见该目录说明）。动作与房间 GLB 的许可证见 `THIRD_PARTY_ASSETS.md`。

---

## 安装

### A. 普通用户（推荐）

1. 打开 **[Releases](https://github.com/axioxmedia/Mnemos/releases)**，下载最新 `Mnemos_*_x64-setup.exe`。  
2. 运行安装程序（Windows 10/11 x64；WebView2 一般已预装）。  
3. 启动 **忆灵 Mnemos**，完成向导，配置 Ollama 或兼容 API。  
4. 可选：打开**形象**窗口，加载你有权使用的 VRM。

维护者本机暂存安装包（**不要提交进 git**）：

`Mnemos-github-assets/Mnemos_0.6.9_x64-setup.exe（本机暂存，见 PUBLISH.md）` → 上传到对应 GitHub Release。

### B. 开发者（从源码）

**环境：** Node.js 20+、Rust stable、Python 3.10+、WebView2、Windows x64。可选 [Ollama](https://ollama.com)。

```powershell
git clone https://github.com/axioxmedia/Mnemos.git
cd Mnemos
npm install

# 引擎 sidecar
python -m venv sidecar\.venv
.\sidecar\.venv\Scripts\Activate.ps1
pip install -e .\sidecar
$env:MNEMOS_DATA_DIR = "$env:USERPROFILE\.mnemos"
python -m mnemos_sidecar
# 监听 http://127.0.0.1:18765
```

另开终端：

```powershell
npm run tauri:dev
# 或仅前端：npm run dev  → http://localhost:1420
```

正式构建：

```powershell
npm run tauri:build
# 产物一般在 src-tauri\target\release\bundle\
```

冒烟（验收「成本恒定」硬约束）：

```powershell
python scripts\smoke_test.py
# 期望：SMOKE OK
```

---

## 架构

```text
┌───────────────────────────┐
│  Tauri 2 + React/TS UI    │  对话 · 记忆中枢 · 设置 · 形象窗
└─────────────┬─────────────┘
              │ 本机 HTTP JSON
┌─────────────▼─────────────┐
│  Python sidecar           │  L0–L3 · 检索 · 巩固 · 插件
└─────────────┬─────────────┘
              │
     ┌────────┼────────┐
     ▼        ▼        ▼
  Ollama   SQLite    插件
  / API    (+vec/FTS) (JSON-RPC)
```

正典摘要：[`CANON.md`](../CANON.md)（完整策划 PDF 在仓库内）。

---

## 文档

| 文档 | 说明 |
|:----|:------------|
| [../README.md](../README.md) | English landing page |
| [../CANON.md](../CANON.md) | 产品正典 / 硬约束 |
| [../RELEASE_NOTES.md](../RELEASE_NOTES.md) | 发行说明 |
| [../resources/avatar/THIRD_PARTY_ASSETS.md](../resources/avatar/THIRD_PARTY_ASSETS.md) | 形象动作 / 场景许可证 |
| [community-plugins.md](./community-plugins.md) | 插件说明 |
| [../PUBLISH.md](../PUBLISH.md) | 干净树与 Release 暂存说明 |

### 常见问题

**大模型权重在哪？** 不在 git。用 Ollama（`ollama pull …`）或自建 API。  
**示例 VRM？** 默认不二次分发 — 自行放入 `public/avatar/models/`。  
**为何安装包单独发？** 约 60MB+；保持仓库精简，挂到 Releases。

---

## 许可证

[MIT License](../LICENSE)。  
第三方形象动作 / 场景保留原 CC/MIT 声明（见 `THIRD_PARTY_ASSETS.md`）。

---

<div align="center">

为「记得克制、而不是无限堆上下文」的人而做。

[English](../README.md) · [Releases](https://github.com/axioxmedia/Mnemos/releases) · [正典](../CANON.md)

</div>
