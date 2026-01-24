use crate::box_drawing::fix_box_drawing_symbols;
use crate::cli::Options;
use crate::error::Result;
use crate::grid::{
    align_box_borders, analyze_grid, normalize_whitespace, shrink_overflowing_box_lines,
};
use crate::parser::parse;
use crate::tables::format_tables;
use crate::text_align::align_text_content;

pub fn format_ascii(input: &str, options: &Options) -> Result<String> {
    if looks_like_markdown(input) {
        return format_markdown(input, options);
    }

    format_diagram(input, options)
}

fn looks_like_markdown(input: &str) -> bool {
    input
        .lines()
        .any(|line| line.trim_start().starts_with("```"))
}

fn format_markdown(input: &str, options: &Options) -> Result<String> {
    let mut out: Vec<String> = Vec::new();
    let mut in_fence = false;
    let mut fence_body: Vec<String> = Vec::new();

    for line in input.lines() {
        let is_fence = line.trim_start().starts_with("```");

        if is_fence {
            if in_fence {
                // Closing fence: format the fenced content as a standalone diagram.
                if !fence_body.is_empty() {
                    let body_input = fence_body.join("\n");
                    if body_input.trim().is_empty() {
                        out.append(&mut fence_body);
                    } else {
                        let formatted_body = format_diagram(&body_input, options)?;
                        out.extend(formatted_body.lines().map(ToString::to_string));
                        fence_body.clear();
                    }
                }

                out.push(line.to_string());
                in_fence = false;
            } else {
                out.push(line.to_string());
                in_fence = true;
                fence_body.clear();
            }

            continue;
        }

        if in_fence {
            fence_body.push(line.to_string());
        } else {
            out.push(line.to_string());
        }
    }

    // Unclosed fence: keep the remaining body as-is.
    if in_fence {
        out.extend(fence_body);
    }

    Ok(out.join("\n"))
}

fn format_diagram(input: &str, options: &Options) -> Result<String> {
    let diagram = parse(input)?;

    let mut grid_metrics = analyze_grid(&diagram.lines)?;
    if options.width > 2 {
        grid_metrics.column_width = options.width;
    }

    let mut formatted = diagram.clone();

    if options.fix_whitespace {
        shrink_overflowing_box_lines(&mut formatted)?;
        normalize_whitespace(&mut formatted, &grid_metrics)?;
    }

    // Run box-drawing fixes after whitespace normalization so corner/junction
    // detection sees the final aligned borders.
    if options.fix_box_drawing {
        fix_box_drawing_symbols(&mut formatted)?;
    }

    if options.fix_whitespace {
        align_box_borders(&mut formatted)?;
    }

    align_text_content(&mut formatted, &grid_metrics, options.style)?;

    if options.fix_whitespace {
        format_tables(&mut formatted)?;
    }

    render(&formatted, options.preserve_empty_lines)
}

fn render(diagram: &crate::parser::ParsedDiagram, preserve_empty_lines: bool) -> Result<String> {
    let lines = diagram.lines.iter().map(|line| line.trim_end());
    let lines: Vec<&str> = if preserve_empty_lines {
        lines.collect()
    } else {
        lines.filter(|line| !line.trim().is_empty()).collect()
    };

    Ok(lines.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_default_options() -> Options {
        Options {
            width: 2,
            style: crate::text_align::Style::Standard,
            fix_box_drawing: true,
            fix_whitespace: true,
            preserve_empty_lines: true,
            dry_run: false,
            verbose: false,
        }
    }

    #[test]
    fn test_format_ascii_simple_box() {
        let input = "+---+\n|   |\n+---+";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_ascii_empty_input() {
        let input = "";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_format_ascii_single_line() {
        let input = "hello world";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("hello"));
    }

    #[test]
    fn test_format_ascii_with_newlines() {
        let input = "line1\nline2\nline3";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.lines().count(), 3);
    }

    #[test]
    fn test_format_ascii_preserves_structure() {
        let input = "\u{250c}\u{2500}\u{2500}\u{2500}\u{2510}\n\u{2502} A \u{2502}\n\u{2514}\u{2500}\u{2500}\u{2500}\u{2518}";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.lines().count(), 3);
    }

    #[test]
    fn test_format_ascii_with_box_drawing_disabled() {
        let input = "+---+\n|   |\n+---+";
        let mut options = create_default_options();
        options.fix_box_drawing = false;

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_ascii_with_whitespace_disabled() {
        let input = "\u{250c}\u{2500}\u{2500}\u{2500}\u{2510}\n\u{2502} A \u{2502}\n\u{2514}\u{2500}\u{2500}\u{2500}\u{2518}";
        let mut options = create_default_options();
        options.fix_whitespace = false;

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_ascii_minimal_style() {
        let input = "\u{250c}\u{2500}\u{2500}\u{2500}\u{2510}\n\u{2502} A \u{2502}\n\u{2514}\u{2500}\u{2500}\u{2500}\u{2518}";
        let mut options = create_default_options();
        options.style = crate::text_align::Style::Minimal;

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_ascii_detailed_style() {
        let input = "\u{250c}\u{2500}\u{2500}\u{2500}\u{2510}\n\u{2502} A \u{2502}\n\u{2514}\u{2500}\u{2500}\u{2500}\u{2518}";
        let mut options = create_default_options();
        options.style = crate::text_align::Style::Detailed;

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_ascii_with_text_content() {
        let input = "\u{250c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}\n\u{2502}hello\u{2502}\n\u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2518}";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("hello"));
    }

    #[test]
    fn test_format_ascii_complex_diagram() {
        let input = "\u{250c}\u{2500}\u{2500}\u{2500}\u{252c}\u{2500}\u{2500}\u{2500}\u{2510}\n\u{2502} A \u{2502} B \u{2502}\n\u{251c}\u{2500}\u{2500}\u{2500}\u{253c}\u{2500}\u{2500}\u{2500}\u{2524}\n\u{2502} C \u{2502} D \u{2502}\n\u{2514}\u{2500}\u{2500}\u{2500}\u{2534}\u{2500}\u{2500}\u{2500}\u{2518}";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_ascii_with_arrows() {
        let input = "A -> B\nB -> C";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_ascii_preserves_empty_lines() {
        let input = "line1\n\nline3";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.lines().count(), 3);
    }

    #[test]
    fn test_format_ascii_unicode_content() {
        let input = "\u{250c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}\n\u{2502}你好\u{2502}\n\u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2518}";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_ascii_with_mixed_chars() {
        let input = "\u{250c}\u{2500}\u{2500}\u{2500}\u{2510}\n\u{2502}A→B\u{2502}\n\u{2514}\u{2500}\u{2500}\u{2500}\u{2518}";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_ascii_indented_lines() {
        let input = "  line1\n    line2\n  line3";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_ascii_with_tabs() {
        let input = "line1\tline2\nline3\tline4";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_ascii_trailing_whitespace() {
        let input = "line1  \nline2  \nline3  ";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_ascii_long_lines() {
        let input = "very long line with lots of text that goes on and on\nand another line";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_ascii_empty_lines_between_content() {
        let input = "line1\n\n\n\nline5";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.lines().count(), 5);
    }

    #[test]
    fn test_format_ascii_single_box() {
        let input = "\u{250c}\u{2500}\u{2510}\n\u{2502} \u{2502}\n\u{2514}\u{2500}\u{2518}";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_ascii_nested_boxes() {
        let input = "\u{250c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}\n\u{2502}\u{250c}\u{2500}\u{2500}\u{2500}\u{2510}\u{2502}\n\u{2502}\u{2502} A \u{2502}\u{2502}\n\u{2502}\u{2514}\u{2500}\u{2500}\u{2500}\u{2518}\u{2502}\n\u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2518}";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_ascii_with_width_option() {
        let input = "\u{250c}\u{2500}\u{2500}\u{2500}\u{2510}\n\u{2502} A \u{2502}\n\u{2514}\u{2500}\u{2500}\u{2500}\u{2518}";
        let mut options = create_default_options();
        options.width = 4;

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_ascii_multiple_diagrams() {
        let input = "\u{250c}\u{2500}\u{2510}\n\u{2502}A\u{2502}\n\u{2514}\u{2500}\u{2518}\n\n\u{250c}\u{2500}\u{2510}\n\u{2502}B\u{2502}\n\u{2514}\u{2500}\u{2518}";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("A"));
        assert!(output.contains("B"));
    }

    #[test]
    fn test_format_ascii_preserves_diagram_type() {
        let input = "\u{250c}\u{2500}\u{2500}\u{2500}\u{2510}\n\u{2502} A \u{2502}\n\u{2514}\u{2500}\u{2500}\u{2500}\u{2518}";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_ascii_with_all_options_disabled() {
        let input = "+---+\n| A |\n+---+";
        let options = Options {
            width: 2,
            style: crate::text_align::Style::Minimal,
            fix_box_drawing: false,
            fix_whitespace: false,
            preserve_empty_lines: false,
            dry_run: false,
            verbose: false,
        };

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_render_single_line() {
        let diagram = crate::parser::ParsedDiagram {
            lines: vec!["hello".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = render(&diagram, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn test_render_multiple_lines() {
        let diagram = crate::parser::ParsedDiagram {
            lines: vec![
                "line1".to_string(),
                "line2".to_string(),
                "line3".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = render(&diagram, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "line1\nline2\nline3");
    }

    #[test]
    fn test_render_empty_lines() {
        let diagram = crate::parser::ParsedDiagram {
            lines: vec!["line1".to_string(), "".to_string(), "line3".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = render(&diagram, true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "line1\n\nline3");
    }

    #[test]
    fn test_format_ascii_tree_diagram() {
        let input = "root\n  child1\n  child2";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_ascii_table_diagram() {
        let input = "\u{250c}\u{2500}\u{2500}\u{2500}\u{252c}\u{2500}\u{2500}\u{2500}\u{2510}\n\u{2502} A \u{2502} B \u{2502}\n\u{2514}\u{2500}\u{2500}\u{2500}\u{2534}\u{2500}\u{2500}\u{2500}\u{2518}";
        let options = create_default_options();

        let result = format_ascii(input, &options);
        assert!(result.is_ok());
    }
}
