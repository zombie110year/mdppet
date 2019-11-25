# mdppet

mdppet 是一个将可读性更好的 Markdown 文档解析为 vscode snippet JSON 的工具。

Markdown 满足以下格式

## mdppet syntax

```markdown
# 标识符/前缀/作用域

描述

```rust
补全
```
```

将会转换成

```json
{
    "标识符": {
        "prefix": "前缀",
        "scope": "作用域",
        "body": [
            "补全"
        ],
        "description": [
            "描述"
        ]
    }
}
```
