use crate::error::Result;
use crate::parser::ParsedDiagram;
use crate::utils::{get_char_at, is_diagonal, is_horizontal, is_vertical};

pub fn fix_box_drawing_symbols(diagram: &mut ParsedDiagram) -> Result<()> {
    for y in 0..diagram.lines.len() {
        let line = diagram.lines[y].clone();
        let mut new_line = String::with_capacity(line.len());

        for (x, ch) in line.chars().enumerate() {
            let fixed = fix_char(ch, x, y, diagram)?;
            new_line.push(fixed);
        }

        diagram.lines[y] = new_line;
    }
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
    let left = get_neighbor_char(x - 1, y, diagram);
    let right = get_neighbor_char(x + 1, y, diagram);
    let up = get_neighbor_char(x, y - 1, diagram);
    let down = get_neighbor_char(x, y + 1, diagram);

    let has_left = left.map_or(false, |c| is_horizontal(c));
    let has_right = right.map_or(false, |c| is_horizontal(c));
    let has_up = up.map_or(false, |c| is_vertical(c));
    let has_down = down.map_or(false, |c| is_vertical(c));

    match (has_left, has_right, has_up, has_down) {
        (true, true, true, true) => Ok('┼'),
        (true, true, false, false) => Ok('─'),
        (false, false, true, true) => Ok('│'),
        (true, true, true, false) => Ok('┴'),
        (true, true, false, true) => Ok('┬'),
        (true, false, true, true) => Ok('┤'),
        (false, true, true, true) => Ok('├'),
        (true, false, true, false) => Ok('┘'),
        (false, true, true, false) => Ok('└'),
        (true, false, false, true) => Ok('┐'),
        (false, true, false, true) => Ok('┌'),
        _ => Ok('+'),
    }
}

fn fix_horizontal(x: usize, y: usize, diagram: &ParsedDiagram) -> Result<char> {
    let left = get_neighbor_char(x - 1, y, diagram);
    let right = get_neighbor_char(x + 1, y, diagram);
    let up = get_neighbor_char(x, y - 1, diagram);
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
    let left = get_neighbor_char(x - 1, y, diagram);
    let right = get_neighbor_char(x + 1, y, diagram);
    let up = get_neighbor_char(x, y - 1, diagram);
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
