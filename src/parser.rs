use crate::{Block, ListItem, ListKind, Span};

pub fn parse_markdown(markdown_text: &str) -> Vec<Block> {
    let lines: Vec<&str> = markdown_text.lines().collect();
    let mut blocks: Vec<Block> = Vec::new();
    let mut i = 0usize;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        if trimmed.is_empty() {
            i += 1;
            continue;
        }

        if trimmed.starts_with("```") {
            let mut code_lines = Vec::new();
            i += 1;
            while i < lines.len() {
                let current = lines[i];
                if current.trim().starts_with("```") {
                    i += 1;
                    break;
                }
                code_lines.push(current.to_owned());
                i += 1;
            }
            blocks.push(Block::CodeBlock {
                code: code_lines.join("\n"),
            });
            continue;
        }

        if let Some((level, content)) = parse_heading_line(trimmed) {
            blocks.push(Block::Heading {
                level,
                spans: parse_inline(content),
            });
            i += 1;
            continue;
        }

        if is_divider_line(trimmed) {
            blocks.push(Block::Divider);
            i += 1;
            continue;
        }

        if stripped_quote_content(line).is_some() {
            let mut quote_lines = Vec::new();
            while i < lines.len() {
                if let Some(content) = stripped_quote_content(lines[i]) {
                    quote_lines.push(content.to_owned());
                    i += 1;
                } else {
                    break;
                }
            }
            blocks.push(Block::Quote {
                blocks: parse_markdown(&quote_lines.join("\n")),
            });
            continue;
        }

        if i + 1 < lines.len() && looks_like_table_header(lines[i], lines[i + 1]) {
            let header_cells = split_table_row(lines[i])
                .into_iter()
                .map(|cell| parse_inline(cell.trim()))
                .collect::<Vec<_>>();
            i += 2;

            let mut rows = Vec::new();
            while i < lines.len() {
                let row_line = lines[i];
                if row_line.trim().is_empty() || !row_line.contains('|') {
                    break;
                }
                let cells = split_table_row(row_line)
                    .into_iter()
                    .map(|cell| parse_inline(cell.trim()))
                    .collect::<Vec<_>>();
                rows.push(cells);
                i += 1;
            }

            blocks.push(Block::Table {
                headers: header_cells,
                rows,
            });
            continue;
        }

        if let Some((start, _)) = parse_ordered_item(trimmed) {
            let mut items = Vec::new();
            while i < lines.len() {
                let current = lines[i].trim();
                if let Some((_n, content)) = parse_ordered_item(current) {
                    items.push(ListItem {
                        spans: parse_inline(content),
                        checked: None,
                    });
                    i += 1;
                } else {
                    break;
                }
            }
            blocks.push(Block::List {
                kind: ListKind::Ordered { start },
                items,
            });
            continue;
        }

        if parse_unordered_item(trimmed).is_some() {
            let mut items = Vec::new();
            while i < lines.len() {
                let current = lines[i].trim();
                if let Some((checked, content)) = parse_unordered_item(current) {
                    items.push(ListItem {
                        spans: parse_inline(content),
                        checked,
                    });
                    i += 1;
                } else {
                    break;
                }
            }
            blocks.push(Block::List {
                kind: ListKind::Unordered,
                items,
            });
            continue;
        }

        let mut paragraph_lines = Vec::new();
        while i < lines.len() {
            let current = lines[i].trim();
            if current.is_empty() || is_block_start(&lines, i) {
                break;
            }
            paragraph_lines.push(current.to_owned());
            i += 1;
        }

        if !paragraph_lines.is_empty() {
            blocks.push(Block::Paragraph {
                spans: parse_inline(&paragraph_lines.join(" ")),
            });
        } else {
            i += 1;
        }
    }

    blocks
}

fn parse_heading_line(line: &str) -> Option<(u8, &str)> {
    let mut hashes = 0usize;
    for c in line.chars() {
        if c == '#' {
            hashes += 1;
        } else {
            break;
        }
    }
    if !(1..=6).contains(&hashes) {
        return None;
    }
    let rest = line.get(hashes..)?.trim_start();
    if rest.is_empty() {
        return None;
    }
    Some((hashes as u8, rest))
}

fn parse_ordered_item(line: &str) -> Option<(u64, &str)> {
    let dot = line.find(". ")?;
    let num = line.get(..dot)?.parse::<u64>().ok()?;
    let content = line.get(dot + 2..)?.trim();
    Some((num, content))
}

fn parse_unordered_item(line: &str) -> Option<(Option<bool>, &str)> {
    let mut chars = line.chars();
    let marker = chars.next()?;
    if marker != '-' && marker != '*' && marker != '+' {
        return None;
    }
    let after = chars.as_str();
    if !after.starts_with(' ') {
        return None;
    }
    let content = after.trim_start();

    let lower = content.to_ascii_lowercase();
    if lower.starts_with("[x] ") {
        return Some((Some(true), content.get(4..)?.trim()));
    }
    if lower.starts_with("[ ] ") {
        return Some((Some(false), content.get(4..)?.trim()));
    }

    Some((None, content))
}

fn is_divider_line(line: &str) -> bool {
    let clean = line.replace(' ', "");
    clean == "---" || clean == "***" || clean == "___"
}

fn stripped_quote_content(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    let rest = trimmed.strip_prefix('>')?;
    Some(rest.strip_prefix(' ').unwrap_or(rest))
}

fn looks_like_table_header(header: &str, sep: &str) -> bool {
    if !header.contains('|') {
        return false;
    }
    let cells = split_table_row(sep);
    if cells.is_empty() {
        return false;
    }
    cells.iter().all(|cell| {
        let c = cell.trim().replace([':', '-'], "");
        c.is_empty() && cell.contains('-')
    })
}

fn split_table_row(line: &str) -> Vec<String> {
    let mut s = line.trim();
    if s.starts_with('|') {
        s = &s[1..];
    }
    if s.ends_with('|') {
        s = &s[..s.len().saturating_sub(1)];
    }
    s.split('|').map(|x| x.trim().to_owned()).collect()
}

fn is_block_start(lines: &[&str], idx: usize) -> bool {
    let trimmed = lines[idx].trim();
    if trimmed.starts_with("```")
        || parse_heading_line(trimmed).is_some()
        || is_divider_line(trimmed)
        || stripped_quote_content(lines[idx]).is_some()
        || parse_ordered_item(trimmed).is_some()
        || parse_unordered_item(trimmed).is_some()
    {
        return true;
    }
    idx + 1 < lines.len() && looks_like_table_header(lines[idx], lines[idx + 1])
}

fn parse_inline(text: &str) -> Vec<Span> {
    let mut spans = Vec::new();
    let mut plain = String::new();
    let mut idx = 0usize;

    let flush_plain = |spans: &mut Vec<Span>, plain: &mut String| {
        if !plain.is_empty() {
            spans.push(Span::plain(std::mem::take(plain)));
        }
    };

    while idx < text.len() {
        if let Some((next, alt, url)) = parse_image(text, idx) {
            flush_plain(&mut spans, &mut plain);
            spans.push(Span {
                text: alt,
                bold: false,
                italic: false,
                code: false,
                strike: false,
                link: None,
                image: Some(url),
            });
            idx = next;
            continue;
        }

        if let Some((next, label, url)) = parse_link(text, idx) {
            flush_plain(&mut spans, &mut plain);
            spans.push(Span {
                text: label,
                bold: false,
                italic: false,
                code: false,
                strike: false,
                link: Some(url),
                image: None,
            });
            idx = next;
            continue;
        }

        if let Some(end) = find_wrapped(text, idx, "`", "`", false) {
            flush_plain(&mut spans, &mut plain);
            let inner = &text[idx + 1..end];
            if !inner.is_empty() {
                spans.push(Span {
                    text: inner.to_owned(),
                    bold: false,
                    italic: false,
                    code: true,
                    strike: false,
                    link: None,
                    image: None,
                });
            }
            idx = end + 1;
            continue;
        }

        if let Some(end) = find_wrapped(text, idx, "**", "**", false) {
            flush_plain(&mut spans, &mut plain);
            let inner = &text[idx + 2..end];
            if !inner.is_empty() {
                spans.push(Span {
                    text: inner.to_owned(),
                    bold: true,
                    italic: false,
                    code: false,
                    strike: false,
                    link: None,
                    image: None,
                });
            }
            idx = end + 2;
            continue;
        }

        if let Some(end) = find_wrapped(text, idx, "~~", "~~", false) {
            flush_plain(&mut spans, &mut plain);
            let inner = &text[idx + 2..end];
            if !inner.is_empty() {
                spans.push(Span {
                    text: inner.to_owned(),
                    bold: false,
                    italic: false,
                    code: false,
                    strike: true,
                    link: None,
                    image: None,
                });
            }
            idx = end + 2;
            continue;
        }

        if let Some(end) = find_wrapped(text, idx, "*", "*", true) {
            let inner = &text[idx + 1..end];
            if !inner.is_empty()
                && !inner.chars().next().is_some_and(char::is_whitespace)
                && !inner.chars().next_back().is_some_and(char::is_whitespace)
            {
                flush_plain(&mut spans, &mut plain);
                spans.push(Span {
                    text: inner.to_owned(),
                    bold: false,
                    italic: true,
                    code: false,
                    strike: false,
                    link: None,
                    image: None,
                });
                idx = end + 1;
                continue;
            }
        }

        if let Some(ch) = text[idx..].chars().next() {
            plain.push(ch);
            idx += ch.len_utf8();
        } else {
            break;
        }
    }

    flush_plain(&mut spans, &mut plain);
    spans
}

fn parse_image(text: &str, start: usize) -> Option<(usize, String, String)> {
    if !text[start..].starts_with("![") {
        return None;
    }
    parse_link_like(text, start, true)
}

fn parse_link(text: &str, start: usize) -> Option<(usize, String, String)> {
    if !text[start..].starts_with('[') {
        return None;
    }
    parse_link_like(text, start, false)
}

fn parse_link_like(text: &str, start: usize, image: bool) -> Option<(usize, String, String)> {
    let label_start = start + if image { 2 } else { 1 };
    let close_label_rel = text[label_start..].find(']')?;
    let close_label = label_start + close_label_rel;
    let open_url = close_label + 1;
    if text.get(open_url..open_url + 1)? != "(" {
        return None;
    }
    let url_start = open_url + 1;
    let close_url_rel = text[url_start..].find(')')?;
    let close_url = url_start + close_url_rel;

    let label = text[label_start..close_label].to_owned();
    let url = text[url_start..close_url].to_owned();
    Some((close_url + 1, label, url))
}

fn find_wrapped(
    text: &str,
    start: usize,
    prefix: &str,
    suffix: &str,
    require_non_space_after_prefix: bool,
) -> Option<usize> {
    if !text[start..].starts_with(prefix) {
        return None;
    }

    let content_start = start + prefix.len();
    if require_non_space_after_prefix {
        let next = text[content_start..].chars().next()?;
        if next.is_whitespace() {
            return None;
        }
    }

    let mut search_from = content_start;
    loop {
        let rel = text[search_from..].find(suffix)?;
        let end = search_from + rel;
        if end > content_start {
            return Some(end);
        }
        search_from = end + suffix.len();
    }
}
