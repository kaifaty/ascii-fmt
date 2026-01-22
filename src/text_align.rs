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
    let (indent, rest) = split_leading_whitespace(line);
    let trimmed = rest.trim();
    let text_len = UnicodeWidthStr::width(trimmed);
    let original_len = UnicodeWidthStr::width(rest);

    if trimmed.is_empty() {
        return String::new();
    }

    if original_len <= text_len {
        return line.to_string();
    }

    match style {
        Style::Minimal => format!(
            "{}{}{}",
            indent,
            trimmed,
            " ".repeat(original_len - text_len)
        ),
        Style::Standard => format!(
            "{}{}{}",
            indent,
            trimmed,
            " ".repeat(original_len - text_len)
        ),
        Style::Detailed => format!("{}{}", indent, center_text(trimmed, original_len)),
    }
}

fn split_leading_whitespace(s: &str) -> (&str, &str) {
    let mut split_at = 0;
    for (idx, ch) in s.char_indices() {
        if ch != ' ' && ch != '\t' {
            split_at = idx;
            break;
        }
        split_at = idx + ch.len_utf8();
    }
    s.split_at(split_at)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_equality() {
        assert_eq!(Style::Minimal, Style::Minimal);
        assert_eq!(Style::Standard, Style::Standard);
        assert_eq!(Style::Detailed, Style::Detailed);
        assert_ne!(Style::Minimal, Style::Standard);
        assert_ne!(Style::Standard, Style::Detailed);
        assert_ne!(Style::Detailed, Style::Minimal);
    }

    #[test]
    fn test_style_copy_and_clone() {
        let style = Style::Standard;
        let style_copy = style;
        assert_eq!(style, style_copy);
        let style_clone = style.clone();
        assert_eq!(style, style_clone);
    }

    #[test]
    fn test_align_text_content_simple() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "  hello  ".to_string(),
                "  world  ".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let metrics = GridMetrics {
            column_width: 2,
            row_height: 1,
            columns: vec![],
            rows: vec![],
        };

        assert!(align_text_content(&mut diagram, &metrics, Style::Minimal).is_ok());
    }

    #[test]
    fn test_align_text_content_with_box_lines() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "┌─────┐".to_string(),
                "│ hello │".to_string(),
                "└─────┘".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let metrics = GridMetrics {
            column_width: 2,
            row_height: 1,
            columns: vec![],
            rows: vec![],
        };

        assert!(align_text_content(&mut diagram, &metrics, Style::Standard).is_ok());
    }

    #[test]
    fn test_align_text_content_empty_diagram() {
        let mut diagram = ParsedDiagram {
            lines: vec![],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let metrics = GridMetrics {
            column_width: 2,
            row_height: 1,
            columns: vec![],
            rows: vec![],
        };

        assert!(align_text_content(&mut diagram, &metrics, Style::Standard).is_ok());
    }

    #[test]
    fn test_is_text_line_with_text() {
        assert!(is_text_line("hello world"));
        assert!(is_text_line("  indented text  "));
        assert!(is_text_line("text with │ symbols"));
        assert!(is_text_line("a"));
    }

    #[test]
    fn test_is_text_line_false() {
        assert!(!is_text_line("│"));
        assert!(!is_text_line("─"));
        assert!(!is_text_line("┌┐└┘"));
        assert!(!is_text_line("├┤┬┴┼"));
        assert!(!is_text_line("     "));
        assert!(!is_text_line(""));
    }

    #[test]
    fn test_is_text_line_mixed() {
        assert!(is_text_line("│ hello │"));
        assert!(is_text_line("── hello ──"));
        assert!(is_text_line("┌ hello ┐"));
    }

    #[test]
    fn test_is_text_line_only_box_drawing() {
        assert!(!is_text_line("│─┌┐└┘├┤┬┴┼ "));
        assert!(!is_text_line("│││"));
        assert!(!is_text_line("────"));
    }

    #[test]
    fn test_align_text_in_line_minimal_style() {
        let result = align_text_in_line("  hello  ", Style::Minimal);
        assert_eq!(result, "  hello  ");
    }

    #[test]
    fn test_align_text_in_line_standard_style() {
        let result = align_text_in_line("  hello  ", Style::Standard);
        assert_eq!(result, "  hello  ");
    }

    #[test]
    fn test_align_text_in_line_detailed_style() {
        let result = align_text_in_line("  hello  ", Style::Detailed);
        assert_eq!(result, "   hello ");
    }

    #[test]
    fn test_align_text_in_line_already_aligned() {
        let result = align_text_in_line("hello", Style::Minimal);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_align_text_in_line_unicode() {
        let result = align_text_in_line("  你好  ", Style::Minimal);
        assert_eq!(result, "  你好  ");
    }

    #[test]
    fn test_align_text_in_line_no_padding() {
        let result = align_text_in_line("hello", Style::Minimal);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_align_text_in_line_trailing_spaces() {
        let result = align_text_in_line("hello  ", Style::Minimal);
        assert_eq!(result, "hello  ");
    }

    #[test]
    fn test_center_text_exact_width() {
        let result = center_text("hello", 5);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_center_text_less_than_width() {
        let result = center_text("hello", 9);
        assert_eq!(result, "  hello  ");
    }

    #[test]
    fn test_center_text_odd_width_difference() {
        let result = center_text("hi", 5);
        assert_eq!(result, " hi  ");
    }

    #[test]
    fn test_center_text_even_width_difference() {
        let result = center_text("hi", 6);
        assert_eq!(result, "  hi  ");
    }

    #[test]
    fn test_center_text_greater_than_width() {
        let result = center_text("hello world", 5);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_center_text_empty_string() {
        let result = center_text("", 5);
        assert_eq!(result, "     ");
    }

    #[test]
    fn test_center_text_single_char() {
        let result = center_text("a", 5);
        assert_eq!(result, "  a  ");
    }

    #[test]
    fn test_center_text_unicode() {
        let result = center_text("你好", 6);
        assert_eq!(result, " 你好 ");
    }

    #[test]
    fn test_align_text_line_with_unicode_width() {
        let result = align_text_in_line("  你好  ", Style::Detailed);
        let trimmed = "你好";
        let original_len = 8;
        let expected_len = UnicodeWidthStr::width(trimmed);
        let padding = original_len - expected_len;
        let left_pad = padding / 2;

        assert!(result.starts_with(" ".repeat(left_pad).as_str()));
        assert!(result.contains("你好"));
    }

    #[test]
    fn test_align_text_content_preserves_lines() {
        let original_lines = vec![
            "  hello  ".to_string(),
            "  world  ".to_string(),
            "  test   ".to_string(),
        ];

        let mut diagram = ParsedDiagram {
            lines: original_lines.clone(),
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let metrics = GridMetrics {
            column_width: 2,
            row_height: 1,
            columns: vec![],
            rows: vec![],
        };

        align_text_content(&mut diagram, &metrics, Style::Standard).unwrap();

        assert_eq!(diagram.lines.len(), 3);
    }

    #[test]
    fn test_align_text_content_with_all_styles() {
        let diagram = ParsedDiagram {
            lines: vec!["  hello  ".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let metrics = GridMetrics {
            column_width: 2,
            row_height: 1,
            columns: vec![],
            rows: vec![],
        };

        for style in [Style::Minimal, Style::Standard, Style::Detailed] {
            let mut diagram_clone = diagram.clone();
            assert!(align_text_content(&mut diagram_clone, &metrics, style).is_ok());
        }
    }

    #[test]
    fn test_align_text_in_line_longer_than_original() {
        let result = align_text_in_line("x", Style::Minimal);
        assert_eq!(result, "x");
    }

    #[test]
    fn test_center_text_zero_width() {
        let result = center_text("hello", 0);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_align_text_in_line_all_whitespace() {
        let result = align_text_in_line("     ", Style::Minimal);
        assert_eq!(result, "");
    }

    #[test]
    fn test_is_text_line_various_box_combinations() {
        assert!(!is_text_line("│─┌┐└┘├┤┬┴┼"));
        assert!(!is_text_line("┌───┐"));
        assert!(!is_text_line("│   │"));
        assert!(!is_text_line("└───┘"));
    }

    #[test]
    fn test_center_text_multiple_spaces() {
        let result = center_text("hi", 10);
        assert_eq!(result, "    hi    ");
    }
}
