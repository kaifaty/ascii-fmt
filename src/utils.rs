use std::collections::HashMap;

pub fn is_box_drawing_char(ch: char) -> bool {
    matches!(ch, '─'..='┿' | '═'..='╿')
}

pub fn is_horizontal(ch: char) -> bool {
    matches!(ch, '─' | '═' | '┄' | '┅' | '-' | '_')
}

pub fn is_vertical(ch: char) -> bool {
    matches!(ch, '│' | '║' | '┆' | '┇' | '|' | '!')
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_box_drawing_char_basic() {
        assert!(is_box_drawing_char('─'));
        assert!(is_box_drawing_char('│'));
        assert!(is_box_drawing_char('┌'));
        assert!(is_box_drawing_char('┐'));
        assert!(is_box_drawing_char('└'));
        assert!(is_box_drawing_char('┘'));
        assert!(is_box_drawing_char('├'));
        assert!(is_box_drawing_char('┤'));
        assert!(is_box_drawing_char('┬'));
        assert!(is_box_drawing_char('┴'));
        assert!(is_box_drawing_char('┼'));
    }

    #[test]
    fn test_is_box_drawing_char_double() {
        assert!(is_box_drawing_char('═'));
        assert!(is_box_drawing_char('║'));
        assert!(is_box_drawing_char('╔'));
        assert!(is_box_drawing_char('╗'));
        assert!(is_box_drawing_char('╚'));
        assert!(is_box_drawing_char('╝'));
    }

    #[test]
    fn test_is_box_drawing_char_false() {
        assert!(!is_box_drawing_char('a'));
        assert!(!is_box_drawing_char('1'));
        assert!(!is_box_drawing_char(' '));
        assert!(!is_box_drawing_char('-'));
        assert!(!is_box_drawing_char('|'));
        assert!(!is_box_drawing_char('+'));
    }

    #[test]
    fn test_is_horizontal_basic() {
        assert!(is_horizontal('─'));
        assert!(is_horizontal('═'));
        assert!(is_horizontal('┄'));
        assert!(is_horizontal('┅'));
    }

    #[test]
    fn test_is_horizontal_false() {
        assert!(!is_horizontal('│'));
        assert!(!is_horizontal('║'));
        assert!(!is_horizontal('┌'));
        assert!(!is_horizontal('a'));
    }

    #[test]
    fn test_is_vertical_basic() {
        assert!(is_vertical('│'));
        assert!(is_vertical('║'));
        assert!(is_vertical('┆'));
        assert!(is_vertical('┇'));
    }

    #[test]
    fn test_is_vertical_false() {
        assert!(!is_vertical('─'));
        assert!(!is_vertical('═'));
        assert!(!is_vertical('┌'));
        assert!(!is_vertical('a'));
    }

    #[test]
    fn test_is_arrow_basic() {
        assert!(is_arrow('↑'));
        assert!(is_arrow('↓'));
        assert!(is_arrow('←'));
        assert!(is_arrow('→'));
        assert!(is_arrow('▼'));
        assert!(is_arrow('▶'));
        assert!(is_arrow('◀'));
    }

    #[test]
    fn test_is_arrow_false() {
        assert!(!is_arrow('^'));
        assert!(!is_arrow('v'));
        assert!(!is_arrow('<'));
        assert!(!is_arrow('>'));
        assert!(!is_arrow('-'));
        assert!(!is_arrow('|'));
    }

    #[test]
    fn test_is_diagonal_basic() {
        assert!(is_diagonal('/'));
        assert!(is_diagonal('\\'));
        assert!(is_diagonal('╱'));
        assert!(is_diagonal('╲'));
    }

    #[test]
    fn test_is_diagonal_false() {
        assert!(!is_diagonal('|'));
        assert!(!is_diagonal('-'));
        assert!(!is_diagonal('a'));
        assert!(!is_diagonal('1'));
    }

    #[test]
    fn test_most_common_single() {
        let items = vec![1, 1, 1, 1, 1];
        assert_eq!(most_common(&items), Some(1));
    }

    #[test]
    fn test_most_common_multiple() {
        let items = vec![1, 2, 2, 3, 3, 3, 2, 2];
        assert_eq!(most_common(&items), Some(2));
    }

    #[test]
    fn test_most_common_tie() {
        let items = vec![1, 1, 2, 2, 3];
        let result = most_common(&items);
        assert!(result == Some(1) || result == Some(2));
    }

    #[test]
    fn test_most_common_empty() {
        let items: Vec<i32> = vec![];
        assert_eq!(most_common(&items), None);
    }

    #[test]
    fn test_most_common_strings() {
        let items = vec!["a", "b", "b", "c", "c", "c", "b", "b"];
        assert_eq!(most_common(&items), Some("b"));
    }

    #[test]
    fn test_most_common_chars() {
        let items: Vec<char> = vec!['a', 'b', 'b', 'c', 'c', 'c', 'b'];
        let result = most_common(&items);
        // 'b' and 'c' both appear 3 times - either is valid
        assert!(result == Some('b') || result == Some('c'));
    }

    #[test]
    fn test_align_to_grid_zero() {
        assert_eq!(align_to_grid(5, 0), 5);
        assert_eq!(align_to_grid(0, 0), 0);
    }

    #[test]
    fn test_align_to_grid_basic() {
        assert_eq!(align_to_grid(5, 2), 6);
        assert_eq!(align_to_grid(6, 2), 6);
        assert_eq!(align_to_grid(7, 2), 8);
    }

    #[test]
    fn test_align_to_grid_larger() {
        assert_eq!(align_to_grid(10, 5), 10);
        assert_eq!(align_to_grid(11, 5), 15);
        assert_eq!(align_to_grid(12, 5), 15);
        assert_eq!(align_to_grid(15, 5), 15);
        assert_eq!(align_to_grid(16, 5), 20);
    }

    #[test]
    fn test_align_to_grid_one() {
        assert_eq!(align_to_grid(5, 1), 5);
        assert_eq!(align_to_grid(0, 1), 0);
        assert_eq!(align_to_grid(10, 1), 10);
    }

    #[test]
    fn test_align_to_grid_already_aligned() {
        assert_eq!(align_to_grid(8, 4), 8);
        assert_eq!(align_to_grid(12, 4), 12);
        assert_eq!(align_to_grid(16, 4), 16);
    }

    #[test]
    fn test_get_char_at_valid() {
        assert_eq!(get_char_at("hello", 0), Some('h'));
        assert_eq!(get_char_at("hello", 1), Some('e'));
        assert_eq!(get_char_at("hello", 4), Some('o'));
    }

    #[test]
    fn test_get_char_at_invalid() {
        assert_eq!(get_char_at("hello", 5), None);
        assert_eq!(get_char_at("hello", 10), None);
        assert_eq!(get_char_at("hello", 100), None);
    }

    #[test]
    fn test_get_char_at_empty() {
        assert_eq!(get_char_at("", 0), None);
        assert_eq!(get_char_at("", 1), None);
    }

    #[test]
    fn test_get_char_at_unicode() {
        assert_eq!(get_char_at("héllo", 1), Some('é'));
        assert_eq!(get_char_at("日本語", 0), Some('日'));
        assert_eq!(get_char_at("日本語", 1), Some('本'));
        assert_eq!(get_char_at("日本語", 2), Some('語'));
    }

    #[test]
    fn test_get_neighbors_middle() {
        let s = "hello";
        let neighbors = get_neighbors(s, 2);
        assert_eq!(neighbors.left, Some('e'));
        assert_eq!(neighbors.right, Some('l'));
    }

    #[test]
    fn test_get_neighbors_start() {
        let s = "hello";
        let neighbors = get_neighbors(s, 0);
        assert_eq!(neighbors.left, None);
        assert_eq!(neighbors.right, Some('e'));
    }

    #[test]
    fn test_get_neighbors_end() {
        let s = "hello";
        let neighbors = get_neighbors(s, 4);
        assert_eq!(neighbors.left, Some('l'));
        assert_eq!(neighbors.right, None);
    }

    #[test]
    fn test_get_neighbors_out_of_bounds() {
        let s = "hello";
        let neighbors = get_neighbors(s, 10);
        assert_eq!(neighbors.left, None);
        assert_eq!(neighbors.right, None);
    }

    #[test]
    fn test_get_neighbors_single_char() {
        let s = "a";
        let neighbors = get_neighbors(s, 0);
        assert_eq!(neighbors.left, None);
        assert_eq!(neighbors.right, None);
    }

    #[test]
    fn test_get_neighbors_empty() {
        let s = "";
        let neighbors = get_neighbors(s, 0);
        assert_eq!(neighbors.left, None);
        assert_eq!(neighbors.right, None);
    }

    #[test]
    fn test_get_neighbors_unicode() {
        let s = "héllo";
        let neighbors = get_neighbors(s, 1);
        assert_eq!(neighbors.left, Some('h'));
        assert_eq!(neighbors.right, Some('l'));
    }

    #[test]
    fn test_neighbors_copy_and_clone() {
        let neighbors = Neighbors {
            left: Some('a'),
            right: Some('b'),
        };

        let neighbors_copy = neighbors;
        assert_eq!(neighbors_copy.left, Some('a'));
        assert_eq!(neighbors_copy.right, Some('b'));

        let neighbors_clone = neighbors.clone();
        assert_eq!(neighbors_clone.left, Some('a'));
        assert_eq!(neighbors_clone.right, Some('b'));
    }

    #[test]
    fn test_neighbors_debug() {
        let neighbors = Neighbors {
            left: Some('a'),
            right: Some('b'),
        };
        format!("{:?}", neighbors);
    }

    #[test]
    fn test_is_box_drawing_char_all_ranges() {
        assert!(is_box_drawing_char('─'));
        assert!(is_box_drawing_char('┿'));
        assert!(is_box_drawing_char('│'));
        assert!(is_box_drawing_char('┼'));
        assert!(is_box_drawing_char('┌'));
        assert!(is_box_drawing_char('┐'));
        assert!(is_box_drawing_char('═'));
        assert!(is_box_drawing_char('╿'));
        assert!(is_box_drawing_char('║'));
        assert!(is_box_drawing_char('╔'));
        assert!(is_box_drawing_char('╗'));
    }

    #[test]
    fn test_align_to_grid_edge_cases() {
        assert_eq!(align_to_grid(1, 2), 2);
        assert_eq!(align_to_grid(0, 2), 0);
        assert_eq!(align_to_grid(1, 100), 100);
        assert_eq!(align_to_grid(99, 100), 100);
        assert_eq!(align_to_grid(100, 100), 100);
        assert_eq!(align_to_grid(101, 100), 200);
    }
}
