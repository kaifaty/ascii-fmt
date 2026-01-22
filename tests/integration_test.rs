use ascii_fmt::cli::{Options, StyleStr};
use ascii_fmt::formatter;
use ascii_fmt::text_align::Style;
use std::path::PathBuf;

#[test]
fn test_integration_simple_box() {
    let input = "+---+\n| A |\n+---+";
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
fn test_integration_complex_table() {
    let input = "+---+---+\n| A | B |\n+---+---+\n| C | D |\n+---+---+";
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
    let input = "A -> B\nB -> C\nC -> D";
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
fn test_integration_architecture_diagram() {
    let input = "+-------+\n| Front |\n+-------+\n    |\n+-------+\n|  DB   |\n+-------+";
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
    let input = "+----+\n|你好|\n+----+";
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
    assert!(output.contains("你好"));
}

#[test]
fn test_integration_minimal_style() {
    let input = "+---+\n| A |\n+---+";
    let options = Options {
        width: 2,
        style: Style::Minimal,
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
fn test_integration_detailed_style() {
    let input = "+-----+\n| Hello |\n+-----+";
    let options = Options {
        width: 2,
        style: Style::Detailed,
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
fn test_integration_no_box_drawing_fix() {
    let input = "+---+\n| A |\n+---+";
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: false,
        fix_whitespace: true,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(input, &options);
    assert!(result.is_ok());
}

#[test]
fn test_integration_no_whitespace_fix() {
    let input = "┌───┐\n| A |\n└───┘";
    let options = Options {
        width: 2,
        style: Style::Standard,
        fix_box_drawing: true,
        fix_whitespace: false,
        preserve_empty_lines: true,
        dry_run: false,
        verbose: false,
    };

    let result = formatter::format_ascii(input, &options);
    assert!(result.is_ok());
}

#[test]
fn test_integration_multiple_boxes() {
    let input = "+---+\n| A |\n+---+\n\n+---+\n| B |\n+---+";
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
fn test_integration_nested_boxes() {
    let input = "+-------+\n|+-----+|\n||Hello||\n|+-----+|\n+-------+";
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
fn test_integration_large_diagram() {
    let input = "+---+---+---+\n| A | B | C |\n+---+---+---+\n| D | E | F |\n+---+---+---+\n| G | H | I |\n+---+---+---+";
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
    let input = "+-------+\n| Hello |\n+-------+";
    let options = Options {
        width: 2,
        style: Style::Detailed,
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
    let input = "+---+\n| A |\n+---+\nA -> B\nB -> C";
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
fn test_integration_width_option() {
    let input = "+---+\n| A |\n+---+";
    let options = Options {
        width: 4,
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
        align_boxes: false,
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
            align_boxes: false,
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

    let input = "+---+\n| A |\n+---+";
    let result = parser::parse(input);
    assert!(result.is_ok());
    let diagram = result.unwrap();
    assert_eq!(diagram.lines.len(), 3);
}

#[test]
fn test_integration_grid_analysis() {
    use ascii_fmt::grid;

    let lines = vec![
        "+---+".to_string(),
        "| A |".to_string(),
        "+---+".to_string(),
    ];

    let result = grid::analyze_grid(&lines);
    assert!(result.is_ok());
    let metrics = result.unwrap();
    assert!(metrics.column_width > 0);
}

#[test]
fn test_integration_pattern_detection() {
    use ascii_fmt::patterns;

    let tree_lines = vec![
        "root".to_string(),
        "  child".to_string(),
    ];

    let tree_type = patterns::detect_type(&tree_lines);
    assert_eq!(tree_type, patterns::DiagramType::Tree);

    let flowchart_lines = vec![
        "A -> B".to_string(),
        "+---+".to_string(),
        "| A |".to_string(),
        "+---+".to_string(),
    ];

    let flowchart_type = patterns::detect_type(&flowchart_lines);
    assert_eq!(flowchart_type, patterns::DiagramType::Flowchart);
}

#[test]
fn test_integration_full_workflow() {
    use ascii_fmt::formatter;

    let input = "+---+---+\n| A | B |\n+---+---+\n| C | D |\n+---+---+";
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
    let input = "+-----+\n| Hello |\n+-----+";

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

        let result = formatter::format_ascii(input, &options);
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
fn test_integration_preserves_word_characters_in_text() {
    let input = "Service ts-rs ChaCha20/Argon2 Orange Pi/ C:\\Path\\to\\file";
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
    let input = "```text\n+---+\n| A |\n+---+\n```";
    let expected = "```text\n┌───┐\n│ A │\n└───┘\n```";

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
    assert_eq!(output, expected);
}

#[test]
fn test_integration_shrinks_overflowing_ascii_box_line_padding() {
    let input = "+--------+\n| Hello  |\n| World   |\n+--------+";
    let expected = "┌────────┐\n│ Hello  │\n│ World  │\n└────────┘";

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
    assert_eq!(output, expected);
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
    assert!(output.contains("╱"));
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
    assert!(output.contains("foo-bar"));
    assert!(output.contains("─────"));
}
