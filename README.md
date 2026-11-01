<div align="center">

# Mnemos

**A local-first Windows companion that remembers for years — without blowing up your context window.**

`Tauri 2 · Python sidecar · Ollama / OpenAI-compatible`

<br/>

[![中文文档](https://img.shields.io/badge/%E4%B8%AD%E6%96%87-docs-blue?style=for-the-badge)](./docs/README-zh.md)
[![Download](https://img.shields.io/badge/Download-Releases-2ea44f?style=for-the-badge&logo=github)](https://github.com/axioxmedia/Mnemos/releases)
[![Star](https://img.shields.io/github/stars/axioxmedia/Mnemos?style=for-the-badge)](https://github.com/axioxmedia/Mnemos)

[Install](#install) · [Features](#features) · [Architecture](#architecture) · [Docs](#documentation)

<br/>

![Windows](https://img.shields.io/badge/Windows-10%2F11-0078D6?style=flat-square&logo=windows&logoColor=white)
![Tauri](https://img.shields.io/badge/Tauri-2-FFC131?style=flat-square&logo=tauri&logoColor=black)
![Python](https://img.shields.io/badge/Python-3.10%2B-3776AB?style=flat-square&logo=python&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-e7c07a?style=flat-square)
![Status](https://img.shields.io/badge/Status-MVP%200.6.9-orange?style=flat-square)

<br/>

<!-- Hero preview: replace docs/APPCap.png with a fresh capture before publish -->
<img src="./docs/APPCap.png" alt="Mnemos app preview" width="880" />

</div>

> **NOTE** — The Windows installer is published on **GitHub Releases**, not inside this git tree.  
> First launch may ask you to pull a local LLM (Ollama) and optionally TTS assets. SmartScreen may warn on unsigned builds — choose *More info → Run anyway* if you trust the Release you downloaded.

---

## At a glance

| | |
|:--|:--|
| **What** | Local-first long-term memory digital human for Windows |
| **Who** | Privacy-minded companions, knowledge workers, local-AI tinkerers, VRM/Live2D creators |
| **Why not "chat logs + search"?** | Mnemos **organizes, forgets, and summarizes** under a fixed context budget |
| **Hard rules** | (1) Long-term memory cost stays flat (2) Capabilities grow via plugins with permission boundaries |

---

## Why Mnemos? (pain points)

Most "AI companions" fail after a few weeks:

1. **Context bloat** — dumping years of chat into the prompt gets expensive and slow.  
2. **Opaque memory** — you cannot see, pin, correct, or truly forget what the model used.  
3. **Cloud-by-default** — private life leaves the machine.  
4. **Toy extensibility** — plugins either cannot reach tools or can reach *everything*.

Mnemos is built around two product constraints from the design canon:

- **Constant cost** — each turn injects memory through fixed slots (S1–S7), so day 90 feels like day 1.  
- **Safe extensibility** — Process JSON-RPC plugins (toward WASM / MCP) with explicit network & memory permissions.

---

## Features

- **Chat + "this-turn memory" panel** — see what was retrieved before you trust the answer  
- **Layered memory (L0–L3)** — raw log → working memory → episodes → semantic profile / facts / graph  
- **Memory hub** — archive, timeline, knowledge graph, forget center  
- **Local / cloud LLM** — Ollama locally; optional OpenAI-compatible endpoints when you allow network  
- **Avatar window** — VRM (Three.js + `@pixiv/three-vrm`) with idle VRMA clips, blink/look-at, lip-sync; Live2D path available  
- **Plugins** — process-isolated JSON-RPC v0; network is opt-in  
- **Windows installer** — Tauri 2 NSIS setup (`Mnemos_*_x64-setup.exe`)

> Avatar **model weights** (`.vrm`) are **not** committed. Place your own under `public/avatar/models/` — see that folder's README. Motions / room GLB ship with licenses under `THIRD_PARTY_ASSETS.md`.

---

## Install

### A. End users (recommended)

1. Open **[Releases](https://github.com/axioxmedia/Mnemos/releases)** and download the latest `Mnemos_*_x64-setup.exe`.  
2. Run the installer (Windows 10/11 x64, WebView2 usually preinstalled).  
3. Launch **忆灵 Mnemos**, finish onboarding, point at Ollama or a compatible API.  
4. Optional: open the **Avatar** window and load a VRM you have rights to use.

Maintainer staging copy (do **not** commit): see `PUBLISH.md` / sibling folder `Mnemos-github-assets\Mnemos_0.6.9_x64-setup.exe` → attach on the GitHub Release.

### B. Developers (from source)

**Requirements:** Node.js 20+, Rust stable, Python 3.10+, WebView2, Windows x64. Optional: [Ollama](https://ollama.com).

```powershell
git clone https://github.com/axioxmedia/Mnemos.git
cd Mnemos
npm install

# Sidecar
python -m venv sidecar\.venv
.\sidecar\.venv\Scripts\Activate.ps1
pip install -e .\sidecar
$env:MNEMOS_DATA_DIR = "$env:USERPROFILE\.mnemos"
python -m mnemos_sidecar
# listens on http://127.0.0.1:18765
```

Another terminal:

```powershell
npm run tauri:dev
# or UI-only: npm run dev  → http://localhost:1420
```

Production build:

```powershell
npm run tauri:build
# artifacts under src-tauri\target\release\bundle\
```

Smoke check (context-budget hard rule):

```powershell
python scripts\smoke_test.py
# expect: SMOKE OK
```

---

## Architecture

```text
┌───────────────────────────┐
│  Tauri 2 + React/TS UI    │  chat · memory hub · settings · avatar window
└─────────────┬─────────────┘
              │ localhost HTTP JSON
┌─────────────▼─────────────┐
│  Python sidecar           │  L0–L3 memory · retrieval · consolidate · plugins
└─────────────┬─────────────┘
              │
     ┌────────┼────────┐
     ▼        ▼        ▼
  Ollama   SQLite    Plugins
  / API    (+vec/FTS) (JSON-RPC)
```

Canonical summary: [`CANON.md`](./CANON.md) (full plan PDF in-repo).

---

## Documentation

| Doc | Description |
|:----|:------------|
| [docs/README-zh.md](./docs/README-zh.md) | Chinese twin (mirrored structure) |
| [CANON.md](./CANON.md) | Product canon / hard constraints |
| [RELEASE_NOTES.md](./RELEASE_NOTES.md) | Release notes |
| [resources/avatar/THIRD_PARTY_ASSETS.md](./resources/avatar/THIRD_PARTY_ASSETS.md) | Avatar motion / scene licenses |
| [docs/community-plugins.md](./docs/community-plugins.md) | Plugin notes |
| [PUBLISH.md](./PUBLISH.md) | Clean tree & Release staging |

### FAQ (short)

**Where are LLM weights?** Not in git. Use Ollama (`ollama pull …`) or your own API.  
**Where is the sample VRM?** Not redistributed here by default — drop your model into `public/avatar/models/`.  
**Why is the installer separate?** ~60MB+ binary; keep git lean and attach it on Releases.

---

## License

MIT License. See [LICENSE](./LICENSE).  
Third-party avatar motions/scenes keep their original CC/MIT notices (see `THIRD_PARTY_ASSETS.md`).

---

<div align="center">

Made for people who want a companion that **remembers carefully**, not endlessly.

[中文](./docs/README-zh.md) · [Releases](https://github.com/axioxmedia/Mnemos/releases) · [Canon](./CANON.md)

</div>
