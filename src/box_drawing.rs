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

    for y in 0..diagram.lines.len() {
        let line = &original_lines[y];
        let mut new_line = String::with_capacity(line.len());

        for (x, ch) in line.chars().enumerate() {
            let fixed = fix_char(ch, x, y, &temp_diagram)?;
            new_line.push(fixed);
        }

        new_lines.push(new_line);
    }

    diagram.lines = new_lines;
    Ok(())
}

fn fix_char(ch: char, x: usize, y: usize, diagram: &ParsedDiagram) -> Result<char> {
    match ch {
        '+' => fix_plus(x, y, diagram),
        '-' | '_' => fix_horizontal(x, y, diagram),
        '|' | '!' => fix_vertical(x, y, diagram),
        c if is_diagonal(c) => fix_diagonal(c),
        c if is_arrow_like(c) => fix_arrow(c),
        _ => Ok(ch),
    }
}

fn fix_plus(x: usize, y: usize, diagram: &ParsedDiagram) -> Result<char> {
    let left = x.checked_sub(1).and_then(|new_x| get_neighbor_char(new_x, y, diagram));
    let right = get_neighbor_char(x + 1, y, diagram);
    let up = y.checked_sub(1).and_then(|new_y| get_neighbor_char(x, new_y, diagram));
    let down = get_neighbor_char(x, y + 1, diagram);

    let has_left = left.map_or(false, |c| is_horizontal(c));
    let has_right = right.map_or(false, |c| is_horizontal(c));
    let has_up = up.map_or(false, |c| is_vertical(c));
    let has_down = down.map_or(false, |c| is_vertical(c));

    match (has_left, has_right, has_up, has_down) {
        (true, true, true, true) => Ok('┼'),
        (true, true, false, false) => Ok('─'),
        (false, false, true, true) => Ok('│'),
        (true, true, true, false) => Ok('┬'),
        (true, true, false, true) => Ok('┴'),
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
    let left = x.checked_sub(1).and_then(|new_x| get_neighbor_char(new_x, y, diagram));
    let right = get_neighbor_char(x + 1, y, diagram);
    let up = y.checked_sub(1).and_then(|new_y| get_neighbor_char(x, new_y, diagram));
    let down = get_neighbor_char(x, y + 1, diagram);

    let has_left = left.map_or(false, |c| is_horizontal(c));
    let has_right = right.map_or(false, |c| is_horizontal(c));
    let has_up = up.map_or(false, |c| is_vertical(c));
    let has_down = down.map_or(false, |c| is_vertical(c));

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
    let left = x.checked_sub(1).and_then(|new_x| get_neighbor_char(new_x, y, diagram));
    let right = get_neighbor_char(x + 1, y, diagram);
    let up = y.checked_sub(1).and_then(|new_y| get_neighbor_char(x, new_y, diagram));
    let down = get_neighbor_char(x, y + 1, diagram);

    let has_left = left.map_or(false, |c| is_horizontal(c));
    let has_right = right.map_or(false, |c| is_horizontal(c));
    let has_up = up.map_or(false, |c| is_vertical(c));
    let has_down = down.map_or(false, |c| is_vertical(c));

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
            lines: vec![
                "---+---".to_string(),
                "  |".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fix_horizontal_line() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "-----".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
        assert_eq!(diagram.lines[0].chars().next(), Some('─'));
    }

    #[test]
    fn test_fix_vertical_line() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "|".to_string(),
                "|".to_string(),
                "|".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
        assert_eq!(diagram.lines[0].chars().next(), Some('│'));
    }

    #[test]
    fn test_fix_diagonal_forward() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "   /".to_string(),
                "  / ".to_string(),
                " /  ".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fix_diagonal_backward() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "\\  ".to_string(),
                " \\ ".to_string(),
                "  \\".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fix_arrow_down() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                " v ".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
        assert_eq!(diagram.lines[0].chars().nth(1), Some('▼'));
    }

    #[test]
    fn test_fix_arrow_up() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                " ^ ".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
        assert_eq!(diagram.lines[0].chars().nth(1), Some('▲'));
    }

    #[test]
    fn test_fix_arrow_left() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                " < ".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
        assert_eq!(diagram.lines[0].chars().nth(1), Some('◀'));
    }

    #[test]
    fn test_fix_arrow_right() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                " > ".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
        assert_eq!(diagram.lines[0].chars().nth(1), Some('▶'));
    }

    #[test]
    fn test_fix_plus_cross() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "-+-".to_string(),
                " | ".to_string(),
                "-+-".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
        assert_eq!(diagram.lines[0].chars().nth(1), Some('┴'));
    }

    #[test]
    fn test_fix_underscore_to_horizontal() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "_____".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
        assert_eq!(diagram.lines[0].chars().next(), Some('─'));
    }

    #[test]
    fn test_fix_exclamation_to_vertical() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "!".to_string(),
            ],
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
            lines: vec![
                "+-+".to_string(),
                "| |".to_string(),
                "+-+".to_string(),
            ],
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
            assert_eq!(unicode_width::UnicodeWidthStr::width(line.as_str()),
                      unicode_width::UnicodeWidthStr::width(original_lines[i].as_str()));
        }
    }

    #[test]
    fn test_fix_diagonal_unicode() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "   ╱".to_string(),
                "  ╱ ".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fix_plus_with_all_directions() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                " | ".to_string(),
                "-+-".to_string(),
                " | ".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
        assert_eq!(diagram.lines[1].chars().nth(1), Some('┼'));
    }

    #[test]
    fn test_fix_horizontal_with_vertical_neighbors() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                " | ".to_string(),
                "---".to_string(),
                " | ".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fix_vertical_with_horizontal_neighbors() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "---".to_string(),
                " | ".to_string(),
                "---".to_string(),
            ],
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
            lines: vec![
                " + ".to_string(),
                " | ".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fix_plus_corner_top_right() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                " + ".to_string(),
                "  |".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fix_plus_corner_bottom_left() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                " | ".to_string(),
                " + ".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fix_plus_corner_bottom_right() {
        let mut diagram = ParsedDiagram {
            lines: vec![
                "  |".to_string(),
                " + ".to_string(),
            ],
            diagram_type: crate::patterns::DiagramType::Unknown,
        };

        let result = fix_box_drawing_symbols(&mut diagram);
        assert!(result.is_ok());
    }
}
