#![allow(dead_code)]
use serde::Serialize;

/// 用于匹配 Markdown 中一个 Snippet 片段的正则表达式
///
/// - `\x20` 表示空格 ` `
/// - `\x23` 表示 `#`
///
/// 以上字符由于和正则引擎冲突，因此使用转义表达法
const MARKDOWN_RE: &str = r#"((?msx)
\x23\x20(?P<id>\S+)/(?P<prefix>\S+)/(?P<scope>\S+)
\n+
(?P<description>
  (?:[^\n]+\n)+
)
\n+
```(?:\S+)?\n
(?P<body>.+)
```
)
$"#;

/// # Snippet
///
/// 一个 Snippet 对象，具有
///
/// - 前缀: prefix
/// - 作用域: scope
/// - 补全体: body
/// - 描述: description
///
/// 四条属性
#[derive(Serialize)]
pub struct Snippet {
    prefix: String,
    scope: String,
    body: Vec<String>,
    description: Vec<String>,
}

impl Snippet {
    pub fn new(
        prefix: String,
        scope: String,
        body: Vec<String>,
        description: Vec<String>,
    ) -> Self {
        Snippet {
            prefix,
            scope,
            body,
            description,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::Read;
    use std::path::Path;
    #[test]
    fn test_markdown_re() {
        let re = Regex::new(MARKDOWN_RE).unwrap();
        let mut text: String = String::new();

        {
            let md1_path = Path::new("tests/test_markdown_re_text.1.md");
            let md1_file = File::open(md1_path).unwrap();
            let mut md1_reader = BufReader::new(md1_file);
            md1_reader.read_to_string(&mut text).unwrap();
        }

        assert_eq!(re.is_match(text.as_str()), true);

        let m = re.captures(text.as_str()).unwrap();
        assert_eq!(m.name("id").unwrap().as_str(), "hello");
        assert_eq!(m.name("prefix").unwrap().as_str(), "hello");
        assert_eq!(m.name("scope").unwrap().as_str(), "rust");
        assert_eq!(
            m.name("description").unwrap().as_str(),
            "Rust 的 HelloWorld 代码\n"
        );
        assert_eq!(
            m.name("body").unwrap().as_str(),
            "println!(\"Hello World!\");\n"
        );
    }
}
