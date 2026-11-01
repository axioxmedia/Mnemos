# t59u AI 发文件（设计草稿，0.6.5）

## 目标
1. AI 必要时投递文件；聊天气泡附件可下载/打开。
2. Tools：`files.write` / `files.deliver`（写本地附件库并回传前端）。
3. 用户历史上传（含 pdf）落盘 + L0/L2/L3 元数据可检索；prompt 只塞指针/摘要。
4. 「还记得我发的策划案 pdf？」→ 记忆检索 → 命中再投递；未命中明示。
5. 成本恒定槽位；按需检索。

## 建议落点
- `sidecar/.../agent_tools/` 增加 files 工具
- 附件库：`{data}/attachments/{session_id}/...` + sqlite/kv 元数据表
- HTTP：`GET /api/attachments/{id}` 下载；chat SSE 事件 `attachment`
- 前端：MessageBubble 附件卡片（文件名、大小、打开/下载）
- 记忆：ingest 时写 L0 事件 + L2/L3 摘要向量；检索命中后 `files.deliver(path)`

## 版本
0.6.4 优先：停止生成 + 声线分页 + 真 Breeze。本项默认进 0.6.5；若 0.6.4 封版前有余量可同包。
