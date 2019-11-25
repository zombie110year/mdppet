#![allow(dead_code)]
use regex::Regex;
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
    pub fn new(prefix: String, scope: String, body: Vec<String>, description: Vec<String>) -> Self {
        Snippet {
            prefix,
            scope,
            body,
            description,
        }
    }

    fn from_text(prefix: String, scope: String, body: String, description: String) -> Self {
        let body = body.trim_end();
        let description = description.trim_end();
        let mut body_v: Vec<String> = Vec::new();
        for i in body.split("\n") {
            body_v.push(String::from(i));
        }
        let mut description_v: Vec<String> = Vec::new();
        for i in description.split("\n") {
            description_v.push(String::from(i));
        }
        Snippet {
            prefix,
            scope,
            body: body_v,
            description: description_v,
        }
    }

    pub fn from_markdown(text: String) -> Self {
        let re = Regex::new(MARKDOWN_RE).unwrap();
        let m = re.captures(text.as_str()).unwrap();
        let prefix = String::from(m.name("prefix").unwrap().as_str());
        let scope = String::from(m.name("scope").unwrap().as_str());
        let body = String::from(m.name("body").unwrap().as_str());
        let description = String::from(m.name("description").unwrap().as_str());
        return Snippet::from_text(prefix, scope, body, description);
    }
}

pub fn get_snippet_segments<'a>(text: &'a String) -> Vec<&'a str> {
    let mut segments: Vec<&str> = Vec::new();
    let re = Regex::new(MARKDOWN_RE).unwrap();
    for segment in re.find_iter(text.as_str()) {
        segments.push(segment.as_str());
    }
    return segments;
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
    #[test]
    fn test_snip_from_markdown() {
        let mut text: String = String::new();

        {
            let md1_path = Path::new("tests/test_markdown_re_text.1.md");
            let md1_file = File::open(md1_path).unwrap();
            let mut md1_reader = BufReader::new(md1_file);
            md1_reader.read_to_string(&mut text).unwrap();
        }

        let snip = Snippet::from_markdown(text);
        assert_eq!(snip.prefix, String::from("hello"));
        assert_eq!(snip.scope, String::from("rust"));
        assert_eq!(snip.body, vec![String::from("println!(\"Hello World!\");")]);
        assert_eq!(snip.description, vec![String::from("Rust 的 HelloWorld 代码")]);
    }
}
