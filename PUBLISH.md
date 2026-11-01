# Publishing notes (maintainers)

## Layout on this machine

| Path | Purpose |
|:-----|:--------|
| `D:\GrokAssets\Mnemos-github\` | Clean source tree ready to `git push` |
| `D:\GrokAssets\Mnemos-github-assets\Mnemos_0.6.9_x64-setup.exe` | Installer for **GitHub Release** assets only |
| `D:\GrokAssets\Mnemos\` | Working / build tree (models, ollama, target, …) — **not** for public git |

## Do

1. Review `EXCLUDED.md` and `.gitignore`.
2. Replace `docs/APPCap.png` with a current screenshot if needed.
3. Declare `LICENSE` before going public.
4. `git remote add origin https://github.com/axioxmedia/Mnemos.git` (create empty repo first if needed).
5. Push **source only**; upload the setup exe on the Release page.

## Don't

- Commit `ollama/`, `*.gguf`, `*.safetensors`, `data/`, `node_modules/`, `src-tauri/target/`, TTS weights, or setup `.exe`.
- Invent download-count or "Packed via" badges.
