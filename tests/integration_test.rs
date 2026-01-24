use ascii_fmt::cli::{Options, StyleStr};
use ascii_fmt::formatter;
use ascii_fmt::text_align::Style;
use std::path::PathBuf;

const PLUS: &str = "\x2b";
const PIPE: &str = "\x7c";
const ARROW_R: &str = "-\x3e";

const BD_H: &str = "\u{2500}";
const BD_V: &str = "\u{2502}";
const BD_TL: &str = "\u{250c}";
const BD_TR: &str = "\u{2510}";
const BD_BL: &str = "\u{2514}";
const BD_BR: &str = "\u{2518}";
const BD_DSLASH: &str = "\u{2571}";

const BD_D_H: &str = "\u{2550}";
const BD_D_V: &str = "\u{2551}";
const BD_D_TL: &str = "\u{2554}";
const BD_D_TR: &str = "\u{2557}";
const BD_D_BL: &str = "\u{255a}";
const BD_D_BR: &str = "\u{255d}";

#[test]
fn test_integration_simple_box() {
    let input = format!("{PLUS}---{PLUS}\n{PIPE} A {PIPE}\n{PLUS}---{PLUS}");
    let options = Options {
        width: 2,
        style: Style::Minimal,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(&input, &options);
    assert!(result.is_ok());
}

#[test]
fn test_integration_complex_table() {
    let input = format!(
        "{PLUS}---{PLUS}---{PLUS}\n{PIPE} A {PIPE} B {PIPE}\n{PLUS}---{PLUS}---{PLUS}\n{PIPE} C {PIPE} D {PIPE}\n{PLUS}---{PLUS}---{PLUS}",
    );
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(&input, &options);
    assert!(result.is_ok());
}

#[test]
fn test_integration_tree_diagram() {
    let input = "root\n  child1\n    grandchild\n  child2";
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(input, &options);
    assert!(result.is_ok());
}

#[test]
fn test_integration_flowchart_with_arrows() {
    let input = format!("A {ARROW_R} B\nB {ARROW_R} C\nC {ARROW_R} D");
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(&input, &options);
    assert!(result.is_ok());
}

#[test]
fn test_integration_architecture_diagram() {
    let input = format!(
        "{PLUS}-------{PLUS}\n{PIPE} Front {PIPE}\n{PLUS}-------{PLUS}\n    {PIPE}\n{PLUS}-------{PLUS}\n{PIPE}  DB   {PIPE}\n{PLUS}-------{PLUS}",
    );
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(&input, &options);
    assert!(result.is_ok());
}

#[test]
fn test_integration_empty_input() {
    let input = "";
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(input, &options);
    assert!(result.is_err());
}

#[test]
fn test_integration_unicode_content() {
    let input = format!("{PLUS}----{PLUS}\n{PIPE}你好{PIPE}\n{PLUS}----{PLUS}");
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(&input, &options);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("你好"));
}

#[test]
fn test_integration_minimal_style() {
    let input = format!("{PLUS}---{PLUS}\n{PIPE} A {PIPE}\n{PLUS}---{PLUS}");
    let options = Options {
        width: 2,
        style: Style::Minimal,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(&input, &options);
    assert!(result.is_ok());
}

#[test]
fn test_integration_detailed_style() {
    let input = format!("{PLUS}-----{PLUS}\n{PIPE} Hello {PIPE}\n{PLUS}-----{PLUS}");
    let options = Options {
        width: 2,
        style: Style::Detailed,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(&input, &options);
    assert!(result.is_ok());
}

#[test]
fn test_integration_no_box_drawing_fix() {
    let input = format!("{PLUS}---{PLUS}\n{PIPE} A {PIPE}\n{PLUS}---{PLUS}");
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: false,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(&input, &options);
    assert!(result.is_ok());
}

#[test]
fn test_integration_no_whitespace_fix() {
    let h3 = BD_H.repeat(3);
    let input = format!("{BD_TL}{h3}{BD_TR}\n{PIPE} A {PIPE}\n{BD_BL}{h3}{BD_BR}");
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: false,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(&input, &options);
    assert!(result.is_ok());
}

#[test]
fn test_integration_multiple_boxes() {
    let input = format!(
        "{PLUS}---{PLUS}\n{PIPE} A {PIPE}\n{PLUS}---{PLUS}\n\n{PLUS}---{PLUS}\n{PIPE} B {PIPE}\n{PLUS}---{PLUS}",
    );
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(&input, &options);
    assert!(result.is_ok());
}

#[test]
fn test_integration_nested_boxes() {
    let input = format!(
        "{PLUS}-------{PLUS}\n{PIPE}{PLUS}-----{PLUS}{PIPE}\n{PIPE}{PIPE}Hello{PIPE}{PIPE}\n{PIPE}{PLUS}-----{PLUS}{PIPE}\n{PLUS}-------{PLUS}",
    );
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(&input, &options);
    assert!(result.is_ok());
}

#[test]
fn test_integration_large_diagram() {
    let input = format!(
        "{PLUS}---{PLUS}---{PLUS}---{PLUS}\n{PIPE} A {PIPE} B {PIPE} C {PIPE}\n{PLUS}---{PLUS}---{PLUS}---{PLUS}\n{PIPE} D {PIPE} E {PIPE} F {PIPE}\n{PLUS}---{PLUS}---{PLUS}---{PLUS}\n{PIPE} G {PIPE} H {PIPE} I {PIPE}\n{PLUS}---{PLUS}---{PLUS}---{PLUS}",
    );
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(&input, &options);
    assert!(result.is_ok());
}

#[test]
fn test_integration_diagonal_lines() {
    let input = "   /\n  / \n /  \n/___";
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(input, &options);
    assert!(result.is_ok());
}

#[test]
fn test_integration_arrows_converted() {
    let input = "v\n^\n<\n>";
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(input, &options);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("▼"));
    assert!(output.contains("▲"));
    assert!(output.contains("◀"));
    assert!(output.contains("▶"));
}

#[test]
fn test_integration_underscore_to_horizontal() {
    let input = "_____";
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(input, &options);
    assert!(result.is_ok());
}

#[test]
fn test_integration_exclamation_to_vertical() {
    let input = "!\n!\n!";
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(input, &options);
    assert!(result.is_ok());
}

#[test]
fn test_integration_text_centering() {
    let input = format!("{PLUS}-------{PLUS}\n{PIPE} Hello {PIPE}\n{PLUS}-------{PLUS}");
    let options = Options {
        width: 2,
        style: Style::Detailed,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(&input, &options);
    assert!(result.is_ok());
}

#[test]
fn test_integration_preserves_empty_lines() {
    let input = "line1\n\nline2\n\n\nline5";
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(input, &options);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.lines().count(), 6);
}

#[test]
fn test_integration_mixed_diagram_types() {
    let input =
        format!("{PLUS}---{PLUS}\n{PIPE} A {PIPE}\n{PLUS}---{PLUS}\nA {ARROW_R} B\nB {ARROW_R} C",);
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(&input, &options);
    assert!(result.is_ok());
}

#[test]
fn test_integration_width_option() {
    let input = format!("{PLUS}---{PLUS}\n{PIPE} A {PIPE}\n{PLUS}---{PLUS}");
    let options = Options {
        width: 4,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(&input, &options);
    assert!(result.is_ok());
}

#[test]
fn test_integration_cli_parsing() {
    let cli = ascii_fmt::cli::Cli {
        command: None,
        input: Some(PathBuf::from("test.txt")),
        output: Some(PathBuf::from("output.txt")),
        width: 4,
        style: StyleStr::Detailed,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: false,
        dry_run: false,
        verbose: 0,
    };

    let options = Options::try_from(cli);
    assert!(options.is_ok());
    let options = options.unwrap();
    assert_eq!(options.width, 4);
    assert_eq!(options.style, Style::Detailed);
    assert!(!options.preserve_empty_lines);
}

#[test]
fn test_integration_cli_valid_styles() {
    for style_str in [StyleStr::Minimal, StyleStr::Standard, StyleStr::Detailed] {
        let cli = ascii_fmt::cli::Cli {
            command: None,
            input: Some(PathBuf::from("test.txt")),
            output: None,
            width: 2,
            style: style_str,
            fix_box_drawing: true,
            fix_whitespace: true,
            preserve_empty_lines: true,
            dry_run: false,
            verbose: 0,
        };

        let options = Options::try_from(cli);
        assert!(options.is_ok());
    }
}

#[test]
fn test_integration_parser_empty_input() {
    use ascii_fmt::parser;

    let result = parser::parse("");
    assert!(result.is_err());
}

#[test]
fn test_integration_parser_valid_diagram() {
    use ascii_fmt::parser;

    let input = format!("{PLUS}---{PLUS}\n{PIPE} A {PIPE}\n{PLUS}---{PLUS}");
    let result = parser::parse(&input);
    assert!(result.is_ok());
    let diagram = result.unwrap();
    assert_eq!(diagram.lines.len(), 3);
}

#[test]
fn test_integration_grid_analysis() {
    use ascii_fmt::grid;

    let lines = vec![
        format!("{PLUS}---{PLUS}"),
        format!("{PIPE} A {PIPE}"),
        format!("{PLUS}---{PLUS}"),
    ];

    let result = grid::analyze_grid(&lines);
    assert!(result.is_ok());
    let metrics = result.unwrap();
    assert!(metrics.column_width > 0);
}

#[test]
fn test_integration_pattern_detection() {
    use ascii_fmt::patterns;

    let tree_lines = vec!["root".to_string(), "  child".to_string()];

    let tree_type = patterns::detect_type(&tree_lines);
    assert_eq!(tree_type, patterns::DiagramType::Tree);

    let flowchart_lines = vec![
        format!("A {ARROW_R} B"),
        format!("{PLUS}---{PLUS}"),
        format!("{PIPE} A {PIPE}"),
        format!("{PLUS}---{PLUS}"),
    ];

    let flowchart_type = patterns::detect_type(&flowchart_lines);
    assert_eq!(flowchart_type, patterns::DiagramType::Flowchart);
}

#[test]
fn test_integration_full_workflow() {
    use ascii_fmt::formatter;

    let input = format!(
        "{PLUS}---{PLUS}---{PLUS}\n{PIPE} A {PIPE} B {PIPE}\n{PLUS}---{PLUS}---{PLUS}\n{PIPE} C {PIPE} D {PIPE}\n{PLUS}---{PLUS}---{PLUS}",
    );
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(&input, &options);
    assert!(result.is_ok());

    let output = result.unwrap();
    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines.len(), 5);
    assert!(output.contains("A"));
    assert!(output.contains("B"));
    assert!(output.contains("C"));
    assert!(output.contains("D"));
}

#[test]
fn test_integration_error_handling() {
    use ascii_fmt::formatter;

    let input = "";
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(input, &options);
    assert!(result.is_err());
}

#[test]
fn test_integration_all_styles() {
    let input = format!("{PLUS}-----{PLUS}\n{PIPE} Hello {PIPE}\n{PLUS}-----{PLUS}");

    for style in [Style::Minimal, Style::Standard, Style::Detailed] {
        let options = Options {
            width: 2,
            style,
            fix_box_drawing: true,
            fix_whitespace: true,
            preserve_empty_lines: true,
            dry_run: false,
            verbose: false,
        };

        let result = formatter::format_ascii(&input, &options);
        assert!(result.is_ok(), "Failed for style: {:?}", style);
    }
}

#[test]
fn test_integration_formats_markdown_example_test_md() {
    let input = include_str!("fixtures/markdown/test.md");
    let expected = include_str!("fixtures/markdown/test.good.md");

    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let output = formatter::format_ascii(input, &options).unwrap();
    assert_eq!(
        output.trim_end_matches(&['\r', '\n'][..]),
        expected.trim_end_matches(&['\r', '\n'][..]),
    );
}

#[test]
fn test_integration_formats_markdown_kitties_md() {
    let input = include_str!("fixtures/markdown/kitties.md");
    let expected = include_str!("fixtures/markdown/kitties.good.md");

    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let output = formatter::format_ascii(input, &options).unwrap();
    assert_eq!(
        output.trim_end_matches(&['\r', '\n'][..]),
        expected.trim_end_matches(&['\r', '\n'][..]),
    );
}

#[test]
fn test_integration_formats_double_border_table_with_mixed_junctions() {
    let input = include_str!("fixtures/tables/dogs.bad.txt");
    let expected = include_str!("fixtures/tables/dogs.good.txt");

    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let output = formatter::format_ascii(input, &options).unwrap();
    assert_eq!(
        output.trim_end_matches(&['\r', '\n'][..]),
        expected.trim_end_matches(&['\r', '\n'][..]),
    );
}

#[test]
fn test_integration_preserves_word_characters_in_text() {
    let input = r"Service ts-rs ChaCha20/Argon2 Orange Pi/ C:\Path\to\file";
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let output = formatter::format_ascii(input, &options).unwrap();
    assert_eq!(output, input);
}

#[test]
fn test_integration_markdown_fence_with_language_is_preserved() {
    let input = format!("```text\n{PLUS}---{PLUS}\n{PIPE} A {PIPE}\n{PLUS}---{PLUS}\n```");

    let h3 = BD_H.repeat(3);
    let expected =
        format!("```text\n{BD_TL}{h3}{BD_TR}\n{BD_V} A {BD_V}\n{BD_BL}{h3}{BD_BR}\n```",);

    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let output = formatter::format_ascii(&input, &options).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn test_integration_shrinks_overflowing_ascii_box_line_padding() {
    let input = format!(
        "{PLUS}--------{PLUS}\n{PIPE} Hello  {PIPE}\n{PIPE} World   {PIPE}\n{PLUS}--------{PLUS}",
    );

    let h8 = BD_H.repeat(8);
    let expected = format!(
        "{BD_TL}{h8}{BD_TR}\n{BD_V} Hello  {BD_V}\n{BD_V} World  {BD_V}\n{BD_BL}{h8}{BD_BR}",
    );

    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let output = formatter::format_ascii(&input, &options).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn test_integration_does_not_over_shrink_lines_with_variation_selector_emoji() {
    let input = format!("{PLUS}----{PLUS}\n{PIPE} ☁️  {PIPE}\n{PLUS}----{PLUS}");

    let h4 = BD_H.repeat(4);
    let expected = format!("{BD_TL}{h4}{BD_TR}\n{BD_V} ☁️ {BD_V}\n{BD_BL}{h4}{BD_BR}");

    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let output = formatter::format_ascii(&input, &options).unwrap();
    assert_eq!(output, expected);
}

#[test]
fn test_integration_shrinks_overflowing_double_border_lines_with_emoji() {
    let inner_width = 42usize;
    let title = "🐕 DOG PACK 🐕";
    let left_pad = 10usize;
    let title_chars = title.chars().count();
    let right_pad = inner_width
        .saturating_sub(left_pad)
        .saturating_sub(title_chars);

    let top = BD_D_H.repeat(inner_width);
    let input = format!(
        "{BD_D_TL}{top}{BD_D_TR}\n{BD_D_V}{left}{title}{right}{BD_D_V}\n{BD_D_BL}{top}{BD_D_BR}",
        left = " ".repeat(left_pad),
        right = " ".repeat(right_pad),
    );

    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let output = formatter::format_ascii(&input, &options).unwrap();
    let mut lines = output.lines();
    let Some(first) = lines.next() else {
        panic!("expected at least one output line");
    };
    let w0 = ascii_fmt::display_width::display_width(first);
    for line in lines {
        assert_eq!(
            ascii_fmt::display_width::display_width(line),
            w0,
            "line widths diverged in output: {output}"
        );
    }
}

#[test]
fn test_integration_idempotent_on_markdown_example_test_md() {
    let input = include_str!("fixtures/markdown/test.md");

    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let once = formatter::format_ascii(input, &options).unwrap();
    let twice = formatter::format_ascii(&once, &options).unwrap();

    assert_eq!(
        once.trim_end_matches(&['\r', '\n'][..]),
        twice.trim_end_matches(&['\r', '\n'][..]),
    );
}

#[test]
fn test_integration_converts_diagonal_slashes_in_diagrams() {
    let input = " / \n/  ";
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let output = formatter::format_ascii(input, &options).unwrap();
    assert!(output.contains(BD_DSLASH));
    assert!(!output.contains('/'));
}

#[test]
fn test_integration_converts_hyphen_runs_but_not_hyphens_in_words() {
    let input = "foo-bar\n-----";
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let output = formatter::format_ascii(input, &options).unwrap();
    let h5 = BD_H.repeat(5);
    assert!(output.contains("foo-bar"));
    assert!(output.contains(&h5));
}
