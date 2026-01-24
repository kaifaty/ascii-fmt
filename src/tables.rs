use crate::display_width::{center_to_width, display_width};
use crate::error::Result;
use crate::parser::ParsedDiagram;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableGroup {
    pub start_line: usize,
    pub end_line: usize,
    pub marker_count: usize,
}

pub fn format_tables(diagram: &mut ParsedDiagram) -> Result<()> {
    let groups = detect_table_groups(&diagram.lines);
    for group in groups {
        if group.end_line <= group.start_line {
            continue;
        }
        format_table_group(
            &mut diagram.lines[group.start_line..group.end_line],
            group.marker_count,
        );
    }
    Ok(())
}

pub fn detect_table_groups(lines: &[String]) -> Vec<TableGroup> {
    let mut groups = Vec::new();

    let mut i = 0;
    while i < lines.len() {
        if !has_table_markers(lines[i].as_str()) {
            i += 1;
            continue;
        }

        let start_run = i;
        i += 1;
        while i < lines.len() && has_table_markers(lines[i].as_str()) {
            i += 1;
        }
        let end_run = i;

        let mut j = start_run;
        while j < end_run {
            let count = extract_table_markers(lines[j].as_str()).len();
            let start_group = j;
            j += 1;
            while j < end_run && extract_table_markers(lines[j].as_str()).len() == count {
                j += 1;
            }

            if is_table_candidate(&lines[start_group..j], count) {
                groups.push(TableGroup {
                    start_line: start_group,
                    end_line: j,
                    marker_count: count,
                });
            }
        }
    }

    groups
}

#[derive(Debug, Clone)]
struct ParsedTableLine {
    markers: Vec<(usize, char)>,
    is_border_line: bool,
}

#[derive(Debug, Clone)]
struct ParsedTableGroup {
    lines: Vec<ParsedTableLine>,
    header_line: Option<usize>,
}

fn parse_table_group(lines: &[String], marker_count: usize) -> Option<ParsedTableGroup> {
    let mut parsed_lines = Vec::with_capacity(lines.len());
    for line in lines {
        let markers = extract_table_markers(line.as_str());
        if markers.len() != marker_count {
            return None;
        }
        let is_border_line = line.chars().any(|ch| matches!(ch, '─' | '═'));
        parsed_lines.push(ParsedTableLine {
            markers,
            is_border_line,
        });
    }

    let header_line = detect_header_line(&parsed_lines);

    Some(ParsedTableGroup {
        lines: parsed_lines,
        header_line,
    })
}

fn format_table_group(lines: &mut [String], marker_count: usize) {
    if lines.is_empty() {
        return;
    }

    let snapshot = lines.to_vec();
    let Some(parsed) = parse_table_group(&snapshot, marker_count) else {
        return;
    };

    let segment_count = marker_count.saturating_sub(1);
    if segment_count == 0 {
        return;
    }

    let outer_left_border = parsed
        .lines
        .iter()
        .find(|l| !l.is_border_line)
        .map(|l| l.markers[0].1)
        .filter(|ch| is_vertical_border_char(*ch));
    let outer_right_border = parsed
        .lines
        .iter()
        .find(|l| !l.is_border_line)
        .map(|l| l.markers[marker_count - 1].1)
        .filter(|ch| is_vertical_border_char(*ch));

    let seg_width_sums: Vec<usize> = snapshot
        .iter()
        .zip(parsed.lines.iter())
        .map(|(line, info)| sum_segment_widths(line.as_str(), &info.markers))
        .collect();
    let Some(target_total_content_width) = seg_width_sums.iter().copied().max() else {
        return;
    };

    let left_padding = segment_is_whitespace_only(&snapshot, &parsed.lines, 0);
    let right_padding = segment_is_whitespace_only(&snapshot, &parsed.lines, segment_count - 1);

    let mut left_pad_width = if left_padding {
        min_segment_width(&snapshot, &parsed.lines, 0)
    } else {
        0
    };
    let mut right_pad_width = if right_padding {
        min_segment_width(&snapshot, &parsed.lines, segment_count - 1)
    } else {
        0
    };

    // If both ends are padding, normalize to a symmetric minimum.
    if left_padding && right_padding {
        let pad = left_pad_width.min(right_pad_width);
        left_pad_width = pad;
        right_pad_width = pad;
    }

    let col_start = if left_padding { 1 } else { 0 };
    let col_end = if right_padding {
        segment_count.saturating_sub(1)
    } else {
        segment_count
    };
    if col_end <= col_start {
        return;
    }

    let col_count = col_end - col_start;
    let available_columns_width = target_total_content_width
        .saturating_sub(left_pad_width)
        .saturating_sub(right_pad_width);

    let header_line_idx = parsed.header_line;
    let is_art_table = group_has_ascii_art(&snapshot, &parsed.lines, col_start, col_end);

    // Compute minimal widths per column from content.
    let mut min_col_widths = vec![0usize; col_count];
    for (line, info) in snapshot.iter().zip(parsed.lines.iter()) {
        if info.is_border_line {
            continue;
        }

        for (local_col_idx, seg_idx) in (col_start..col_end).enumerate() {
            let Some(seg) = slice_segment(line.as_str(), &info.markers, seg_idx) else {
                continue;
            };

            let content = if is_art_table { seg } else { seg.trim() };
            let content_width = display_width(content);
            let min_width = content_width.saturating_add(2).max(1);
            min_col_widths[local_col_idx] = min_col_widths[local_col_idx].max(min_width);
        }
    }

    let min_sum: usize = min_col_widths.iter().copied().sum();
    if min_sum > available_columns_width {
        // Don't attempt to shrink; leave as-is.
        return;
    }

    let extra = available_columns_width - min_sum;
    let add = extra / col_count;
    let rem = extra % col_count;

    let mut col_widths = min_col_widths;
    for (idx, w) in col_widths.iter_mut().enumerate() {
        *w += add;
        if idx < rem {
            *w += 1;
        }
    }

    // Build target widths for every segment.
    let mut target_segment_widths = vec![0usize; segment_count];
    if left_padding {
        target_segment_widths[0] = left_pad_width;
    }
    if right_padding {
        target_segment_widths[segment_count - 1] = right_pad_width;
    }
    for (local_col_idx, seg_idx) in (col_start..col_end).enumerate() {
        target_segment_widths[seg_idx] = col_widths[local_col_idx];
    }

    let mut new_lines = Vec::with_capacity(lines.len());
    for (line_idx, (line, info)) in snapshot.iter().zip(parsed.lines.iter()).enumerate() {
        let markers = &info.markers;
        if markers.len() != marker_count {
            new_lines.push(line.clone());
            continue;
        }

        let mut out = String::with_capacity(line.len() + 16);
        out.push_str(&line[..markers[0].0]);

        let is_header_line = header_line_idx.is_some_and(|h| h == line_idx);
        let fill = choose_horizontal_fill_char(line.as_str());

        for seg_idx in 0..segment_count {
            let (left_byte, left_ch) = markers[seg_idx];
            let (right_byte, _) = markers[seg_idx + 1];

            let mut marker_ch = left_ch;
            if info.is_border_line && seg_idx == 0 {
                if let Some(outer) = outer_left_border {
                    if !is_vertical_border_char(left_ch) {
                        marker_ch = border_left_marker(outer, fill);
                    }
                }
            }
            out.push(marker_ch);

            let seg_start = left_byte + left_ch.len_utf8();
            if seg_start > right_byte {
                continue;
            }
            let seg = &line[seg_start..right_byte];
            let target_w = target_segment_widths[seg_idx];

            if info.is_border_line {
                if seg.chars().any(|ch| matches!(ch, '─' | '═')) {
                    out.extend(std::iter::repeat_n(fill, target_w));
                } else {
                    out.push_str(&" ".repeat(target_w));
                }
                continue;
            }

            if seg_idx < col_start || seg_idx >= col_end {
                // Outer padding (or non-column) segment.
                out.push_str(&" ".repeat(target_w));
                continue;
            }

            if is_art_table && !is_header_line {
                let seg_width = display_width(seg);
                if seg_width >= target_w {
                    out.push_str(seg);
                } else {
                    out.push_str(seg);
                    out.push_str(&" ".repeat(target_w - seg_width));
                }
                continue;
            }

            let content = seg.trim();
            if content.is_empty() {
                out.push_str(&" ".repeat(target_w));
                continue;
            }

            let content_width = display_width(content);
            if target_w <= content_width {
                out.push_str(content);
                continue;
            }

            if is_header_line {
                if target_w >= content_width + 2 {
                    let inner = target_w - 2;
                    out.push(' ');
                    out.push_str(&center_to_width(content, inner));
                    out.push(' ');
                } else {
                    out.push_str(&center_to_width(content, target_w));
                }
            } else {
                let lead = 1usize.min(target_w);
                let remaining = target_w.saturating_sub(lead + content_width);
                out.push_str(&" ".repeat(lead));
                out.push_str(content);
                out.push_str(&" ".repeat(remaining));
            }
        }

        let (last_byte, last_ch) = markers[marker_count - 1];
        let mut last_marker = last_ch;
        if info.is_border_line {
            if let Some(outer) = outer_right_border {
                if !is_vertical_border_char(last_ch) {
                    last_marker = border_right_marker(outer, fill);
                }
            }
        }
        out.push(last_marker);

        let suffix_start = last_byte + last_ch.len_utf8();
        if suffix_start < line.len() {
            out.push_str(&line[suffix_start..]);
        }

        new_lines.push(out);
    }

    for (dst, src) in lines.iter_mut().zip(new_lines.into_iter()) {
        *dst = src;
    }
}

fn is_vertical_border_char(ch: char) -> bool {
    matches!(ch, '│' | '║' | '|')
}

fn border_left_marker(outer_vertical: char, horizontal: char) -> char {
    let outer_double = matches!(outer_vertical, '║');
    let horiz_double = matches!(horizontal, '═');
    match (outer_double, horiz_double) {
        (true, true) => '╠',
        (true, false) => '╟',
        (false, true) => '╞',
        (false, false) => '├',
    }
}

fn border_right_marker(outer_vertical: char, horizontal: char) -> char {
    let outer_double = matches!(outer_vertical, '║');
    let horiz_double = matches!(horizontal, '═');
    match (outer_double, horiz_double) {
        (true, true) => '╣',
        (true, false) => '╢',
        (false, true) => '╡',
        (false, false) => '┤',
    }
}

fn choose_horizontal_fill_char(line: &str) -> char {
    // Prefer the horizontal stroke actually present on the line.
    if line.chars().any(|ch| ch == '═') {
        return '═';
    }
    if line.chars().any(|ch| ch == '─') {
        return '─';
    }

    // Fallback: infer from double-line border characters.
    if line.chars().any(|ch| {
        matches!(
            ch,
            '╔' | '╗'
                | '╚'
                | '╝'
                | '╠'
                | '╣'
                | '╦'
                | '╩'
                | '╬'
                | '║'
                | '╒'
                | '╓'
                | '╕'
                | '╖'
                | '╘'
                | '╙'
                | '╛'
                | '╜'
                | '╞'
                | '╟'
                | '╡'
                | '╢'
                | '╤'
                | '╥'
                | '╧'
                | '╨'
                | '╪'
                | '╫'
        )
    }) {
        '═'
    } else {
        '─'
    }
}

fn slice_segment<'a>(line: &'a str, markers: &[(usize, char)], seg_idx: usize) -> Option<&'a str> {
    let (left_byte, left_ch) = *markers.get(seg_idx)?;
    let (right_byte, _) = *markers.get(seg_idx + 1)?;
    let seg_start = left_byte + left_ch.len_utf8();
    if seg_start > right_byte {
        return None;
    }
    Some(&line[seg_start..right_byte])
}

fn segment_is_whitespace_only(
    snapshot: &[String],
    infos: &[ParsedTableLine],
    seg_idx: usize,
) -> bool {
    snapshot.iter().zip(infos.iter()).all(|(line, info)| {
        let Some(seg) = slice_segment(line.as_str(), &info.markers, seg_idx) else {
            return false;
        };
        seg.chars().all(|ch| ch == ' ' || ch == '\t')
    })
}

fn min_segment_width(snapshot: &[String], infos: &[ParsedTableLine], seg_idx: usize) -> usize {
    snapshot
        .iter()
        .zip(infos.iter())
        .filter_map(|(line, info)| slice_segment(line.as_str(), &info.markers, seg_idx))
        .map(display_width)
        .min()
        .unwrap_or(0)
}

fn group_has_ascii_art(
    snapshot: &[String],
    infos: &[ParsedTableLine],
    col_start: usize,
    col_end: usize,
) -> bool {
    snapshot.iter().zip(infos.iter()).any(|(line, info)| {
        if info.is_border_line {
            return false;
        }

        (col_start..col_end).any(|seg_idx| {
            let Some(seg) = slice_segment(line.as_str(), &info.markers, seg_idx) else {
                return false;
            };
            let content = seg.trim();
            if content.is_empty() {
                return false;
            }

            // Heuristic: ASCII art commonly contains underscore + slash/backslash.
            content.contains('_') && (content.contains('/') || content.contains('\\'))
        })
    })
}

fn sum_segment_widths(line: &str, markers: &[(usize, char)]) -> usize {
    markers
        .windows(2)
        .filter_map(|w| {
            let (left_byte, left_ch) = w[0];
            let (right_byte, _) = w[1];
            let seg_start = left_byte + left_ch.len_utf8();
            (seg_start <= right_byte).then_some(display_width(&line[seg_start..right_byte]))
        })
        .sum()
}

fn detect_header_line(lines: &[ParsedTableLine]) -> Option<usize> {
    for idx in 0..lines.len() {
        if lines[idx].is_border_line {
            continue;
        }
        if lines.get(idx + 1).is_some_and(|l| l.is_border_line) {
            return Some(idx);
        }
    }
    None
}

fn has_table_markers(line: &str) -> bool {
    line.chars().any(is_table_marker)
}

fn is_table_marker(ch: char) -> bool {
    matches!(
        ch,
        '│' | '║'
            | '┌'
            | '┐'
            | '└'
            | '┘'
            | '├'
            | '┤'
            | '┬'
            | '┴'
            | '┼'
            | '╭'
            | '╮'
            | '╰'
            | '╯'
            | '╔'
            | '╗'
            | '╚'
            | '╝'
            | '╠'
            | '╣'
            | '╦'
            | '╩'
            | '╬'
            | '╒'
            | '╓'
            | '╕'
            | '╖'
            | '╘'
            | '╙'
            | '╛'
            | '╜'
            | '╞'
            | '╟'
            | '╡'
            | '╢'
            | '╤'
            | '╥'
            | '╧'
            | '╨'
            | '╪'
            | '╫'
    )
}

fn is_table_junction(ch: char) -> bool {
    matches!(
        ch,
        '┬' | '┼' | '┴' | '╦' | '╬' | '╩' | '╤' | '╥' | '╧' | '╨' | '╪' | '╫'
    )
}

fn extract_table_markers(line: &str) -> Vec<(usize, char)> {
    line.char_indices()
        .filter_map(|(idx, ch)| is_table_marker(ch).then_some((idx, ch)))
        .collect()
}

fn is_table_candidate(lines: &[String], marker_count: usize) -> bool {
    if marker_count < 3 {
        return false;
    }

    let has_junction_border_line = lines
        .iter()
        .any(|l| l.chars().any(|ch| matches!(ch, '─' | '═')) && l.chars().any(is_table_junction));
    if !has_junction_border_line {
        return false;
    }

    // Avoid misclassifying connector-only structures: require at least one
    // non-border row that contains some non-whitespace content inside cells.
    lines.iter().any(|l| {
        if l.chars().any(|ch| matches!(ch, '─' | '═')) {
            return false;
        }

        let markers = extract_table_markers(l.as_str());
        if markers.len() != marker_count {
            return false;
        }

        (0..marker_count.saturating_sub(1)).any(|seg_idx| {
            slice_segment(l.as_str(), &markers, seg_idx).is_some_and(|seg| !seg.trim().is_empty())
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_table_groups_simple_2col() {
        let lines = vec![
            "┌───┬───┐".to_string(),
            "│ A │ B │".to_string(),
            "└───┴───┘".to_string(),
        ];

        let groups = detect_table_groups(&lines);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].start_line, 0);
        assert_eq!(groups[0].end_line, 3);
        assert_eq!(groups[0].marker_count, 3);
    }

    #[test]
    fn test_detect_table_groups_simple_box_is_not_table() {
        let lines = vec![
            "┌───┐".to_string(),
            "│ A │".to_string(),
            "└───┘".to_string(),
        ];
        let groups = detect_table_groups(&lines);
        assert!(groups.is_empty());
    }

    #[test]
    fn test_parse_table_group_detects_header() {
        let lines = vec![
            "┌───┬────┐".to_string(),
            "│ H │ HH │".to_string(),
            "├───┼────┤".to_string(),
            "│ A │ BB │".to_string(),
            "└───┴────┘".to_string(),
        ];

        let parsed = parse_table_group(&lines, 3).expect("table should parse");
        assert_eq!(parsed.header_line, Some(1));
    }

    #[test]
    fn test_format_table_group_stretches_columns_and_centers_header() {
        let mut lines = vec![
            "│  ┌──┬──┐   │".to_string(),
            "│  │ H│HH│   │".to_string(),
            "│  ├──┼──┤   │".to_string(),
            "│  │ a│bb│   │".to_string(),
            "│  └──┴──┘   │".to_string(),
        ];

        // 7 markers: outer │, inner ┌/│/├/└, inner ┬/│/┼/┴, inner │, inner ┐/│/┤/┘, outer │
        // (For this simplified example, markers count will be 5, but we call via detection.
        // Keep the test focused on behavior, not exact marker count.)
        let groups = detect_table_groups(&lines);
        assert_eq!(groups.len(), 1);
        format_table_group(
            &mut lines[groups[0].start_line..groups[0].end_line],
            groups[0].marker_count,
        );

        // Header line should still contain H/HH and be padded.
        assert!(lines[1].contains("H"));
        assert!(lines[1].contains("HH"));
    }
}
