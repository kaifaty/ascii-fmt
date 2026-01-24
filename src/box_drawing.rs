use crate::error::Result;
use crate::parser::ParsedDiagram;
use crate::utils::{get_char_at, is_diagonal, is_horizontal, is_vertical};

pub fn fix_box_drawing_symbols(diagram: &mut ParsedDiagram) -> Result<()> {
    let original_lines = diagram.lines.clone();
    let temp_diagram = ParsedDiagram {
        lines: original_lines.clone(),
        diagram_type: diagram.diagram_type,
    };
    let mut new_lines = Vec::with_capacity(diagram.lines.len());

    for (y, line) in original_lines.iter().enumerate() {
        let unicode_verticals: Vec<usize> = line
            .chars()
            .enumerate()
            .filter_map(|(x, ch)| is_unicode_vertical_border(ch).then_some(x))
            .collect();
        let mut new_line = String::with_capacity(line.len());

        for (x, ch) in line.chars().enumerate() {
            let fixed = fix_char(ch, x, y, &temp_diagram, &unicode_verticals)?;
            new_line.push(fixed);
        }

        new_lines.push(new_line);
    }

    diagram.lines = new_lines;
    Ok(())
}

fn fix_char(
    ch: char,
    x: usize,
    y: usize,
    diagram: &ParsedDiagram,
    unicode_verticals: &[usize],
) -> Result<char> {
    // Inside a Unicode box cell, treat ASCII pseudo-graphics as content.
    // This prevents accidental conversion of ASCII art inside already boxed diagrams.
    if is_ascii_pseudo_graphics(ch) && is_inside_unicode_vertical_borders(x, unicode_verticals) {
        return Ok(ch);
    }

    match ch {
        '+' => fix_plus(x, y, diagram),
        '-' | '_' => {
            if should_fix_horizontal(ch, x, y, diagram) {
                fix_horizontal(x, y, diagram)
            } else {
                Ok(ch)
            }
        }
        '|' | '!' => {
            if should_fix_vertical(ch, x, y, diagram) {
                fix_vertical(x, y, diagram)
            } else {
                Ok(ch)
            }
        }
        '/' | '\\' => {
            if should_fix_diagonal(ch, x, y, diagram) {
                fix_diagonal(ch)
            } else {
                Ok(ch)
            }
        }
        c if is_diagonal(c) => fix_diagonal(c),
        c if is_arrow_like(c) => {
            if should_fix_arrow(c, x, y, diagram) {
                fix_arrow(c)
            } else {
                Ok(c)
            }
        }
        _ => Ok(ch),
    }
}

fn is_unicode_vertical_border(ch: char) -> bool {
    matches!(ch, '│' | '║')
}

fn is_inside_unicode_vertical_borders(x: usize, unicode_verticals: &[usize]) -> bool {
    if unicode_verticals.len() < 2 {
        return false;
    }

    let idx = match unicode_verticals.binary_search(&x) {
        Ok(i) => i,
        Err(i) => i,
    };

    idx > 0 && idx < unicode_verticals.len()
}

fn is_ascii_pseudo_graphics(ch: char) -> bool {
    matches!(
        ch,
        '+' | '-' | '_' | '|' | '!' | '/' | '\\' | 'v' | '^' | '<' | '>'
    )
}

fn should_fix_horizontal(ch: char, x: usize, y: usize, diagram: &ParsedDiagram) -> bool {
    let left = x
        .checked_sub(1)
        .and_then(|new_x| get_neighbor_char(new_x, y, diagram));
    let right = get_neighbor_char(x + 1, y, diagram);
    let up = y
        .checked_sub(1)
        .and_then(|new_y| get_neighbor_char(x, new_y, diagram));
    let down = get_neighbor_char(x, y + 1, diagram);

    if is_word_boundary_char(left) && is_word_boundary_char(right) {
        return false;
    }

    let has_line_neighbor = left.is_some_and(is_horizontal_like)
        || right.is_some_and(is_horizontal_like)
        || up.is_some_and(is_vertical_like)
        || down.is_some_and(is_vertical_like)
        || left.is_some_and(is_arrow_like)
        || right.is_some_and(is_arrow_like);

    // Underscores/hyphens in free text should be preserved unless they connect to a line.
    match ch {
        '-' | '_' => has_line_neighbor,
        _ => has_line_neighbor,
    }
}

fn should_fix_vertical(_ch: char, x: usize, y: usize, diagram: &ParsedDiagram) -> bool {
    let left = x
        .checked_sub(1)
        .and_then(|new_x| get_neighbor_char(new_x, y, diagram));
    let right = get_neighbor_char(x + 1, y, diagram);
    let up = y
        .checked_sub(1)
        .and_then(|new_y| get_neighbor_char(x, new_y, diagram));
    let down = get_neighbor_char(x, y + 1, diagram);

    // Don't convert separators inside words/identifiers.
    if is_word_boundary_char(left) || is_word_boundary_char(right) {
        return false;
    }

    // Convert when connected to surrounding lines or used standalone.
    left.is_some_and(is_horizontal_like)
        || right.is_some_and(is_horizontal_like)
        || up.is_some_and(is_vertical_like)
        || down.is_some_and(is_vertical_like)
        || left.is_none_or(|c| c == ' ')
        || right.is_none_or(|c| c == ' ')
}

fn should_fix_diagonal(_ch: char, x: usize, y: usize, diagram: &ParsedDiagram) -> bool {
    let left = x
        .checked_sub(1)
        .and_then(|new_x| get_neighbor_char(new_x, y, diagram));
    let right = get_neighbor_char(x + 1, y, diagram);
    // Treat slashes in identifiers/paths as text.
    if is_word_boundary_char(left) || is_word_boundary_char(right) {
        return false;
    }
    true
}

fn is_word_boundary_char(ch: Option<char>) -> bool {
    ch.is_some_and(|c| c.is_alphanumeric())
}

fn is_horizontal_like(ch: char) -> bool {
    is_horizontal(ch)
        || matches!(
            ch,
            '┌' | '┐' | '└' | '┘' | '┬' | '┴' | '┼' | '+' | '├' | '┤'
        )
}

fn is_vertical_like(ch: char) -> bool {
    is_vertical(ch)
        || matches!(
            ch,
            '┌' | '┐' | '└' | '┘' | '┬' | '┴' | '┼' | '+' | '├' | '┤'
        )
}

fn should_fix_arrow(ch: char, x: usize, y: usize, diagram: &ParsedDiagram) -> bool {
    let left = x
        .checked_sub(1)
        .and_then(|new_x| get_neighbor_char(new_x, y, diagram));
    let right = get_neighbor_char(x + 1, y, diagram);

    // If it is inside a word/identifier, keep it.
    if is_word_boundary_char(left) && is_word_boundary_char(right) {
        return false;
    }

    match ch {
        // Letters in words (e.g. Service) should never turn into arrows.
        'v' | '^' => !(is_word_boundary_char(left) || is_word_boundary_char(right)),
        '<' | '>' => {
            left.is_some_and(is_horizontal_like)
                || right.is_some_and(is_horizontal_like)
                || !(is_word_boundary_char(left) || is_word_boundary_char(right))
        }
        _ => true,
    }
}

fn fix_plus(x: usize, y: usize, diagram: &ParsedDiagram) -> Result<char> {
    let left = x
        .checked_sub(1)
        .and_then(|new_x| get_neighbor_char(new_x, y, diagram));
    let right = get_neighbor_char(x + 1, y, diagram);
    let up = y
        .checked_sub(1)
        .and_then(|new_y| get_neighbor_char(x, new_y, diagram));
    let down = get_neighbor_char(x, y + 1, diagram);

    let has_left = left.is_some_and(is_horizontal);
    let has_right = right.is_some_and(is_horizontal);
    let has_up = up.is_some_and(is_vertical);
    let has_down = down.is_some_and(is_vertical);

    match (has_left, has_right, has_up, has_down) {
        (true, true, true, true) => Ok('┼'),
        (true, true, false, false) => Ok('─'),
        (false, false, true, true) => Ok('│'),
        (true, true, true, false) => Ok('┴'),
        (true, true, false, true) => Ok('┬'),
        (true, false, true, true) => Ok('┤'),
        (false, true, true, true) => Ok('├'),
        (true, false, true, false) => Ok('┘'),
        (true, false, false, true) => Ok('┐'),
        (false, true, true, false) => Ok('└'),
        (false, true, false, true) => Ok('┌'),
        _ => Ok('+'),
    }
}

fn fix_horizontal(x: usize, y: usize, diagram: &ParsedDiagram) -> Result<char> {
    let left = x
        .checked_sub(1)
        .and_then(|new_x| get_neighbor_char(new_x, y, diagram));
    let right = get_neighbor_char(x + 1, y, diagram);
    let up = y
        .checked_sub(1)
        .and_then(|new_y| get_neighbor_char(x, new_y, diagram));
    let down = get_neighbor_char(x, y + 1, diagram);

    let has_left = left.is_some_and(is_horizontal);
    let has_right = right.is_some_and(is_horizontal);
    let has_up = up.is_some_and(is_vertical);
    let has_down = down.is_some_and(is_vertical);

    if has_left && has_right && has_up && !has_down {
        return Ok('┴');
    }
    if has_left && has_right && !has_up && has_down {
        return Ok('┬');
    }
    if has_left && !has_right && has_up && has_down {
        return Ok('┤');
    }
    if !has_left && has_right && has_up && has_down {
        return Ok('├');
    }

    Ok('─')
}

fn fix_vertical(x: usize, y: usize, diagram: &ParsedDiagram) -> Result<char> {
    let left = x
        .checked_sub(1)
        .and_then(|new_x| get_neighbor_char(new_x, y, diagram));
    let right = get_neighbor_char(x + 1, y, diagram);
    let up = y
        .checked_sub(1)
        .and_then(|new_y| get_neighbor_char(x, new_y, diagram));
    let down = get_neighbor_char(x, y + 1, diagram);

    let has_left = left.is_some_and(is_horizontal);
    let has_right = right.is_some_and(is_horizontal);
    let has_up = up.is_some_and(is_vertical);
    let has_down = down.is_some_and(is_vertical);

    if has_left && has_right && has_up && !has_down {
        return Ok('┴');
    }
    if has_left && has_right && !has_up && has_down {
        return Ok('┬');
    }
    if has_left && !has_right && has_up && has_down {
        return Ok('┤');
    }
    if !has_left && has_right && has_up && has_down {
        return Ok('├');
    }

    Ok('│')
}

fn fix_diagonal(ch: char) -> Result<char> {
    match ch {
        '/' => Ok('╱'),
        '\\' => Ok('╲'),
        _ => Ok(ch),
    }
}

fn fix_arrow(ch: char) -> Result<char> {
    match ch {
        'v' => Ok('▼'),
        '^' => Ok('▲'),
        '<' => Ok('◀'),
        '>' => Ok('▶'),
        _ => Ok(ch),
    }
}

fn is_arrow_like(ch: char) -> bool {
    matches!(ch, 'v' | '^' | '<' | '>')
}

fn get_neighbor_char(x: usize, y: usize, diagram: &ParsedDiagram) -> Option<char> {
    if y >= diagram.lines.len() {
        return None;
    }
    get_char_at(&diagram.lines[y], x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_box_drawing_symbols_simple() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "+-----+".to_string(),
                "|     |".to_string(),
                "+-----+".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fix_plus_to_cross() {
        let mut diagram = ParsedDiagram {
            lines: vec!["---+---".to_string(), "  |".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fix_horizontal_line() {
        let mut diagram = ParsedDiagram {
            lines: vec!["-----".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
        assert_eq!(diagram.lines[0].chars().next(), Some('─'));
    }

    #[test]
    fn test_fix_vertical_line() {
        let mut diagram = ParsedDiagram {
            lines: vec!["|".to_string(), "|".to_string(), "|".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
        assert_eq!(diagram.lines[0].chars().next(), Some('│'));
    }

    #[test]
    fn test_fix_diagonal_forward() {
        let mut diagram = ParsedDiagram {
            lines: vec!["   /".to_string(), "  / ".to_string(), " /  ".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fix_diagonal_backward() {
        let mut diagram = ParsedDiagram {
            lines: vec!["\\  ".to_string(), " \\ ".to_string(), "  \\".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fix_arrow_down() {
        let mut diagram = ParsedDiagram {
            lines: vec![" v ".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
        assert_eq!(diagram.lines[0].chars().nth(1), Some('▼'));
    }

    #[test]
    fn test_fix_arrow_up() {
        let mut diagram = ParsedDiagram {
            lines: vec![" ^ ".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
        assert_eq!(diagram.lines[0].chars().nth(1), Some('▲'));
    }

    #[test]
    fn test_fix_arrow_left() {
        let mut diagram = ParsedDiagram {
            lines: vec![" < ".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
        assert_eq!(diagram.lines[0].chars().nth(1), Some('◀'));
    }

    #[test]
    fn test_fix_arrow_right() {
        let mut diagram = ParsedDiagram {
            lines: vec![" > ".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
        assert_eq!(diagram.lines[0].chars().nth(1), Some('▶'));
    }

    #[test]
    fn test_fix_plus_cross() {
        let mut diagram = ParsedDiagram {
            lines: vec!["-+-".to_string(), " | ".to_string(), "-+-".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
        // Top junction: left/right + down.
        assert_eq!(diagram.lines[0].chars().nth(1), Some('┬'));
    }

    #[test]
    fn test_fix_plus_table_junctions_top_and_bottom() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "+----+----+----+".to_string(),
                "| A  | B  | C  |".to_string(),
                "+----+----+----+".to_string(),
                "| D  | E  | F  |".to_string(),
                "+----+----+----+".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        fix_box_drawing_symbols(&mut diagram).unwrap();

        assert_eq!(diagram.lines[0], "┌────┬────┬────┐");
        assert_eq!(diagram.lines[2], "├────┼────┼────┤");
        assert_eq!(diagram.lines[4], "└────┴────┴────┘");
    }

    #[test]
    fn test_fix_underscore_to_horizontal() {
        let mut diagram = ParsedDiagram {
            lines: vec!["_____".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
        assert_eq!(diagram.lines[0].chars().next(), Some('─'));
    }

    #[test]
    fn test_fix_exclamation_to_vertical() {
        let mut diagram = ParsedDiagram {
            lines: vec!["!".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
        assert_eq!(diagram.lines[0].chars().next(), Some('│'));
    }

    #[test]
    fn test_is_diagonal() {
        assert!(crate::utils::is_diagonal('/'));
        assert!(crate::utils::is_diagonal('\\'));
        assert!(crate::utils::is_diagonal('╱'));
        assert!(crate::utils::is_diagonal('╲'));
    }

    #[test]
    fn test_fix_empty_diagram() {
        let mut diagram = ParsedDiagram {
            lines: vec![],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
        assert!(diagram.lines.is_empty());
    }

    #[test]
    fn test_fix_box_with_corners() {
        let mut diagram = ParsedDiagram {
            lines: vec!["+-+".to_string(), "| |".to_string(), "+-+".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fix_box_drawing_preserves_length() {
        let original_lines = vec![
            "+-----+".to_string(),
            "|     |".to_string(),
            "+-----+".to_string(),
        ];

        let mut diagram = ParsedDiagram {
            lines: original_lines.clone(),
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        fix_box_drawing_symbols(&mut diagram).unwrap();

        assert_eq!(diagram.lines.len(), original_lines.len());
        for (i, line) in diagram.lines.iter().enumerate() {
            assert_eq!(
                crate::display_width::display_width(line.as_str()),
                crate::display_width::display_width(original_lines[i].as_str())
            );
        }
    }

    #[test]
    fn test_fix_diagonal_unicode() {
        let mut diagram = ParsedDiagram {
            lines: vec!["   ╱".to_string(), "  ╱ ".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fix_plus_with_all_directions() {
        let mut diagram = ParsedDiagram {
            lines: vec![" | ".to_string(), "-+-".to_string(), " | ".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
        assert_eq!(diagram.lines[1].chars().nth(1), Some('┼'));
    }

    #[test]
    fn test_fix_horizontal_with_vertical_neighbors() {
        let mut diagram = ParsedDiagram {
            lines: vec![" | ".to_string(), "---".to_string(), " | ".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fix_vertical_with_horizontal_neighbors() {
        let mut diagram = ParsedDiagram {
            lines: vec!["---".to_string(), " | ".to_string(), "---".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fix_box_drawing_mixed_characters() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "+---+".to_string(),
                "| A |".to_string(),
                "+---+".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
        assert!(diagram.lines[1].contains('A'));
    }

    #[test]
    fn test_fix_box_drawing_preserves_text() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "+--------+".to_string(),
                "| Hello  |".to_string(),
                "+--------+".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        fix_box_drawing_symbols(&mut diagram).unwrap();

        assert!(diagram.lines[1].contains("Hello"));
    }

    #[test]
    fn test_fix_box_drawing_complex_box() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "+---+---+".to_string(),
                "| A | B |".to_string(),
                "+---+---+".to_string(),
                "| C | D |".to_string(),
                "+---+---+".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fix_plus_corner_top_left() {
        let mut diagram = ParsedDiagram {
            lines: vec![" + ".to_string(), " | ".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fix_plus_corner_top_right() {
        let mut diagram = ParsedDiagram {
            lines: vec![" + ".to_string(), "  |".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fix_plus_corner_bottom_left() {
        let mut diagram = ParsedDiagram {
            lines: vec![" | ".to_string(), " + ".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fix_plus_corner_bottom_right() {
        let mut diagram = ParsedDiagram {
            lines: vec!["  |".to_string(), " + ".to_string()],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }
}
