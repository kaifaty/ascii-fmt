use crate::error::Result;
use crate::grid::GridMetrics;
use crate::parser::ParsedDiagram;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    Minimal,
    Standard,
    Detailed,
}

pub fn align_text_content(
    diagram: &mut ParsedDiagram,
    _metrics: &GridMetrics,
    style: Style,
) -> Result<()> {
    for line in &mut diagram.lines {
        if is_text_line(line) {
            *line = align_text_in_line(line, style);
        }
    }
    Ok(())
}

fn is_text_line(line: &str) -> bool {
    !line.chars().all(|c| matches!(c, '│' | '─' | '┌' | '┐' | '└' | '┘' | '├' | '┤' | '┬' | '┴' | '┼' | ' '))
}

fn align_text_in_line(line: &str, style: Style) -> String {
    let trimmed = line.trim();
    let text_len = UnicodeWidthStr::width(trimmed);
    let original_len = line.len();

    if original_len <= text_len {
        return line.to_string();
    }

    match style {
        Style::Minimal => format!("{}{}", trimmed, " ".repeat(original_len - text_len)),
        Style::Standard => format!("{}{}", trimmed, " ".repeat(original_len - text_len)),
        Style::Detailed => center_text(trimmed, original_len),
    }
}

fn center_text(text: &str, width: usize) -> String {
    let text_len = UnicodeWidthStr::width(text);
    if text_len >= width {
        return text.to_string();
    }

    let padding = width - text_len;
    let left_pad = padding / 2;
    let right_pad = padding - left_pad;

    format!("{}{}{}", " ".repeat(left_pad), text, " ".repeat(right_pad))
}
