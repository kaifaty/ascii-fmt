use std::collections::HashMap;

pub fn is_box_drawing_char(ch: char) -> bool {
    matches!(ch, '─'..='┿' | '═'..='╿')
}

pub fn is_horizontal(ch: char) -> bool {
    matches!(ch, '─' | '═' | '┄' | '┅')
}

pub fn is_vertical(ch: char) -> bool {
    matches!(ch, '│' | '║' | '┆' | '┇')
}

pub fn is_arrow(ch: char) -> bool {
    matches!(ch, '↑' | '↓' | '←' | '→' | '▼' | '▶' | '◀')
}

pub fn is_diagonal(ch: char) -> bool {
    matches!(ch, '/' | '\\' | '╱' | '╲')
}

pub fn most_common<T: std::hash::Hash + Eq + Clone>(items: &[T]) -> Option<T> {
    let mut counts: HashMap<&T, usize> = HashMap::new();

    for item in items {
        *counts.entry(item).or_insert(0) += 1;
    }

    counts.into_iter().max_by_key(|&(_, count)| count).map(|(k, _)| k.clone())
}

pub fn align_to_grid(value: usize, grid_size: usize) -> usize {
    if grid_size == 0 {
        return value;
    }
    ((value + grid_size - 1) / grid_size) * grid_size
}

pub fn get_char_at(s: &str, index: usize) -> Option<char> {
    s.chars().nth(index)
}

pub fn get_neighbors(s: &str, x: usize) -> Neighbors {
    let chars: Vec<char> = s.chars().collect();
    let left = if x > 0 { chars.get(x - 1).copied() } else { None };
    let right = chars.get(x + 1).copied();
    Neighbors { left, right }
}

#[derive(Debug, Clone, Copy)]
pub struct Neighbors {
    pub left: Option<char>,
    pub right: Option<char>,
}
