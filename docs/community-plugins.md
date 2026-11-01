# 社区插件索引

忆灵支持从**可配置 URL**拉取公开只读插件索引（JSON）。

```json
{
  "plugins": [
    {"id": "example.echo", "name": "Echo", "version": "1.0.0", "download": "https://.../echo.mnp"}
  ]
}
```

## 配置
- KV：`community_plugins_url`
- API：`GET /api/community/plugins?url=`

## 密钥交换
端到端互联网密钥交换**尚未实现**。当前仅公开索引；安装仍走本地 `.mnp` + ed25519 校验。

## 风格 LoRA
`style_lora_path` 指向本机适配器；不捆绑第三方权重。
