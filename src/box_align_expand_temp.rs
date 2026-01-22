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
            for _ in 0..diff {
                new_chars.push(' ');
            }
        }
        new_chars.push(ch);
    }

    lines[y] = new_chars.into_iter().collect();
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
