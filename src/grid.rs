use crate::display_width::{display_width, display_width_char};
use crate::error::Result;
use crate::parser::ParsedDiagram;
use crate::utils::most_common;

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
        let gaps: Vec<usize> = vertical_positions.windows(2).map(|w| w[1] - w[0]).collect();
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
        let mut x_cells = 0usize;
        for ch in line.chars() {
            if is_grid_vertical_marker(ch) {
                char_counts.push((x_cells, 1));
            }
            x_cells = x_cells.saturating_add(display_width_char(ch));
        }
    }

    let mut position_map: std::collections::HashMap<usize, usize> =
        std::collections::HashMap::new();
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
        if line.chars().any(is_grid_horizontal_marker) {
            positions.push(y);
        }
    }

    positions
}

pub fn normalize_whitespace(diagram: &mut ParsedDiagram, metrics: &GridMetrics) -> Result<()> {
    for line in &mut diagram.lines {
        let trimmed = line.trim_end().to_string();
        line.clear();
        line.push_str(&trimmed);

        // Avoid padding when we only have the fallback grid width.
        if metrics.column_width <= 2 {
            continue;
        }

        let current_len = display_width(line.as_str());
        let target_len = align_to_grid(current_len, metrics.column_width);

        if current_len < target_len {
            line.push_str(&" ".repeat(target_len - current_len));
        }
    }
    Ok(())
}

/// Aligns outer box borders by normalizing line widths.
///
/// This pass is conservative: it only touches runs of consecutive lines that
/// look like box frames (first and last non-whitespace characters are border
/// glyphs). It pads inside the right border so the right edge aligns.
pub fn align_box_borders(diagram: &mut ParsedDiagram) -> Result<()> {
    let mut i = 0;
    while i < diagram.lines.len() {
        let Some(start_info) = outer_border_info(diagram.lines[i].as_str()) else {
            i += 1;
            continue;
        };

        let start = i;
        let indent = start_info.indent;
        i += 1;

        while i < diagram.lines.len() {
            let Some(info) = outer_border_info(diagram.lines[i].as_str()) else {
                break;
            };
            if info.indent != indent {
                break;
            }
            i += 1;
        }

        let end = i;
        align_box_borders_in_range(&mut diagram.lines[start..end]);
    }

    Ok(())
}

fn align_box_borders_in_range(lines: &mut [String]) {
    if lines.is_empty() {
        return;
    }

    let mut target_width = 0usize;
    for line in lines.iter() {
        if let Some(info) = outer_border_info(line.as_str()) {
            target_width = target_width.max(info.effective_width);
        }
    }

    if target_width == 0 {
        return;
    }

    for line in lines {
        let Some(info) = outer_border_info(line.as_str()) else {
            continue;
        };

        if info.effective_width >= target_width {
            continue;
        }

        let pad = target_width - info.effective_width;
        let (insert_byte, pad_char) =
            match trailing_inner_vertical_border(line.as_str(), info.indent, info.right_byte, 4) {
                Some(inner_right_byte) => (inner_right_byte, ' '),
                None => {
                    let pad_char = line[..info.right_byte]
                        .chars()
                        .last()
                        .filter(|ch| matches!(ch, '\u{2500}' | '\u{2550}' | '-' | '_' | '='))
                        .unwrap_or(' ');
                    (info.right_byte, pad_char)
                }
            };

        let mut out = String::with_capacity(info.right_end + pad * pad_char.len_utf8());
        out.push_str(&line[..insert_byte]);
        out.extend(std::iter::repeat_n(pad_char, pad));
        out.push_str(&line[insert_byte..info.right_end]);
        *line = out;
    }
}

fn trailing_inner_vertical_border(
    line: &str,
    indent: usize,
    outer_right_byte: usize,
    max_trailing_padding_bytes: usize,
) -> Option<usize> {
    let prefix = line.get(..outer_right_byte)?;
    let trimmed = prefix.trim_end_matches(&[' ', '\t'][..]);
    if trimmed.len() == prefix.len() {
        return None;
    }

    let trailing = prefix.len() - trimmed.len();
    if trailing > max_trailing_padding_bytes {
        return None;
    }

    let (last_byte, last_ch) = trimmed.char_indices().last()?;
    if last_byte <= indent {
        return None;
    }

    if matches!(last_ch, '\u{2502}' | '\u{2551}' | '|') {
        Some(last_byte)
    } else {
        None
    }
}

#[derive(Debug, Clone, Copy)]
struct OuterBorderInfo {
    indent: usize,
    right_byte: usize,
    right_end: usize,
    effective_width: usize,
}

fn outer_border_info(line: &str) -> Option<OuterBorderInfo> {
    let indent = leading_whitespace_len(line);

    let mut first: Option<(usize, char)> = None;
    let mut last: Option<(usize, char)> = None;
    for (idx, ch) in line.char_indices() {
        if ch == ' ' || ch == '\t' {
            continue;
        }
        if first.is_none() {
            first = Some((idx, ch));
        }
        last = Some((idx, ch));
    }

    let (first_idx, first_ch) = first?;
    let (right_byte, right_ch) = last?;
    if first_idx == right_byte {
        return None;
    }

    if !is_outer_border_char(first_ch) || !is_outer_border_char(right_ch) {
        return None;
    }

    let right_end = right_byte + right_ch.len_utf8();
    let effective_width = display_width(&line[..right_end]);

    Some(OuterBorderInfo {
        indent,
        right_byte,
        right_end,
        effective_width,
    })
}

fn is_outer_border_char(ch: char) -> bool {
    matches!(
        ch,
        '\u{2502}'
            | '\u{2551}'
            | '|'
            | '+'
            | '\u{250c}'
            | '\u{2510}'
            | '\u{2514}'
            | '\u{2518}'
            | '\u{251c}'
            | '\u{2524}'
            | '\u{252c}'
            | '\u{2534}'
            | '\u{253c}'
            | '\u{256d}'
            | '\u{256e}'
            | '\u{2570}'
            | '\u{256f}'
            | '\u{2554}'
            | '\u{2557}'
            | '\u{255a}'
            | '\u{255d}'
            | '\u{2560}'
            | '\u{2563}'
            | '\u{2566}'
            | '\u{2569}'
            | '\u{256c}'
    )
}

/// Shrink lines that overflow a box by trimming padding next to vertical borders.
///
/// This is a conservative fix for common AI output where one line exceeds the
/// box width due to extra spaces around text, causing right borders to drift.
pub fn shrink_overflowing_box_lines(diagram: &mut ParsedDiagram) -> Result<()> {
    use std::collections::HashMap;

    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
    for (idx, line) in diagram.lines.iter().enumerate() {
        let indent = leading_whitespace_len(line);
        groups.entry(indent).or_default().push(idx);
    }

    for (indent, line_indices) in groups {
        let lengths: Vec<usize> = line_indices
            .iter()
            .map(|&i| display_width(diagram.lines[i].as_str()))
            .filter(|&len| len > 0)
            .collect();

        let Some(target_len) = most_common(&lengths) else {
            continue;
        };

        for &i in &line_indices {
            let line_len = display_width(diagram.lines[i].as_str());
            if line_len <= target_len {
                continue;
            }

            let chars: Vec<char> = diagram.lines[i].chars().collect();
            let mut excess = line_len - target_len;

            let remove = mark_padding_removals(&chars, indent, &mut excess);
            if excess == line_len - target_len {
                continue;
            }

            let mut out = String::with_capacity(diagram.lines[i].len());
            for (idx, ch) in chars.iter().copied().enumerate() {
                if !remove[idx] {
                    out.push(ch);
                }
            }
            diagram.lines[i] = out;
        }
    }

    Ok(())
}

fn leading_whitespace_len(line: &str) -> usize {
    line.chars().take_while(|c| *c == ' ' || *c == '\t').count()
}

fn is_grid_vertical_marker(ch: char) -> bool {
    matches!(
        ch,
        '\u{2502}'
            | '\u{2551}'
            | '\u{251c}'
            | '\u{2524}'
            | '\u{253c}'
            | '\u{2560}'
            | '\u{2563}'
            | '\u{256c}'
            | '|'
    )
}

fn is_grid_horizontal_marker(ch: char) -> bool {
    matches!(
        ch,
        '\u{2500}'
            | '\u{2550}'
            | '\u{252c}'
            | '\u{2534}'
            | '\u{253c}'
            | '\u{2566}'
            | '\u{2569}'
            | '\u{256c}'
            | '-'
            | '='
    )
}

fn is_vertical_border(ch: char) -> bool {
    matches!(ch, '\u{2502}' | '\u{2551}' | '|')
}

fn mark_padding_removals(chars: &[char], min_index: usize, excess: &mut usize) -> Vec<bool> {
    let mut remove = vec![false; chars.len()];

    remove_spaces_before_vertical_border(chars, &mut remove, min_index, excess, false);
    remove_spaces_after_vertical_border(chars, &mut remove, min_index, excess, false);
    remove_spaces_before_vertical_border(chars, &mut remove, min_index, excess, true);
    remove_spaces_after_vertical_border(chars, &mut remove, min_index, excess, true);

    remove
}

fn remove_spaces_before_vertical_border(
    chars: &[char],
    remove: &mut [bool],
    min_index: usize,
    excess: &mut usize,
    allow_border_left: bool,
) {
    if *excess == 0 {
        return;
    }
    if chars.len().saturating_sub(min_index) < 2 {
        return;
    }

    let mut i = chars.len().saturating_sub(2);
    loop {
        if *excess == 0 {
            break;
        }

        if i < min_index {
            break;
        }

        if remove[i] || chars[i] != ' ' {
            if i == 0 {
                break;
            }
            i -= 1;
            continue;
        }

        if !is_vertical_border(chars[i + 1]) {
            if i == 0 {
                break;
            }
            i -= 1;
            continue;
        }

        let mut run_start = i;
        while run_start > min_index && chars[run_start - 1] == ' ' && !remove[run_start - 1] {
            run_start -= 1;
        }
        if run_start == min_index {
            if i == 0 {
                break;
            }
            i -= 1;
            continue;
        }

        let prev_non_space = chars[run_start - 1];
        if !allow_border_left && is_vertical_border(prev_non_space) {
            if i == 0 {
                break;
            }
            i -= 1;
            continue;
        }

        let mut idx = i;
        while *excess > 0 && idx >= run_start {
            if !remove[idx] && chars[idx] == ' ' {
                remove[idx] = true;
                *excess -= 1;
            }

            if idx == 0 {
                break;
            }
            idx -= 1;
        }

        if run_start == 0 {
            break;
        }
        i = run_start - 1;
    }
}

fn remove_spaces_after_vertical_border(
    chars: &[char],
    remove: &mut [bool],
    min_index: usize,
    excess: &mut usize,
    allow_border_right: bool,
) {
    if *excess == 0 {
        return;
    }
    if chars.len().saturating_sub(min_index) < 2 {
        return;
    }

    let mut i = min_index;
    while i < chars.len() {
        if *excess == 0 {
            break;
        }

        if remove[i] || chars[i] != ' ' {
            i += 1;
            continue;
        }

        let prev = i.checked_sub(1).and_then(|j| chars.get(j)).copied();
        if !prev.is_some_and(is_vertical_border) {
            i += 1;
            continue;
        }

        let mut run_end = i;
        while run_end + 1 < chars.len() && chars[run_end + 1] == ' ' && !remove[run_end + 1] {
            run_end += 1;
        }

        if run_end + 1 >= chars.len() {
            i = run_end + 1;
            continue;
        }

        let next_non_space = chars[run_end + 1];
        if !allow_border_right && is_vertical_border(next_non_space) {
            i = run_end + 1;
            continue;
        }

        let mut idx = i;
        while *excess > 0 && idx <= run_end {
            if !remove[idx] && chars[idx] == ' ' {
                remove[idx] = true;
                *excess -= 1;
            }
            idx += 1;
        }

        i = run_end + 1;
    }
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
            "\u{250c}\u{2500}\u{2500}\u{2500}\u{2510}".to_string(),
            "\u{2502}   \u{2502}".to_string(),
            "\u{2514}\u{2500}\u{2500}\u{2500}\u{2518}".to_string(),
        ];

        let result = analyze_grid(&lines).unwrap();
        assert_eq!(result.column_width, 4);
        assert_eq!(result.row_height, 1);
    }

    #[test]
    fn test_analyze_grid_multiple_columns() {
        let lines = vec![
            "\u{250c}\u{2500}\u{2500}\u{2500}\u{252c}\u{2500}\u{2500}\u{2500}\u{2510}".to_string(),
            "\u{2502} A \u{2502} B \u{2502}".to_string(),
            "\u{251c}\u{2500}\u{2500}\u{2500}\u{253c}\u{2500}\u{2500}\u{2500}\u{2524}".to_string(),
            "\u{2502} C \u{2502} D \u{2502}".to_string(),
            "\u{2514}\u{2500}\u{2500}\u{2500}\u{2534}\u{2500}\u{2500}\u{2500}\u{2518}".to_string(),
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
        let lines = vec!["Hello World".to_string(), "Test Data".to_string()];

        let result = analyze_grid(&lines).unwrap();
        assert_eq!(result.column_width, 2);
    }

    #[test]
    fn test_analyze_grid_single_vertical_line() {
        let lines = vec![
            "\u{2502}".to_string(),
            "\u{2502}".to_string(),
            "\u{2502}".to_string(),
        ];

        let result = analyze_grid(&lines).unwrap();
        assert!(result.columns.contains(&0));
    }

    #[test]
    fn test_analyze_grid_with_mixed_lines() {
        let lines = vec![
            "\u{250c}\u{2500}\u{2500}\u{2500}\u{2510}".to_string(),
            "\u{2502}   \u{2502}".to_string(),
            "text".to_string(),
            "\u{2514}\u{2500}\u{2500}\u{2500}\u{2518}".to_string(),
        ];

        let result = analyze_grid(&lines).unwrap();
        assert_eq!(result.row_height, 1);
    }

    #[test]
    fn test_find_vertical_positions_basic() {
        let lines = vec![
            "\u{2502}   \u{2502}".to_string(),
            "\u{2502}   \u{2502}".to_string(),
            "\u{2502}   \u{2502}".to_string(),
        ];

        let positions = find_vertical_positions(&lines);
        assert!(positions.contains(&0));
        assert!(positions.contains(&4));
    }

    #[test]
    fn test_find_vertical_positions_threshold() {
        let lines = vec![
            "\u{2502}   \u{2502}".to_string(),
            "\u{2502}   \u{2502}".to_string(),
            "     ".to_string(),
            "\u{2502}   \u{2502}".to_string(),
        ];

        let positions = find_vertical_positions(&lines);
        assert!(positions.contains(&0));
        assert!(positions.contains(&4));
    }

    #[test]
    fn test_find_vertical_positions_no_verticals() {
        let lines = vec!["\u{2500}".repeat(5), "\u{2500}".repeat(5)];

        let positions = find_vertical_positions(&lines);
        assert!(positions.is_empty());
    }

    #[test]
    fn test_find_vertical_positions_duplicates() {
        let lines = vec![
            "\u{2502}".to_string(),
            "\u{2502}".to_string(),
            "\u{2502}".to_string(),
        ];

        let positions = find_vertical_positions(&lines);
        assert_eq!(positions.len(), 1);
        assert_eq!(positions[0], 0);
    }

    #[test]
    fn test_find_horizontal_positions_basic() {
        let lines = vec![
            "\u{2500}".repeat(5),
            "     ".to_string(),
            "\u{2500}".repeat(5),
        ];

        let positions = find_horizontal_positions(&lines);
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&0));
        assert!(positions.contains(&2));
    }

    #[test]
    fn test_find_horizontal_positions_with_tee() {
        let lines = vec![
            "\u{250c}\u{2500}\u{2500}\u{2500}\u{2510}".to_string(),
            "\u{2502}   \u{2502}".to_string(),
            "\u{2514}\u{2500}\u{2500}\u{2500}\u{2518}".to_string(),
        ];

        let positions = find_horizontal_positions(&lines);
        assert!(positions.contains(&0));
    }

    #[test]
    fn test_find_horizontal_positions_no_horizontals() {
        let lines = vec![
            "\u{2502}".to_string(),
            "\u{2502}".to_string(),
            "\u{2502}".to_string(),
        ];

        let positions = find_horizontal_positions(&lines);
        assert!(positions.is_empty());
    }

    #[test]
    fn test_normalize_whitespace_basic() {
        let mut diagram = ParsedDiagram {
            lines: vec!["hello  ".to_string(), "world ".to_string()],
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
            lines: vec!["hi".to_string()],
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
        assert_eq!(display_width(diagram.lines[0].as_str()), 4);
    }

    #[test]
    fn test_normalize_whitespace_empty_lines() {
        let mut diagram = ParsedDiagram {
            lines: vec!["".to_string(), "".to_string()],
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
            lines: vec!["你好".to_string()],
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
        assert_eq!(display_width(diagram.lines[0].as_str()), 4);
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
            "\u{2502} \u{2502} \u{2502}".to_string(),
            "\u{2502} \u{2502} \u{2502}".to_string(),
            "\u{2502} \u{2502} \u{2502}".to_string(),
        ];

        let result = analyze_grid(&lines).unwrap();
        assert!(result.columns.len() >= 3);
    }

    #[test]
    fn test_find_vertical_positions_sorted() {
        let lines = vec![
            "  \u{2502}".to_string(),
            "\u{2502}".to_string(),
            "   \u{2502}".to_string(),
        ];

        let positions = find_vertical_positions(&lines);
        for i in 1..positions.len() {
            assert!(positions[i] > positions[i - 1]);
        }
    }

    #[test]
    fn test_analyze_grid_returns_correct_column_width() {
        let lines = vec![
            "\u{2502}   \u{2502}".to_string(),
            "\u{2502}   \u{2502}".to_string(),
            "\u{2502}   \u{2502}".to_string(),
        ];

        let result = analyze_grid(&lines).unwrap();
        assert!(result.column_width > 0);
    }

    #[test]
    fn test_normalize_whitespace_longer_than_grid() {
        let mut diagram = ParsedDiagram {
            lines: vec!["this is a very long string that exceeds grid".to_string()],
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
            "\u{251c}\u{2500}\u{2500}\u{2500}\u{2524}".to_string(),
            "\u{2502}   \u{2502}".to_string(),
            "\u{2514}\u{2500}\u{2500}\u{2500}\u{2518}".to_string(),
        ];

        let result = analyze_grid(&lines).unwrap();
        assert!(result.columns.len() >= 2);
    }

    #[test]
    fn test_find_horizontal_positions_all_lines() {
        let lines = vec![
            "\u{2500}".repeat(5),
            "\u{2500}".repeat(5),
            "\u{2500}".repeat(5),
        ];

        let positions = find_horizontal_positions(&lines);
        assert_eq!(positions.len(), 3);
    }

    #[test]
    fn test_analyze_grid_preserves_column_positions() {
        let lines = vec![
            "\u{2502}   \u{2502}".to_string(),
            "\u{2502}   \u{2502}".to_string(),
        ];

        let result = analyze_grid(&lines).unwrap();
        assert!(result.columns.contains(&0));
        assert!(result.columns.contains(&4));
    }
}
