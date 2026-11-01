忆灵 Mnemos — 便携版说明（临时 pywebview + PyInstaller；正典壳仍为 Tauri 2）

双击：release\Mnemos\Mnemos.exe

本版更新（0.2）：
1) 全链路 UTF-8，修复「寒暄」等中文变成 ???? 的问题；L0 跳过明显坏行。
2) 对话改为流式 /api/chat/stream；DeepSeek R1 思考过程可折叠展示（「思考中」卡片）。
3) 冷启动显示「正在加载模型…」心跳，模型未安装立即 Mock/报错，不再空等约 8 分钟。
4) 白色/深色主题（设定里切换，默认白色，记住上次选择）。
5) 本地捆绑 Font Awesome 7 图标。
6) 设定页：Skill 库（导入/开关/删除）、角色设定、插件库（导入导出/案例插件/制作插件 Skill）。
7) 记忆图谱（力导向动效，顶栏图标打开）。

数据目录：D:\GrokAssets\Mnemos\data
Ollama：仅使用 D:\GrokAssets\Mnemos\ollama（不要装到 C:\Users\...\Programs\Ollama）
导出目录：D:\GrokAssets\Mnemos\export

硬约束未改：固定槽位预算；插件可扩展且权限隔离。
