use unicode_display_width::width;

/// Returns the display width of the string in terminal cells.
///
/// This uses `unicode-display-width` (grapheme-aware) and is intended for
/// monospace rendering in terminals.
pub fn display_width(s: &str) -> usize {
    match usize::try_from(width(s)) {
        Ok(v) => v,
        Err(_) => usize::MAX,
    }
}

/// Returns the display width of a single character.
pub fn display_width_char(ch: char) -> usize {
    let mut buf = [0u8; 4];
    let s = ch.encode_utf8(&mut buf);
    display_width(s)
}

/// Pads `s` on the right with spaces until it reaches `width` cells.
pub fn ljust_to_width(s: &str, width: usize) -> String {
    let w = display_width(s);
    if w >= width {
        return s.to_string();
    }

    let mut out = String::with_capacity(s.len() + (width - w));
    out.push_str(s);
    out.push_str(&" ".repeat(width - w));
    out
}

/// Pads `s` on the left with spaces until it reaches `width` cells.
pub fn rjust_to_width(s: &str, width: usize) -> String {
    let w = display_width(s);
    if w >= width {
        return s.to_string();
    }

    let mut out = String::with_capacity(s.len() + (width - w));
    out.push_str(&" ".repeat(width - w));
    out.push_str(s);
    out
}

/// Centers `s` in a field of `width` cells.
///
/// If padding is odd, the extra cell is placed on the right.
pub fn center_to_width(s: &str, width: usize) -> String {
    let w = display_width(s);
    if w >= width {
        return s.to_string();
    }

    let pad = width - w;
    let left = pad / 2;
    let right = pad - left;

    let mut out = String::with_capacity(s.len() + pad);
    out.push_str(&" ".repeat(left));
    out.push_str(s);
    out.push_str(&" ".repeat(right));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_width_ascii() {
        assert_eq!(display_width(""), 0);
        assert_eq!(display_width("abc"), 3);
    }

    #[test]
    fn test_ljust_to_width() {
        assert_eq!(ljust_to_width("a", 1), "a");
        assert_eq!(ljust_to_width("a", 3), "a  ");
        assert_eq!(ljust_to_width("abc", 2), "abc");
    }

    #[test]
    fn test_rjust_to_width() {
        assert_eq!(rjust_to_width("a", 1), "a");
        assert_eq!(rjust_to_width("a", 3), "  a");
        assert_eq!(rjust_to_width("abc", 2), "abc");
    }

    #[test]
    fn test_center_to_width() {
        assert_eq!(center_to_width("a", 1), "a");
        assert_eq!(center_to_width("a", 3), " a ");
        // odd padding -> extra on the right
        assert_eq!(center_to_width("a", 4), " a  ");
        assert_eq!(center_to_width("abc", 2), "abc");
    }
}
