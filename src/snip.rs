#![allow(dead_code)]
use regex::Regex;
use serde::Serialize;
use std::fmt::Debug;

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
(?P<body>.+?)
```
)"#;

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
///
/// 推荐使用 `from_markdown` 方法实例化。
///
/// ## 示例
///
/// ```rust
/// let markdown = r#"
/// # a/b/rust
/// description
/// ```rust
/// body
/// ```
/// "#;
///
/// let snip = Snippet::from_markdown(markdown);
/// assert_eq!(snip.get_identifier(), &String::from("a"));
/// assert_eq!(snip.get_prefix(), &String::from("b"));
/// assert_eq!(snip.get_scope(), &String::from("rust"));
/// assert_eq!(snip.get_description(), &vec![String::from("description")]);
/// assert_eq!(snip.get_body(), &vec![String::from("body")]);
/// ```
#[derive(Debug)]
pub struct Snippet {
    identifier: String,
    body: SnippetBody,
}

#[derive(Serialize, Debug)]
pub struct SnippetBody {
    prefix: String,
    scope: String,
    body: Vec<String>,
    description: Vec<String>,
}

impl Snippet {
    pub fn new(
        identifier: &str,
        prefix: &str,
        scope: &str,
        body: &Vec<&str>,
        description: &Vec<&str>,
    ) -> Self {
        let identifier_new = String::from(identifier);
        let prefix_new = String::from(prefix);
        let scope_new = String::from(scope);
        let mut body_new: Vec<String> = Vec::new();
        let mut description_new: Vec<String> = Vec::new();

        for i in body.iter() {
            body_new.push(String::from(*i));
        }
        for i in description.iter() {
            description_new.push(String::from(*i));
        }

        let body = SnippetBody::new(prefix_new, scope_new, body_new, description_new);
        Snippet {
            identifier: identifier_new,
            body,
        }
    }

    pub fn from_text(
        identifier: &str,
        prefix: &str,
        scope: &str,
        body: &str,
        description: &str,
    ) -> Self {
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

        let body = SnippetBody::new(
            String::from(prefix),
            String::from(scope),
            body_v,
            description_v,
        );
        Snippet {
            identifier: String::from(identifier),
            body,
        }
    }

    pub fn from_markdown(text: &str) -> Self {
        let re = Regex::new(MARKDOWN_RE).unwrap();
        let m = re.captures(text).unwrap();
        let id = m.name("id").unwrap().as_str();
        let prefix = m.name("prefix").unwrap().as_str();
        let scope = m.name("scope").unwrap().as_str();
        let body = m.name("body").unwrap().as_str();
        let description = m.name("description").unwrap().as_str();
        return Snippet::from_text(id, prefix, scope, body, description);
    }

    pub fn get_identifier(&self) -> &String {
        return &self.identifier;
    }
    pub fn get_snippetbody(&self) -> &SnippetBody {
        return &self.body;
    }
    pub fn get_prefix(&self) -> &String {
        return &self.body.prefix;
    }
    pub fn get_scope(&self) -> &String {
        return &self.body.scope;
    }
    pub fn get_body(&self) -> &Vec<String> {
        return &self.body.body;
    }
    pub fn get_description(&self) -> &Vec<String> {
        return &self.body.description;
    }
}

impl SnippetBody {
    pub fn new(prefix: String, scope: String, body: Vec<String>, description: Vec<String>) -> Self {
        SnippetBody {
            prefix,
            scope,
            body,
            description,
        }
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
        let text = read_text("tests/test_markdown_re_text.1.md");

        let snip = Snippet::from_markdown(text.as_str());
        assert_eq!(snip.get_identifier(), &String::from("hello"));
        assert_eq!(snip.get_prefix(), &String::from("hello"));
        assert_eq!(snip.get_scope(), &String::from("rust"));
        assert_eq!(
            snip.get_body(),
            &vec![String::from("println!(\"Hello World!\");")]
        );
        assert_eq!(
            snip.get_description(),
            &vec![String::from("Rust 的 HelloWorld 代码")]
        );
    }
    #[test]
    fn test_multi_snip_markdown_1() {
        let text: String = read_text("tests/test_markdown.2.md");

        let snips: Vec<Snippet> = get_snippet_segments(&text)
            .iter()
            .map(|&md_text| Snippet::from_markdown(md_text))
            .collect();

        let snip1 = &snips[0];
        assert_eq!(snip1.get_identifier().as_str(), "a");
        assert_eq!(snip1.get_prefix().as_str(), "b");
        assert_eq!(snip1.get_scope().as_str(), "c");
        assert_eq!(
            snip1
                .get_description()
                .iter()
                .map(|line| line.as_str())
                .collect::<Vec<&str>>(),
            vec!["desp"]
        );
        assert_eq!(
            snip1
                .get_body()
                .iter()
                .map(|line| line.as_str())
                .collect::<Vec<&str>>(),
            vec!["println!(\"Hello\");"]
        );
    }
    #[test]
    fn test_multi_snip_markdown_2() {
        let text: String = read_text("tests/test_markdown.2.md");

        let snips: Vec<Snippet> = get_snippet_segments(&text)
            .iter()
            .map(|&md_text| Snippet::from_markdown(md_text))
            .collect();

        let snip1 = &snips[1];
        assert_eq!(snip1.get_identifier().as_str(), "e");
        assert_eq!(snip1.get_prefix().as_str(), "f");
        assert_eq!(snip1.get_scope().as_str(), "g");
        assert_eq!(
            snip1
                .get_description()
                .iter()
                .map(|line| line.as_str())
                .collect::<Vec<&str>>(),
            vec!["desp2", "desp2"]
        );
        assert_eq!(
            snip1
                .get_body()
                .iter()
                .map(|line| line.as_str())
                .collect::<Vec<&str>>(),
            vec!["abc"]
        );
    }
    #[test]
    fn test_multi_snip_markdown_3() {
        let text: String = read_text("tests/test_markdown.2.md");

        let snips: Vec<Snippet> = get_snippet_segments(&text)
            .iter()
            .map(|&md_text| Snippet::from_markdown(md_text))
            .collect();

        let snip1 = &snips[2];
        assert_eq!(snip1.get_identifier().as_str(), "abc");
        assert_eq!(snip1.get_prefix().as_str(), "123");
        assert_eq!(snip1.get_scope().as_str(), "python,lua");
        assert_eq!(
            snip1
                .get_description()
                .iter()
                .map(|line| line.as_str())
                .collect::<Vec<&str>>(),
            vec!["desp3", "desp4"]
        );
        assert_eq!(
            snip1
                .get_body()
                .iter()
                .map(|line| line.as_str())
                .collect::<Vec<&str>>(),
            vec!["print(\"Hello1\")", "print(\"Hello2\")"]
        );
    }

    fn read_text(path: &str) -> String {
        let mut text: String = String::new();
        {
            let md_path = Path::new(path);
            let md_file = File::open(md_path).unwrap();
            let mut reader = BufReader::new(md_file);
            reader.read_to_string(&mut text).unwrap();
        }
        return text;
    }
}
