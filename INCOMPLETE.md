# 未完成 / 深挖项（0.6.4）

## Breeze 真机
- Windows 上需拉齐权重 + 专用 tts-venv + CUDA torch；设定→声线→下载/准备模型，或 `scripts/setup_breeze_tts.ps1`
- 试听仅 ready 后走 breeze-tts-2；未就绪明示勿冒充
- 权重研究/非商用（BreezeBlue）

## 继承
CORS/watchdog 等见 0.6.2 说明。

## t59u AI 发文件 / 用户附件入记忆（目标 0.6.5 或同包若余量）
- Agent Tools：`files.write` / `files.deliver` → 本地附件库 + 前端气泡可下载/打开（代码/文档/json/md/pdf 等），勿只贴超长代码墙
- 历史用户上传：落盘 + L0/L2/L3 可检索元数据（文件名、类型、摘要、路径、会话 id）；大文件二进制不进 prompt，只指针/摘要
- 「还记得我发的 xxx.pdf？」→ 记忆优先检索 → 命中再次投递；未命中明确说没有
- 硬约束：成本恒定槽位；内容按需检索，不全量注入
