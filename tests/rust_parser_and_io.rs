use std::fs;
use std::path::PathBuf;

use markdown_reader::io::{read_text_with_fallback, write_text_utf8};
use markdown_reader::parser::parse_markdown;
use markdown_reader::{Block, ListKind, Span};

fn temp_file(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("md_reader_rust_{name}"))
}

#[test]
fn parses_heading_and_bold() {
    let blocks = parse_markdown("# 标题\n\n普通 **加粗** 文本");

    assert_eq!(blocks.len(), 2);
    match &blocks[0] {
        Block::Heading { level, spans } => {
            assert_eq!(*level, 1);
            assert_eq!(spans, &vec![Span::plain("标题")]);
        }
        _ => panic!("expected heading"),
    }
    match &blocks[1] {
        Block::Paragraph { spans } => {
            assert!(spans.iter().any(|s| s.bold && s.text == "加粗"));
        }
        _ => panic!("expected paragraph"),
    }
}

#[test]
fn parses_unordered_list() {
    let blocks = parse_markdown("- 第一项\n- 第二项");

    assert_eq!(blocks.len(), 1);
    match &blocks[0] {
        Block::List { kind, items } => {
            assert_eq!(kind, &ListKind::Unordered);
            assert_eq!(items.len(), 2);
            assert_eq!(items[0].spans[0].text, "第一项");
            assert_eq!(items[1].spans[0].text, "第二项");
        }
        _ => panic!("expected list"),
    }
}

#[test]
fn parses_code_block() {
    let blocks = parse_markdown("```rust\nprintln!(\"ok\");\n```");

    assert_eq!(blocks.len(), 1);
    match &blocks[0] {
        Block::CodeBlock { code } => assert!(code.contains("println!(\"ok\")")),
        _ => panic!("expected code block"),
    }
}

#[test]
fn does_not_treat_math_asterisk_as_italic() {
    let blocks = parse_markdown("2 * 3 * 4");

    assert_eq!(blocks.len(), 1);
    match &blocks[0] {
        Block::Paragraph { spans } => {
            assert!(!spans.iter().any(|s| s.italic));
            let text = spans.iter().map(|s| s.text.as_str()).collect::<String>();
            assert_eq!(text, "2 * 3 * 4");
        }
        _ => panic!("expected paragraph"),
    }
}

#[test]
fn reads_utf16_file() {
    let path = temp_file("utf16.md");
    // UTF-16LE with BOM for "你好"
    let utf16_bytes = vec![0xFF, 0xFE, 0x60, 0x4F, 0x7D, 0x59];
    fs::write(&path, utf16_bytes).expect("write temp file");

    let content = read_text_with_fallback(&path).expect("decode utf16");
    assert_eq!(content, "你好");

    let _ = fs::remove_file(path);
}

#[test]
fn parses_ordered_list() {
    let blocks = parse_markdown("1. one\n2. two");

    assert_eq!(blocks.len(), 1);
    match &blocks[0] {
        Block::List { kind, items } => {
            assert_eq!(kind, &ListKind::Ordered { start: 1 });
            assert_eq!(items.len(), 2);
            assert_eq!(items[0].spans[0].text, "one");
            assert_eq!(items[1].spans[0].text, "two");
        }
        _ => panic!("expected ordered list"),
    }
}

#[test]
fn parses_quote_block() {
    let blocks = parse_markdown("> 引用行");

    assert_eq!(blocks.len(), 1);
    match &blocks[0] {
        Block::Quote { blocks } => {
            assert_eq!(blocks.len(), 1);
            match &blocks[0] {
                Block::Paragraph { spans } => assert_eq!(spans[0].text, "引用行"),
                _ => panic!("expected paragraph in quote"),
            }
        }
        _ => panic!("expected quote"),
    }
}

#[test]
fn parses_link_and_image() {
    let blocks = parse_markdown("看这个[链接](https://example.com)和![图](img.png)");

    assert_eq!(blocks.len(), 1);
    match &blocks[0] {
        Block::Paragraph { spans } => {
            assert!(
                spans
                    .iter()
                    .any(|s| s.link.as_deref() == Some("https://example.com"))
            );
            assert!(spans.iter().any(|s| s.image.as_deref() == Some("img.png")));
        }
        _ => panic!("expected paragraph"),
    }
}

#[test]
fn parses_divider() {
    let blocks = parse_markdown("---");
    assert_eq!(blocks.len(), 1);
    assert!(matches!(blocks[0], Block::Divider));
}

#[test]
fn parses_task_list() {
    let blocks = parse_markdown("- [x] done\n- [ ] todo");

    assert_eq!(blocks.len(), 1);
    match &blocks[0] {
        Block::List { kind, items } => {
            assert_eq!(kind, &ListKind::Unordered);
            assert_eq!(items.len(), 2);
            assert_eq!(items[0].checked, Some(true));
            assert_eq!(items[1].checked, Some(false));
        }
        _ => panic!("expected list"),
    }
}

#[test]
fn parses_table() {
    let blocks = parse_markdown("| A | B |\n| --- | --- |\n| 1 | 2 |");

    assert_eq!(blocks.len(), 1);
    match &blocks[0] {
        Block::Table { headers, rows } => {
            assert_eq!(headers.len(), 2);
            assert_eq!(headers[0][0].text, "A");
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].len(), 2);
            assert_eq!(rows[0][1][0].text, "2");
        }
        _ => panic!("expected table"),
    }
}

#[test]
fn parses_strikethrough() {
    let blocks = parse_markdown("~~删除线~~");

    assert_eq!(blocks.len(), 1);
    match &blocks[0] {
        Block::Paragraph { spans } => {
            assert!(spans.iter().any(|s| s.strike));
        }
        _ => panic!("expected paragraph"),
    }
}

#[test]
fn writes_and_reads_utf8_file() {
    let path = temp_file("write_utf8.md");
    let content = "# 标题\n\n这是编辑后保存的内容。";

    write_text_utf8(&path, content).expect("write utf8 file");
    let loaded = read_text_with_fallback(&path).expect("read utf8 file");
    assert_eq!(loaded, content);

    let _ = fs::remove_file(path);
}
