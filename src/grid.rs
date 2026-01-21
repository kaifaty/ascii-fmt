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
    ((value + grid_size - 1) / grid_size) * grid_size
}
