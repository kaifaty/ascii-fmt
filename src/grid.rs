use crate::error::Result;
use crate::parser::ParsedDiagram;
use crate::utils::most_common;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone)]
pub struct GridMetrics {
    pub column_width: usize,
    pub row_height: usize,
    pub columns: Vec<usize>,
    pub rows: Vec<usize>,
}

pub fn analyze_grid(lines: &[String]) -> Result<GridMetrics> {
    let vertical_positions = find_vertical_positions(lines);
    let column_width = if !vertical_positions.is_empty() {
        let gaps: Vec<usize> = vertical_positions
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect();
        most_common(&gaps).unwrap_or(2)
    } else {
        2
    };

    let horizontal_positions = find_horizontal_positions(lines);

    Ok(GridMetrics {
        column_width,
        row_height: 1,
        columns: vertical_positions,
        rows: horizontal_positions,
    })
}

fn find_vertical_positions(lines: &[String]) -> Vec<usize> {
    let mut positions = Vec::new();
    let mut char_counts: Vec<(usize, usize)> = Vec::new();

    for line in lines {
        for (x, ch) in line.chars().enumerate() {
            if ch == '│' || ch == '├' || ch == '┤' || ch == '┼' {
                char_counts.push((x, 1));
            }
        }
    }

    let mut position_map: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    for (x, _) in char_counts {
        *position_map.entry(x).or_insert(0) += 1;
    }

    let threshold = lines.len() / 2;
    for (x, count) in position_map {
        if count >= threshold.max(1) {
            positions.push(x);
        }
    }

    positions.sort();
    positions.dedup();
    positions
}

fn find_horizontal_positions(lines: &[String]) -> Vec<usize> {
    let mut positions = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        if line.contains('─') || line.contains('┬') || line.contains('┴') {
            positions.push(y);
        }
    }

    positions
}

pub fn normalize_whitespace(diagram: &mut ParsedDiagram, metrics: &GridMetrics) -> Result<()> {
    for line in &mut diagram.lines {
        let trimmed = line.trim_end().to_string();
        let current_len = UnicodeWidthStr::width(trimmed.as_str());
        let target_len = align_to_grid(current_len, metrics.column_width);

        if current_len < target_len {
            line.clear();
            line.push_str(&trimmed);
            line.push_str(&" ".repeat(target_len - current_len));
        } else {
            line.clear();
            line.push_str(&trimmed);
        }
    }
    Ok(())
}

fn align_to_grid(value: usize, grid_size: usize) -> usize {
    if grid_size == 0 {
        return value;
    }
    value.div_ceil(grid_size) * grid_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_metrics_default() {
        let metrics = GridMetrics {
            column_width: 0,
            row_height: 0,
            columns: vec![],
            rows: vec![],
        };
        assert_eq!(metrics.column_width, 0);
        assert_eq!(metrics.row_height, 0);
        assert!(metrics.columns.is_empty());
        assert!(metrics.rows.is_empty());
    }

    #[test]
    fn test_grid_metrics_clone() {
        let metrics = GridMetrics {
            column_width: 2,
            row_height: 1,
            columns: vec![1, 2, 3],
            rows: vec![4, 5, 6],
        };

        let metrics_clone = metrics.clone();
        assert_eq!(metrics_clone.column_width, 2);
        assert_eq!(metrics_clone.row_height, 1);
        assert_eq!(metrics_clone.columns.len(), 3);
        assert_eq!(metrics_clone.rows.len(), 3);
    }

    #[test]
    fn test_analyze_grid_simple_box() {
        let lines = vec![
            "┌───┐".to_string(),
            "│   │".to_string(),
            "└───┘".to_string(),
        ];

        let result = analyze_grid(&lines).unwrap();
        assert_eq!(result.column_width, 4);
        assert_eq!(result.row_height, 1);
    }

    #[test]
    fn test_analyze_grid_multiple_columns() {
        let lines = vec![
            "┌───┬───┐".to_string(),
            "│ A │ B │".to_string(),
            "├───┼───┤".to_string(),
            "│ C │ D │".to_string(),
            "└───┴───┘".to_string(),
        ];

        let result = analyze_grid(&lines).unwrap();
        assert_eq!(result.column_width, 4);
    }

    #[test]
    fn test_analyze_grid_empty_lines() {
        let lines: Vec<String> = vec![];
        let result = analyze_grid(&lines);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().column_width, 2);
    }

    #[test]
    fn test_analyze_grid_text_only() {
        let lines = vec![
            "Hello World".to_string(),
            "Test Data".to_string(),
        ];

        let result = analyze_grid(&lines).unwrap();
        assert_eq!(result.column_width, 2);
    }

    #[test]
    fn test_analyze_grid_single_vertical_line() {
        let lines = vec![
            "│".to_string(),
            "│".to_string(),
            "│".to_string(),
        ];

        let result = analyze_grid(&lines).unwrap();
        assert!(result.columns.contains(&0));
    }

    #[test]
    fn test_analyze_grid_with_mixed_lines() {
        let lines = vec![
            "┌───┐".to_string(),
            "│   │".to_string(),
            "text".to_string(),
            "└───┘".to_string(),
        ];

        let result = analyze_grid(&lines).unwrap();
        assert_eq!(result.row_height, 1);
    }

    #[test]
    fn test_find_vertical_positions_basic() {
        let lines = vec![
            "│   │".to_string(),
            "│   │".to_string(),
            "│   │".to_string(),
        ];

        let positions = find_vertical_positions(&lines);
        assert!(positions.contains(&0));
        assert!(positions.contains(&4));
    }

    #[test]
    fn test_find_vertical_positions_threshold() {
        let lines = vec![
            "│   │".to_string(),
            "│   │".to_string(),
            "     ".to_string(),
            "│   │".to_string(),
        ];

        let positions = find_vertical_positions(&lines);
        assert!(positions.contains(&0));
        assert!(positions.contains(&4));
    }

    #[test]
    fn test_find_vertical_positions_no_verticals() {
        let lines = vec![
            "─────".to_string(),
            "─────".to_string(),
        ];

        let positions = find_vertical_positions(&lines);
        assert!(positions.is_empty());
    }

    #[test]
    fn test_find_vertical_positions_duplicates() {
        let lines = vec![
            "│".to_string(),
            "│".to_string(),
            "│".to_string(),
        ];

        let positions = find_vertical_positions(&lines);
        assert_eq!(positions.len(), 1);
        assert_eq!(positions[0], 0);
    }

    #[test]
    fn test_find_horizontal_positions_basic() {
        let lines = vec![
            "─────".to_string(),
            "     ".to_string(),
            "─────".to_string(),
        ];

        let positions = find_horizontal_positions(&lines);
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&0));
        assert!(positions.contains(&2));
    }

    #[test]
    fn test_find_horizontal_positions_with_tee() {
        let lines = vec![
            "┌───┐".to_string(),
            "│   │".to_string(),
            "└───┘".to_string(),
        ];

        let positions = find_horizontal_positions(&lines);
        assert!(positions.contains(&0));
    }

    #[test]
    fn test_find_horizontal_positions_no_horizontals() {
        let lines = vec![
            "│".to_string(),
            "│".to_string(),
            "│".to_string(),
        ];

        let positions = find_horizontal_positions(&lines);
        assert!(positions.is_empty());
    }

    #[test]
    fn test_normalize_whitespace_basic() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "hello  ".to_string(),
                "world ".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let metrics = GridMetrics {
            column_width: 2,
            row_height: 1,
            columns: vec![],
            rows: vec![],
        };

        let result = normalize_whitespace(&mut diagram, &metrics);
        assert!(result.is_ok());
        assert_eq!(diagram.lines[0].trim_end(), "hello");
        assert_eq!(diagram.lines[1].trim_end(), "world");
    }

    #[test]
    fn test_normalize_whitespace_align_to_grid() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "hi".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let metrics = GridMetrics {
            column_width: 4,
            row_height: 1,
            columns: vec![],
            rows: vec![],
        };

        let result = normalize_whitespace(&mut diagram, &metrics);
        assert!(result.is_ok());
        assert_eq!(UnicodeWidthStr::width(diagram.lines[0].as_str()), 4);
    }

    #[test]
    fn test_normalize_whitespace_empty_lines() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "".to_string(),
                "".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let metrics = GridMetrics {
            column_width: 2,
            row_height: 1,
            columns: vec![],
            rows: vec![],
        };

        let result = normalize_whitespace(&mut diagram, &metrics);
        assert!(result.is_ok());
        assert_eq!(diagram.lines[0], "");
        assert_eq!(diagram.lines[1], "");
    }

    #[test]
    fn test_normalize_whitespace_unicode() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "你好".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let metrics = GridMetrics {
            column_width: 4,
            row_height: 1,
            columns: vec![],
            rows: vec![],
        };

        let result = normalize_whitespace(&mut diagram, &metrics);
        assert!(result.is_ok());
        assert_eq!(UnicodeWidthStr::width(diagram.lines[0].as_str()), 4);
    }

    #[test]
    fn test_align_to_grid_zero_grid_size() {
        assert_eq!(align_to_grid(10, 0), 10);
        assert_eq!(align_to_grid(0, 0), 0);
    }

    #[test]
    fn test_align_to_grid_basic() {
        assert_eq!(align_to_grid(5, 2), 6);
        assert_eq!(align_to_grid(6, 2), 6);
        assert_eq!(align_to_grid(7, 2), 8);
    }

    #[test]
    fn test_align_to_grid_larger_values() {
        assert_eq!(align_to_grid(10, 5), 10);
        assert_eq!(align_to_grid(11, 5), 15);
        assert_eq!(align_to_grid(14, 5), 15);
        assert_eq!(align_to_grid(15, 5), 15);
    }

    #[test]
    fn test_analyze_grid_with_multiple_vertical_positions() {
        let lines = vec![
            "│ │ │".to_string(),
            "│ │ │".to_string(),
            "│ │ │".to_string(),
        ];

        let result = analyze_grid(&lines).unwrap();
        assert!(result.columns.len() >= 3);
    }

    #[test]
    fn test_find_vertical_positions_sorted() {
        let lines = vec![
            "  │".to_string(),
            "│".to_string(),
            "   │".to_string(),
        ];

        let positions = find_vertical_positions(&lines);
        for i in 1..positions.len() {
            assert!(positions[i] > positions[i - 1]);
        }
    }

    #[test]
    fn test_analyze_grid_returns_correct_column_width() {
        let lines = vec![
            "│   │".to_string(),
            "│   │".to_string(),
            "│   │".to_string(),
        ];

        let result = analyze_grid(&lines).unwrap();
        assert!(result.column_width > 0);
    }

    #[test]
    fn test_normalize_whitespace_longer_than_grid() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "this is a very long string that exceeds grid".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let metrics = GridMetrics {
            column_width: 4,
            row_height: 1,
            columns: vec![],
            rows: vec![],
        };

        let result = normalize_whitespace(&mut diagram, &metrics);
        assert!(result.is_ok());
    }

    #[test]
    fn test_analyze_grid_with_crosses() {
        let lines = vec![
            "├───┤".to_string(),
            "│   │".to_string(),
            "└───┘".to_string(),
        ];

        let result = analyze_grid(&lines).unwrap();
        assert!(result.columns.len() >= 2);
    }

    #[test]
    fn test_find_horizontal_positions_all_lines() {
        let lines = vec![
            "─────".to_string(),
            "─────".to_string(),
            "─────".to_string(),
        ];

        let positions = find_horizontal_positions(&lines);
        assert_eq!(positions.len(), 3);
    }

    #[test]
    fn test_analyze_grid_preserves_column_positions() {
        let lines = vec![
            "│   │".to_string(),
            "│   │".to_string(),
        ];

        let result = analyze_grid(&lines).unwrap();
        assert!(result.columns.contains(&0));
        assert!(result.columns.contains(&4));
    }
}
