# Excluded from the clean GitHub tree

Confirm before first push. Sizes are approximate from the working tree `D:\GrokAssets\Mnemos`.

## Models & runtimes
| Path / pattern | ~Size | Reason |
|:---------------|------:|:-------|
| `ollama/` | ~42 GB | Local LLM weights / blobs |
| `models/` (breeze-tts, …) | varies | TTS / vendor weights |
| `*.gguf` `*.safetensors` `*.onnx` (bulk) | — | Model weights |
| `tts-venv/` `.tools/` | ~1 GB+ | Local toolchains / caches |
| `vendor/breeze-tts/` | — | Vendor TTS checkout |

## Build / deps
| Path | ~Size | Reason |
|:-----|------:|:-------|
| `node_modules/` | ~105 MB | Reinstall via `npm install` |
| `src-tauri/target/` | ~3.2 GB | Rust build |
| `src-tauri/binaries/*.exe` | ~100 MB | Packaged sidecar binaries |
| `dist/` | ~9 MB | Vite output |
| `sidecar/.venv/` | ~140 MB | Python venv |
| `packaging/build/` | — | PyInstaller scratch |

## User data & secrets
| Path | Reason |
|:-----|:-------|
| `data/` (incl. `Alicia.vrm`, `*.db`, chats) | Personal / runtime data |
| `.env` / API keys | Secrets |
| Root `*.log` `*.pid` scratch scripts | Local noise |

## Installers
| Path | ~Size | Where instead |
|:-----|------:|:--------------|
| `release/tauri/Mnemos_*_x64-setup.exe` | ~62 MB | GitHub **Releases** (`Mnemos-github-assets` staging) |
| Old setups / archive | — | Deleted; never re-archive into repo |

## Intentionally kept (with license notes)
- `public/avatar/motions/*.vrma` + `LICENSE` (ACCAD → VRMA, CC BY)
- `public/avatar/scenes/anime_room/japaneseroom.glb` (CC BY)
- Vendored Three / three-vrm JS under `public/avatar/vendor/`
- `docs/APPCap.png` (placeholder screenshot — replace before marketing push)
- Plan PDF + `CANON.md`

## Not shipped as sample avatar
- `Alicia.vrm` (~7.5 MB) left out of git by default; place under `public/avatar/models/` yourself if license allows.
