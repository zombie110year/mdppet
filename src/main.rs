use clap::{App, Arg};
use regex::Regex;
use serde_json::Result;
use serde::{Serialize, Deserialize};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::collections::BTreeMap;

const BIN_NAME: &str = "mdppet";

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
}
