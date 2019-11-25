use clap::{App, Arg};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

const BIN_NAME: &str = "mdppet";

/// 用于匹配 Markdown 中一个 Snippet 片段的正则表达式
///
/// - `\x20` 表示空格 ` `
/// - `\x23` 表示 `#`
///
/// 以上字符由于和正则引擎冲突，因此使用转义表达法
const markdown_re_text: &str = r#"((?msx)
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

fn main() {
    let args = get_app().get_matches();
    let src = args.value_of("src").unwrap();
    let out = args.value_of("dest").unwrap();
    println!("{} -> {}", src, out);
}

fn get_app() -> App<'static, 'static> {
    let parser = App::new(BIN_NAME)
        .about("mdppet is a tool to transfer markdown to vscode snippet json.")
        .version("0.1.0")
        .author("zombie110year <zombie110year@outlook.com>")
        .arg(Arg::with_name("src").required(true))
        .arg(Arg::with_name("dest").short("o").default_value("out.json"));

    return parser;
}

fn get_read_stream(file: &PathBuf) -> io::BufReader<fs::File> {
    let ifile = fs::File::open(file).ok().unwrap();
    let istream = io::BufReader::new(ifile);
    return istream;
}

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
    scope: Vec<String>,
    body: Vec<String>,
    description: Vec<String>,
}

impl Snippet {
    pub fn new(
        prefix: String,
        scope: Vec<String>,
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
    #[test]
    fn test_arg_parse() {
        let parser = get_app();
        let matches = parser.get_matches_from([BIN_NAME, "source.md", "-o", "output.json"].iter());
        let src = matches.value_of("src").expect("无法获取到 src 参数的值");
        let out = matches.value_of("dest").expect("无法获取到 dest 参数的值");
        assert_eq!(src, "source.md");
        assert_eq!(out, "output.json");
    }
    #[test]
    fn test_markdown_re() {
        let re = Regex::new(markdown_re_text).unwrap();
        let mut text: String = String::new();

        {
            let md1_path = Path::new("tests/test_markdown_re_text.1.md");
            let md1_file = fs::File::open(md1_path).unwrap();
            let mut md1_reader = io::BufReader::new(md1_file);
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
