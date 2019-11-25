mod snip;

use clap::{App, Arg};
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

const BIN_NAME: &str = "mdppet";

fn main() {
    let args = get_app().get_matches();
    let src = args.value_of("src").unwrap();
    let out = args.value_of("dest").unwrap();
    let mut text = String::new();
    let mut snips: Vec<snip::Snippet> = Vec::new();

    let mut istream = get_read_stream(Path::new(src));
    istream.read_to_string(&mut text).unwrap();
    for md in snip::get_snippet_segments(&text) {
        snips.push(snip::Snippet::from_markdown(md));
    }

    let mut json_buffer: BTreeMap<&str, &snip::SnippetBody> = BTreeMap::new();
    let mut ostream = fs::File::create(Path::new(out)).ok().unwrap();
    for i in snips.iter() {
        let id = i.get_identifier().as_str();
        let body = i.get_snippetbody();
        json_buffer.insert(id, body);
    }

    let serielized_text = serde_json::to_string_pretty(&json_buffer).ok().unwrap();
    write!(&mut ostream, "{}", serielized_text).ok().unwrap();
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

fn get_read_stream(file: &Path) -> io::BufReader<fs::File> {
    let ifile = fs::File::open(file).ok().unwrap();
    let istream = io::BufReader::new(ifile);
    return istream;
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
