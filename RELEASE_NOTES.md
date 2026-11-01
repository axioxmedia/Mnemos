## 0.6.8+ packaging — release\tauri 旧 setup 改为直接删除（不再 archive）（2026-10-31 Asia/Shanghai）

- 打包脚本 `scripts/build_tauri_windows.ps1`：`release\tauri` 只保留最新 `Mnemos_*_x64-setup.exe`；旧 setup / 重复中文包 / 多余 build log **Remove-Item 删除**，禁止 mkdir archive、禁止 Move-Item 归档。

## 0.6.7 — 形象窗女性 idle ×6 + 日式房间 + 双模型修复（2026-10-31 Asia/Shanghai）

- **Idle**：替换为 ACCAD Female1 站立系 VRMA×6（Stand/Sway/SwayT2/Wait/Look/Swing，CC BY 3.0），随机 crossfade；观感偏女性站姿。
- **房间**：去掉 hotel_room HDR；改加载 Poly Pizza JapaneseRoom GLB（CC BY，Aimi Sekiguchi）带家具轻量室内。
- **双模型**：loadVrm 加 generation token；同 URI 去重；openAvatarWindow 已开窗不再 spam vatar-init。
- 脸追眼神跟随仍未做。
# 蹇嗙伒 Mnemos 0.6.5

## VRM 褰㈣薄绐?- 淇闈欑疆浠嶅憟 T-pose锛氱▼搴忓寲 idle 琛ヤ笂 A-pose 钀借噦 + 鍛煎惛/杞诲井鎽嗗姩锛涚湪鐪间笌浜掓枼鍞囧舰淇濈暀
- 璇磋瘽鏃?idle 鍔ㄦ€佸帇浣庯紝钀借噦濮挎€佷繚鐣?
## 瀹夎
`Mnemos_0.6.5_x64-setup.exe`锛沗/S` 闈欓粯銆傝鍓嶉€€鍑烘棫杩涚▼銆?

