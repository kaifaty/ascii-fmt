//! Box alignment module for auto-adjusting box widths.
//!
//! This module provides functionality to:
//! 1. Detect closed boxes in ASCII diagrams
//! 2. Build hierarchy of nested boxes
//! 3. Calculate minimum widths based on content
//! 4. Expand boxes to fit their content

use crate::error::Result;
use crate::parser::ParsedDiagram;
use std::collections::HashSet;
use unicode_width::UnicodeWidthStr;

/// Represents a detected box in the diagram
#[derive(Debug, Clone)]
pub struct Box {
    pub id: usize,
    pub top_left: (usize, usize),     // (x, y)
    pub bottom_right: (usize, usize), // (x, y)
    pub content_lines: Vec<usize>,    // indices of lines with content
    pub parent_id: Option<usize>,
    pub children_ids: Vec<usize>,
}

impl Box {
    /// Returns the width of the box (including borders)
    pub fn width(&self) -> usize {
        self.bottom_right.0.saturating_sub(self.top_left.0) + 1
    }

    /// Returns the height of the box (including borders)
    pub fn height(&self) -> usize {
        self.bottom_right.1.saturating_sub(self.top_left.1) + 1
    }

    /// Returns the area of the box
    pub fn area(&self) -> usize {
        self.width() * self.height()
    }
}

/// Metrics for box sizing calculations
#[derive(Debug, Clone)]
pub struct BoxMetrics {
    pub current_width: usize,
    pub min_content_width: usize, // max width of text inside
    pub required_width: usize,    // with padding
}

pub fn align_box_widths(diagram: &mut ParsedDiagram) -> Result<()> {
    let mut boxes = detect_boxes(&diagram.lines);

    if boxes.is_empty() {
        return Ok(());
    }

    // Assign IDs
    for (i, b) in boxes.iter_mut().enumerate() {
        b.id = i;
    }

    build_hierarchy(&mut boxes);
    let metrics = calculate_min_widths(&boxes, &diagram.lines);

    // Process boxes bottom-up (children before parents)
    let order = get_bottom_up_order(&boxes);

    for box_id in order {
        let box_metrics = &metrics[box_id];
        if box_metrics.required_width > box_metrics.current_width {
            let diff = box_metrics.required_width - box_metrics.current_width;
            expand_box(&mut diagram.lines, &boxes[box_id], diff);
            update_box_coordinates(&mut boxes, box_id, diff);
        }
    }

    Ok(())
}

/// Detect all closed boxes in the diagram
pub fn detect_boxes(lines: &[String]) -> Vec<Box> {
    let mut boxes = Vec::new();
    let chars: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    for (y, line) in chars.iter().enumerate() {
        for (x, &ch) in line.iter().enumerate() {
            if is_top_left_corner(ch) {
                if let Some(b) = try_detect_box(&chars, x, y) {
                    // Check if this box is not already detected (avoid duplicates)
                    let is_duplicate = boxes.iter().any(|existing: &Box| {
                        existing.top_left == b.top_left && existing.bottom_right == b.bottom_right
                    });
                    if !is_duplicate {
                        boxes.push(b);
                    }
                }
            }
        }
    }

    boxes
}

/// Check if character is a top-left corner
fn is_top_left_corner(ch: char) -> bool {
    matches!(ch, '┌' | '+' | '╔' | '╭')
}

/// Check if character is a top-right corner
fn is_top_right_corner(ch: char) -> bool {
    matches!(ch, '┐' | '+' | '╗' | '╮')
}

/// Check if character is a bottom-left corner
fn is_bottom_left_corner(ch: char) -> bool {
    matches!(ch, '└' | '+' | '╚' | '╰')
}

/// Check if character is a bottom-right corner
fn is_bottom_right_corner(ch: char) -> bool {
    matches!(ch, '┘' | '+' | '╝' | '╯')
}

/// Check if character is a horizontal line
fn is_horizontal(ch: char) -> bool {
    matches!(ch, '─' | '-' | '═' | '━' | '┬' | '┴' | '╦' | '╩' | '┼' | '╬')
}

/// Check if character is a vertical line
fn is_vertical(ch: char) -> bool {
    matches!(ch, '│' | '|' | '║' | '┃' | '├' | '┤' | '╠' | '╣' | '┼' | '╬')
}

fn try_detect_box(chars: &[Vec<char>], start_x: usize, start_y: usize) -> Option<Box> {
    let top_left = (start_x, start_y);

    let end_x = find_top_right(chars, start_x, start_y)?;
    let end_y = find_bottom_left(chars, start_x, start_y)?;

    if !verify_bottom_right(chars, end_x, end_y)
        || !verify_horizontal_edge(chars, start_x, end_x, end_y)
        || !verify_vertical_edge(chars, end_x, start_y, end_y)
    {
        return None;
    }

    Some(Box {
        id: 0,
        top_left,
        bottom_right: (end_x, end_y),
        content_lines: ((start_y + 1)..end_y).collect(),
        parent_id: None,
        children_ids: vec![],
    })
}

fn find_top_right(chars: &[Vec<char>], start_x: usize, start_y: usize) -> Option<usize> {
    let line = &chars[start_y];

    for (x, &ch) in line.iter().enumerate().skip(start_x + 1) {
        if is_top_right_corner(ch) {
            return Some(x);
        }
        if !is_horizontal(ch) && ch != ' ' {
            break;
        }
    }
    None
}

fn find_bottom_left(chars: &[Vec<char>], start_x: usize, start_y: usize) -> Option<usize> {
    for (y, row) in chars.iter().enumerate().skip(start_y + 1) {
        if start_x >= row.len() {
            break;
        }
        let ch = row[start_x];
        if is_bottom_left_corner(ch) {
            return Some(y);
        }
        if !is_vertical(ch) && ch != ' ' {
            break;
        }
    }
    None
}
    let line = &chars[start_y];

    for (x, &ch) in line.iter().enumerate().skip(start_x + 1) {
        if is_top_right_corner(ch) {
            return Some(x);
        }
        if !is_horizontal(ch) && ch != ' ' {
            break;
        }
    }
    None
}

fn find_bottom_left(chars: &[Vec<char>], start_x: usize, start_y: usize) -> Option<usize> {
    for (y, row) in chars.iter().enumerate().skip(start_y + 1) {
        if start_x >= row.len() {
            break;
        }
        let ch = row[start_x];
        if is_bottom_left_corner(ch) {
            return Some(y);
        }
        if !is_vertical(ch) && ch != ' ' {
            break;
        }
    }
    None
}

/// Verify that the bottom-right corner exists
fn verify_bottom_right(chars: &[Vec<char>], x: usize, y: usize) -> bool {
    if y >= chars.len() || x >= chars[y].len() {
        return false;
    }
    is_bottom_right_corner(chars[y][x])
}

/// Verify horizontal edge exists between two x positions
fn verify_horizontal_edge(chars: &[Vec<char>], start_x: usize, end_x: usize, y: usize) -> bool {
    if y >= chars.len() {
        return false;
    }
    let line = &chars[y];

    for x in (start_x + 1)..end_x {
        if x >= line.len() {
            return false;
        }
        if !is_horizontal(line[x]) && line[x] != ' ' {
            return false;
        }
    }
    true
}

/// Verify vertical edge exists between two y positions
fn verify_vertical_edge(chars: &[Vec<char>], x: usize, start_y: usize, end_y: usize) -> bool {
    for y in (start_y + 1)..end_y {
        if y >= chars.len() || x >= chars[y].len() {
            return false;
        }
        if !is_vertical(chars[y][x]) && chars[y][x] != ' ' {
            return false;
        }
    }
    true
}

pub fn build_hierarchy(boxes: &mut [Box]) {
    let n = boxes.len();

    let mut sorted_indices: Vec<usize> = (0..n).collect();
    sorted_indices.sort_by(|&a, &b| boxes[b].area().cmp(&boxes[a].area()));

    for &box_id in &sorted_indices {
        let mut parent_id: Option<usize> = None;
        let mut min_parent_area = usize::MAX;

        for &potential_parent_id in &sorted_indices {
            if potential_parent_id == box_id {
                continue;
            }

            if is_inside(&boxes[box_id], &boxes[potential_parent_id]) {
                let parent_area = boxes[potential_parent_id].area();
                if parent_area < min_parent_area {
                    min_parent_area = parent_area;
                    parent_id = Some(potential_parent_id);
                }
            }
        }

        boxes[box_id].parent_id = parent_id;
    }

    let parent_ids: Vec<Option<usize>> = boxes.iter().map(|b| b.parent_id).collect();
    for (child_id, parent_opt) in parent_ids.iter().enumerate() {
        if let Some(parent_id) = parent_opt {
            boxes[*parent_id].children_ids.push(child_id);
        }
    }
}

/// Check if inner box is strictly inside outer box
fn is_inside(inner: &Box, outer: &Box) -> bool {
    inner.top_left.0 > outer.top_left.0
        && inner.top_left.1 > outer.top_left.1
        && inner.bottom_right.0 < outer.bottom_right.0
        && inner.bottom_right.1 < outer.bottom_right.1
}

/// Calculate minimum widths for all boxes
pub fn calculate_min_widths(boxes: &[Box], lines: &[String]) -> Vec<BoxMetrics> {
    boxes
        .iter()
        .map(|b| {
            let current_width = b.width();
            let min_content_width = calculate_content_width(b, lines);

            // Required width = content + 2 (for │ on each side) + 2 (for padding spaces)
            let required_width = if min_content_width > 0 {
                min_content_width + 4
            } else {
                current_width
            };

            // Also consider children widths
            let max_child_required = b
                .children_ids
                .iter()
                .filter_map(|&child_id| {
                    if child_id < boxes.len() {
                        let child = &boxes[child_id];
                        let child_width = child.width();
                        // Child needs indent from parent borders
                        let indent = child.top_left.0 - b.top_left.0;
                        Some(child_width + indent * 2)
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or(0);

            let required_width = required_width.max(max_child_required);

            BoxMetrics {
                current_width,
                min_content_width,
                required_width,
            }
        })
        .collect()
}

/// Calculate the maximum content width inside a box
fn calculate_content_width(b: &Box, lines: &[String]) -> usize {
    let mut max_width = 0;

    for &line_idx in &b.content_lines {
        if line_idx >= lines.len() {
            continue;
        }
        let line = &lines[line_idx];
        let chars: Vec<char> = line.chars().collect();

        // Find content between the box borders
        let start_x = b.top_left.0 + 1;
        let end_x = b.bottom_right.0;

        if start_x >= chars.len() {
            continue;
        }

        let actual_end = end_x.min(chars.len());
        if start_x >= actual_end {
            continue;
        }

        let content: String = chars[start_x..actual_end].iter().collect();
        let trimmed = content.trim();

        if trimmed.starts_with('┌')
            || trimmed.starts_with('└')
            || trimmed.starts_with('+')
            || trimmed.starts_with('├')
        {
            continue;
        }

        let text_content = if trimmed.starts_with('│') || trimmed.starts_with('|') {
            let inner = trimmed.trim_start_matches('│').trim_start_matches('|');
            let inner = inner.trim_end_matches('│').trim_end_matches('|');
            inner.trim()
        } else {
            trimmed
        };

        let width = UnicodeWidthStr::width(text_content);
        max_width = max_width.max(width);
    }

    max_width
}

/// Get bottom-up ordering of boxes (children before parents)
fn get_bottom_up_order(boxes: &[Box]) -> Vec<usize> {
    let mut result = Vec::new();
    let mut visited = HashSet::new();

    fn visit(box_id: usize, boxes: &[Box], visited: &mut HashSet<usize>, result: &mut Vec<usize>) {
        if visited.contains(&box_id) {
            return;
        }
        visited.insert(box_id);

        // Visit children first
        for &child_id in &boxes[box_id].children_ids {
            visit(child_id, boxes, visited, result);
        }

        result.push(box_id);
    }

    // Start from root boxes (no parent)
    for (id, b) in boxes.iter().enumerate() {
        if b.parent_id.is_none() {
            visit(id, boxes, &mut visited, &mut result);
        }
    }

    result
}

fn expand_box(lines: &mut [String], b: &Box, diff: usize) {
    // Expand top border
    expand_horizontal_line(lines, b.top_left.0, b.top_left.1, b.width(), diff, '─');

    // Expand content lines
    for &line_idx in &b.content_lines {
        expand_content_line(lines, b.top_left.0, line_idx, b.width(), diff);
    }

    // Expand bottom border
    expand_horizontal_line(
        lines,
        b.top_left.0,
        b.bottom_right.1,
        b.width(),
        diff,
        '─',
    );
}

fn expand_horizontal_line(
    lines: &mut [String],
    start_x: usize,
    y: usize,
    old_width: usize,
    diff: usize,
    fill_char: char,
) {
    if y >= lines.len() {
        return;
    }

    let line = &lines[y];
    let chars: Vec<char> = line.chars().collect();

    let insert_pos = start_x + old_width - 1;

    if insert_pos > chars.len() {
        return;
    }

    let mut new_chars: Vec<char> = Vec::with_capacity(chars.len() + diff);

fn expand_content_line(
//! 1. Detect closed boxes in ASCII diagrams
//! 2. Build hierarchy of nested boxes
//! 3. Calculate minimum widths based on content
//! 4. Expand boxes to fit their content

use crate::error::Result;
use crate::parser::ParsedDiagram;
use std::collections::HashSet;
use unicode_width::UnicodeWidthStr;

/// Represents a detected box in the diagram
#[derive(Debug, Clone)]
pub struct Box {
    pub id: usize,
    pub top_left: (usize, usize),     // (x, y)
    pub bottom_right: (usize, usize), // (x, y)
    pub content_lines: Vec<usize>,    // indices of lines with content
    pub parent_id: Option<usize>,
    pub children_ids: Vec<usize>,
}

impl Box {
    /// Returns the width of the box (including borders)
    pub fn width(&self) -> usize {
        self.bottom_right.0.saturating_sub(self.top_left.0) + 1
    }

    /// Returns the height of the box (including borders)
    pub fn height(&self) -> usize {
        self.bottom_right.1.saturating_sub(self.top_left.1) + 1
    }

    /// Returns the area of the box
    pub fn area(&self) -> usize {
        self.width() * self.height()
    }
}

/// Metrics for box sizing calculations
#[derive(Debug, Clone)]
pub struct BoxMetrics {
    pub current_width: usize,
    pub min_content_width: usize, // max width of text inside
    pub required_width: usize,    // with padding
}

pub fn align_box_widths(diagram: &mut ParsedDiagram) -> Result<()> {
    let mut boxes = detect_boxes(&diagram.lines);

    if boxes.is_empty() {
        return Ok(());
    }

    // Assign IDs
    for (i, b) in boxes.iter_mut().enumerate() {
        b.id = i;
    }

    build_hierarchy(&mut boxes);
    let metrics = calculate_min_widths(&boxes, &diagram.lines);

    // Process boxes bottom-up (children before parents)
    let order = get_bottom_up_order(&boxes);

    for box_id in order {
        let box_metrics = &metrics[box_id];
        if box_metrics.required_width > box_metrics.current_width {
            let diff = box_metrics.required_width - box_metrics.current_width;
            expand_box(&mut diagram.lines, &boxes[box_id], diff);
            update_box_coordinates(&mut boxes, box_id, diff);
        }
    }

    Ok(())
}

/// Detect all closed boxes in the diagram
pub fn detect_boxes(lines: &[String]) -> Vec<Box> {
    let mut boxes = Vec::new();
    let chars: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    for (y, line) in chars.iter().enumerate() {
        for (x, &ch) in line.iter().enumerate() {
            if is_top_left_corner(ch) {
                if let Some(b) = try_detect_box(&chars, x, y) {
                    // Check if this box is not already detected (avoid duplicates)
                    let is_duplicate = boxes.iter().any(|existing: &Box| {
                        existing.top_left == b.top_left && existing.bottom_right == b.bottom_right
                    });
                    if !is_duplicate {
                        boxes.push(b);
                    }
                }
            }
        }
    }

    boxes
}

/// Check if character is a top-left corner
fn is_top_left_corner(ch: char) -> bool {
    matches!(ch, '┌' | '+' | '╔' | '╭')
}

/// Check if character is a top-right corner
fn is_top_right_corner(ch: char) -> bool {
    matches!(ch, '┐' | '+' | '╗' | '╮')
}

/// Check if character is a bottom-left corner
fn is_bottom_left_corner(ch: char) -> bool {
    matches!(ch, '└' | '+' | '╚' | '╰')
}

/// Check if character is a bottom-right corner
fn is_bottom_right_corner(ch: char) -> bool {
    matches!(ch, '┘' | '+' | '╝' | '╯')
}

/// Check if character is a horizontal line
fn is_horizontal(ch: char) -> bool {
    matches!(ch, '─' | '-' | '═' | '━' | '┬' | '┴' | '╦' | '╩' | '┼' | '╬')
}

/// Check if character is a vertical line
fn is_vertical(ch: char) -> bool {
    matches!(ch, '│' | '|' | '║' | '┃' | '├' | '┤' | '╠' | '╣' | '┼' | '╬')
}

fn try_detect_box(chars: &[Vec<char>], start_x: usize, start_y: usize) -> Option<Box> {
    let top_left = (start_x, start_y);

    let end_x = find_top_right(chars, start_x, start_y)?;
    let end_y = find_bottom_left(chars, start_x, start_y)?;

    if !verify_bottom_right(chars, end_x, end_y)
        || !verify_horizontal_edge(chars, start_x, end_x, end_y)
        || !verify_vertical_edge(chars, end_x, start_y, end_y)
    {
        return None;
    }

    Some(Box {
        id: 0,
        top_left,
        bottom_right: (end_x, end_y),
        content_lines: ((start_y + 1)..end_y).collect(),
        parent_id: None,
        children_ids: vec![],
    })
}

fn find_top_right(chars: &[Vec<char>], start_x: usize, start_y: usize) -> Option<usize> {
    let line = &chars[start_y];

    for (x, &ch) in line.iter().enumerate().skip(start_x + 1) {
        if is_top_right_corner(ch) {
            return Some(x);
        }
        if !is_horizontal(ch) && ch != ' ' {
            break;
        }
    }
    None
}

fn find_bottom_left(chars: &[Vec<char>], start_x: usize, start_y: usize) -> Option<usize> {
    for (y, row) in chars.iter().enumerate().skip(start_y + 1) {
        if start_x >= row.len() {
            break;
        }
        let ch = row[start_x];
        if is_bottom_left_corner(ch) {
            return Some(y);
        }
        if !is_vertical(ch) && ch != ' ' {
            break;
        }
    }
    None
}
    let line = &chars[start_y];

    for (x, &ch) in line.iter().enumerate().skip(start_x + 1) {
        if is_top_right_corner(ch) {
            return Some(x);
        }
        if !is_horizontal(ch) && ch != ' ' {
            break;
        }
    }
    None
}

fn find_bottom_left(chars: &[Vec<char>], start_x: usize, start_y: usize) -> Option<usize> {
    for (y, row) in chars.iter().enumerate().skip(start_y + 1) {
        if start_x >= row.len() {
            break;
        }
        let ch = row[start_x];
        if is_bottom_left_corner(ch) {
            return Some(y);
        }
        if !is_vertical(ch) && ch != ' ' {
            break;
        }
    }
    None
}

/// Verify that the bottom-right corner exists
fn verify_bottom_right(chars: &[Vec<char>], x: usize, y: usize) -> bool {
    if y >= chars.len() || x >= chars[y].len() {
        return false;
    }
    is_bottom_right_corner(chars[y][x])
}

/// Verify horizontal edge exists between two x positions
fn verify_horizontal_edge(chars: &[Vec<char>], start_x: usize, end_x: usize, y: usize) -> bool {
    if y >= chars.len() {
        return false;
    }
    let line = &chars[y];

    for x in (start_x + 1)..end_x {
        if x >= line.len() {
            return false;
        }
        if !is_horizontal(line[x]) && line[x] != ' ' {
            return false;
        }
    }
    true
}

/// Verify vertical edge exists between two y positions
fn verify_vertical_edge(chars: &[Vec<char>], x: usize, start_y: usize, end_y: usize) -> bool {
    for y in (start_y + 1)..end_y {
        if y >= chars.len() || x >= chars[y].len() {
            return false;
        }
        if !is_vertical(chars[y][x]) && chars[y][x] != ' ' {
            return false;
        }
    }
    true
}

pub fn build_hierarchy(boxes: &mut [Box]) {
    let n = boxes.len();

    let mut sorted_indices: Vec<usize> = (0..n).collect();
    sorted_indices.sort_by(|&a, &b| boxes[b].area().cmp(&boxes[a].area()));

    for &box_id in &sorted_indices {
        let mut parent_id: Option<usize> = None;
        let mut min_parent_area = usize::MAX;

        for &potential_parent_id in &sorted_indices {
            if potential_parent_id == box_id {
                continue;
            }

            if is_inside(&boxes[box_id], &boxes[potential_parent_id]) {
                let parent_area = boxes[potential_parent_id].area();
                if parent_area < min_parent_area {
                    min_parent_area = parent_area;
                    parent_id = Some(potential_parent_id);
                }
            }
        }

        boxes[box_id].parent_id = parent_id;
    }

    let parent_ids: Vec<Option<usize>> = boxes.iter().map(|b| b.parent_id).collect();
    for (child_id, parent_opt) in parent_ids.iter().enumerate() {
        if let Some(parent_id) = parent_opt {
            boxes[*parent_id].children_ids.push(child_id);
        }
    }
}

/// Check if inner box is strictly inside outer box
fn is_inside(inner: &Box, outer: &Box) -> bool {
    inner.top_left.0 > outer.top_left.0
        && inner.top_left.1 > outer.top_left.1
        && inner.bottom_right.0 < outer.bottom_right.0
        && inner.bottom_right.1 < outer.bottom_right.1
}

/// Calculate minimum widths for all boxes
pub fn calculate_min_widths(boxes: &[Box], lines: &[String]) -> Vec<BoxMetrics> {
    boxes
        .iter()
        .map(|b| {
            let current_width = b.width();
            let min_content_width = calculate_content_width(b, lines);

            // Required width = content + 2 (for │ on each side) + 2 (for padding spaces)
            let required_width = if min_content_width > 0 {
                min_content_width + 4
            } else {
                current_width
            };

            // Also consider children widths
            let max_child_required = b
                .children_ids
                .iter()
                .filter_map(|&child_id| {
                    if child_id < boxes.len() {
                        let child = &boxes[child_id];
                        let child_width = child.width();
                        // Child needs indent from parent borders
                        let indent = child.top_left.0 - b.top_left.0;
                        Some(child_width + indent * 2)
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or(0);

            let required_width = required_width.max(max_child_required);

            BoxMetrics {
                current_width,
                min_content_width,
                required_width,
            }
        })
        .collect()
}

/// Calculate the maximum content width inside a box
fn calculate_content_width(b: &Box, lines: &[String]) -> usize {
    let mut max_width = 0;

    for &line_idx in &b.content_lines {
        if line_idx >= lines.len() {
            continue;
        }
        let line = &lines[line_idx];
        let chars: Vec<char> = line.chars().collect();

        // Find content between the box borders
        let start_x = b.top_left.0 + 1;
        let end_x = b.bottom_right.0;

        if start_x >= chars.len() {
            continue;
        }

        let actual_end = end_x.min(chars.len());
        if start_x >= actual_end {
            continue;
        }

        let content: String = chars[start_x..actual_end].iter().collect();
        let trimmed = content.trim();

        if trimmed.starts_with('┌')
            || trimmed.starts_with('└')
            || trimmed.starts_with('+')
            || trimmed.starts_with('├')
        {
            continue;
        }

        let text_content = if trimmed.starts_with('│') || trimmed.starts_with('|') {
            let inner = trimmed.trim_start_matches('│').trim_start_matches('|');
            let inner = inner.trim_end_matches('│').trim_end_matches('|');
            inner.trim()
        } else {
            trimmed
        };

        let width = UnicodeWidthStr::width(text_content);
        max_width = max_width.max(width);
    }

    max_width
}

/// Get bottom-up ordering of boxes (children before parents)
fn get_bottom_up_order(boxes: &[Box]) -> Vec<usize> {
    let mut result = Vec::new();
    let mut visited = HashSet::new();

    fn visit(box_id: usize, boxes: &[Box], visited: &mut HashSet<usize>, result: &mut Vec<usize>) {
        if visited.contains(&box_id) {
            return;
        }
        visited.insert(box_id);

        // Visit children first
        for &child_id in &boxes[box_id].children_ids {
            visit(child_id, boxes, visited, result);
        }

        result.push(box_id);
    }

    // Start from root boxes (no parent)
    for (id, b) in boxes.iter().enumerate() {
        if b.parent_id.is_none() {
            visit(id, boxes, &mut visited, &mut result);
        }
    }

    result
}

fn expand_box(lines: &mut [String], b: &Box, diff: usize) {
    // Expand top border
    expand_horizontal_line(lines, b.top_left.0, b.top_left.1, b.width(), diff, '─');

    // Expand content lines
    for &line_idx in &b.content_lines {
        expand_content_line(lines, b.top_left.0, line_idx, b.width(), diff);
    }

    // Expand bottom border
    expand_horizontal_line(
        lines,
        b.top_left.0,
        b.bottom_right.1,
        b.width(),
        diff,
        '─',
    );
}

fn expand_horizontal_line(
    lines: &mut [String],
    start_x: usize,
    y: usize,
    old_width: usize,
    diff: usize,
    fill_char: char,
) {
    if y >= lines.len() {
        return;
    }

    let line = &lines[y];
    let chars: Vec<char> = line.chars().collect();

    let insert_pos = start_x + old_width - 1;

    if insert_pos > chars.len() {
        return;
    }

    let mut new_chars: Vec<char> = Vec::with_capacity(chars.len() + diff);

    for (i, &ch) in chars.iter().enumerate() {
        if i == insert_pos {
            new_chars.extend(std::iter::repeat_n(fill_char, diff));
        }
        new_chars.push(ch);
    }

    lines[y] = new_chars.into_iter().collect();
}

fn expand_content_line(
    lines: &mut [String],
    start_x: usize,
    y: usize,
    old_width: usize,
    diff: usize,
) {
    if y >= lines.len() {
        return;
    }

    let line = &lines[y];
    let chars: Vec<char> = line.chars().collect();

    let insert_pos = start_x + old_width - 1;

    if insert_pos > chars.len() {
        return;
    }

    let mut new_chars: Vec<char> = Vec::with_capacity(chars.len() + diff);

    for (i, &ch) in chars.iter().enumerate() {
        if i == insert_pos {
            new_chars.extend(std::iter::repeat_n(' ', diff));
        }
        new_chars.push(ch);
    }

    lines[y] = new_chars.into_iter().collect();
}

fn update_box_coordinates(boxes: &mut [Box], expanded_box_id: usize, diff: usize) {
    let expanded_box = boxes[expanded_box_id].clone();

    for (id, b) in boxes.iter_mut().enumerate() {
        if id == expanded_box_id {
            b.bottom_right.0 += diff;
            continue;
        }

        if b.top_left.0 > expanded_box.bottom_right.0 {
            b.top_left.0 += diff;
            b.bottom_right.0 += diff;
        }
    }
}

fn expand_content_line(
    lines: &mut [String],
    start_x: usize,
    y: usize,
    old_width: usize,
    diff: usize,
) {
    if y >= lines.len() {
        return;
    }

    let line = &lines[y];
    let chars: Vec<char> = line.chars().collect();

    let insert_pos = start_x + old_width - 1;

    if insert_pos > chars.len() {
        return;
    }

    let mut new_chars: Vec<char> = Vec::with_capacity(chars.len() + diff);

    for (i, &ch) in chars.iter().enumerate() {
        if i == insert_pos {
            new_chars.extend(std::iter::repeat_n(' ', diff));
        }
        new_chars.push(ch);
    }

    lines[y] = new_chars.into_iter().collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patterns::DiagramType;

    fn make_diagram(input: &str) -> ParsedDiagram {
        ParsedDiagram {
            lines: input.lines().map(|s| s.to_string()).collect(),
            diagram_type: DiagramType::Unknown,
        }
    }

    #[test]
    fn test_detect_simple_box() {
        let lines: Vec<String> = vec![
            "┌───┐".to_string(),
            "│ A │".to_string(),
            "└───┘".to_string(),
        ];

        let boxes = detect_boxes(&lines);
        assert_eq!(boxes.len(), 1);
        assert_eq!(boxes[0].top_left, (0, 0));
        assert_eq!(boxes[0].bottom_right, (4, 2));
    }

    #[test]
    fn test_detect_nested_boxes() {
        let lines: Vec<String> = vec![
            "┌─────────┐".to_string(),
            "│ ┌─────┐ │".to_string(),
            "│ │ ABC │ │".to_string(),
            "│ └─────┘ │".to_string(),
            "└─────────┘".to_string(),
        ];

        let boxes = detect_boxes(&lines);
        assert_eq!(boxes.len(), 2);
    }

    #[test]
    fn test_build_hierarchy_nested() {
        let lines: Vec<String> = vec![
            "┌─────────┐".to_string(),
            "│ ┌─────┐ │".to_string(),
            "│ │ ABC │ │".to_string(),
            "│ └─────┘ │".to_string(),
            "└─────────┘".to_string(),
        ];

        let mut boxes = detect_boxes(&lines);
        for (i, b) in boxes.iter_mut().enumerate() {
            b.id = i;
        }
        build_hierarchy(&mut boxes);

        // The smaller box should have the larger as parent
        let inner_box = boxes.iter().find(|b| b.area() < 50).unwrap();
        assert!(inner_box.parent_id.is_some());
    }

    #[test]
    fn test_calculate_content_width() {
        let lines: Vec<String> = vec![
            "┌────────────────┐".to_string(),
            "│ Hello World    │".to_string(),
            "└────────────────┘".to_string(),
        ];

        let boxes = detect_boxes(&lines);
        assert_eq!(boxes.len(), 1);

        let width = calculate_content_width(&boxes[0], &lines);
        assert_eq!(width, 11); // "Hello World"
    }

    #[test]
    fn test_box_width_and_height() {
        let b = Box {
            id: 0,
            top_left: (0, 0),
            bottom_right: (4, 2),
            content_lines: vec![1],
            parent_id: None,
            children_ids: vec![],
        };

        assert_eq!(b.width(), 5);
        assert_eq!(b.height(), 3);
    }

    #[test]
    fn test_is_inside() {
        let outer = Box {
            id: 0,
            top_left: (0, 0),
            bottom_right: (10, 5),
            content_lines: vec![],
            parent_id: None,
            children_ids: vec![],
        };

        let inner = Box {
            id: 1,
            top_left: (2, 1),
            bottom_right: (8, 4),
            content_lines: vec![],
            parent_id: None,
            children_ids: vec![],
        };

        let outside = Box {
            id: 2,
            top_left: (15, 0),
            bottom_right: (20, 5),
            content_lines: vec![],
            parent_id: None,
            children_ids: vec![],
        };

        assert!(is_inside(&inner, &outer));
        assert!(!is_inside(&outer, &inner));
        assert!(!is_inside(&outside, &outer));
    }

    #[test]
    fn test_align_box_widths_no_change_needed() {
        let mut diagram = make_diagram("┌───┐\n│ A │\n└───┘");
        let result = align_box_widths(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_detect_boxes_with_plus_corners() {
        let lines: Vec<String> = vec![
            "+---+".to_string(),
            "| A |".to_string(),
            "+---+".to_string(),
        ];

        let boxes = detect_boxes(&lines);
        assert_eq!(boxes.len(), 1);
    }

    #[test]
    fn test_empty_diagram() {
        let mut diagram = make_diagram("Hello World");
        let result = align_box_widths(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_bottom_up_order() {
        let boxes = vec![
            Box {
                id: 0,
                top_left: (0, 0),
                bottom_right: (10, 5),
                content_lines: vec![],
                parent_id: None,
                children_ids: vec![1],
            },
            Box {
                id: 1,
                top_left: (2, 1),
                bottom_right: (8, 4),
                content_lines: vec![],
                parent_id: Some(0),
                children_ids: vec![],
            },
        ];

        let order = get_bottom_up_order(&boxes);
        // Child should come before parent
        assert_eq!(order, vec![1, 0]);
    }
}
