# Architecture Documentation

## Обзор

`ascii-fmt` - CLI инструмент для форматирования ASCII диаграмм. Архитектура разработана с упором на производительность, модульность и тестируемость.

## Дизайн-принципы

1. **Two-Pass Algorithm** - Сначала анализ, затем рендеринг (не инкрементно)
2. **Grid-Based Coordinates** - Все вычисления на сетке, не на символах
3. **Zero-Copy Where Possible** - Минимизация аллокаций
4. **Separation of Concerns** - Четкое разделение парсинга, анализа, форматирования
5. **Testability** - Все модули изолированы и тестируемы независимо

## Модульная архитектура

```
┌─────────────────────────────────────────────────────────────┐
│                         main.rs                              │
│  (CLI entry point, stdin/stdout handling, error reporting)   │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                          cli.rs                              │
│  (Argument parsing, validation, options construction)       │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                      formatter.rs                            │
│  (Orchestrator: parse → analyze → fix → render)             │
└──────┬────────────┬────────────┬────────────┬────────────────┘
       │            │            │            │
       ▼            ▼            ▼            ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│   parser.rs  │ │   grid.rs    │ │box_drawing. rs│ │ text_align.rs│
│  (Parse AST) │ │ (Grid logic) │ │  (Fix chars) │ │ (Align text) │
└──────────────┘ └──────────────┘ └──────────────┘ └──────────────┘
                     │
                     ▼
              ┌──────────────┐
              │  patterns.rs │
              │(Detect type) │
              └──────────────┘
```

## Детальное описание модулей

### 1. `main.rs` - Entry Point

**Ответственность**:
- Обработка CLI аргументов
- Чтение из stdin/file
- Запись в stdout/file
- Глобальная обработка ошибок

**Ключевые функции**:
```rust
fn main() -> Result<(), Error>
fn read_input(input: Option<PathBuf>) -> Result<String>
fn write_output(output: Option<PathBuf>, content: &str) -> Result<()>
```

### 2. `cli.rs` - Argument Parsing

**Ответственность**:
- Парсинг CLI аргументов через `clap`
- Валидация опций
- Конструкция `Options` struct

**Ключевые типы**:
```rust
#[derive(Parser, Debug)]
#[command(name = "ascii-fmt")]
struct Cli {
    #[arg(value_name = "INPUT")]
    input: Option<PathBuf>,

    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    #[arg(short, long, default_value_t = 2)]
    width: usize,

    #[arg(short, long, default_value = "standard")]
    style: Style,

    #[arg(long, default_value_t = true)]
    fix_box_drawing: bool,

    #[arg(long, default_value_t = true)]
    fix_whitespace: bool,

    #[arg(long, default_value_t = true)]
    preserve_empty_lines: bool,

    #[arg(long)]
    dry_run: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum Style {
    Minimal,
    Standard,
    Detailed,
}
```

### 3. `formatter.rs` - Main Formatting Logic

**Ответственность**:
- Орхестрация всего процесса форматирования
- Применение опций в зависимости от стиля
- Компоновка результатов из разных модулей

**Основной алгоритм**:
```rust
pub fn format_ascii(input: &str, options: &Options) -> Result<String> {
    // Phase 1: Parse
    let diagram = parser::parse(input)?;

    // Phase 2: Analyze grid
    let grid_metrics = grid::analyze_grid(&diagram.lines)?;

    // Phase 3: Apply fixes
    let mut formatted = diagram.clone();
    if options.fix_box_drawing {
        box_drawing::fix_box_drawing_symbols(&mut formatted)?;
    }
    if options.fix_whitespace {
        grid::normalize_whitespace(&mut formatted, &grid_metrics)?;
    }
    text_align::align_text_content(&mut formatted, &grid_metrics, options)?;

    // Phase 4: Render
    Ok(formatter::render(&formatted, &grid_metrics))
}
```

### 4. `parser.rs` - Diagram Parsing

**Ответственность**:
- Парсинг строк в AST (Abstract Syntax Tree)
- Определение типа диаграммы
- Выделение структурных элементов (boxes, connections)

**Ключевые типы**:
```rust
#[derive(Debug, Clone)]
struct ParsedDiagram {
    lines: Vec<String>,
    diagram_type: DiagramType,
    boxes: Vec<Rect>,
    connections: Vec<Connection>,
}

#[derive(Debug, Clone, PartialEq)]
enum DiagramType {
    Flowchart,
    Tree,
    Architecture,
    Sequence,
    Table,
    Unknown,
}

#[derive(Debug, Clone)]
struct Rect {
    x: usize,      // Column position
    y: usize,      // Row position
    width: usize,
    height: usize,
}

#[derive(Debug, Clone)]
struct Connection {
    from: Point,
    to: Point,
    direction: Direction,
}
```

**Алгоритм парсинга**:
```rust
pub fn parse(input: &str) -> Result<ParsedDiagram> {
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    // Detect diagram type by pattern matching
    let diagram_type = patterns::detect_type(&lines);

    // Find all boxes by looking for corner patterns
    let boxes = find_boxes(&lines)?;

    // Find connections (arrows, lines)
    let connections = find_connections(&lines, &boxes)?;

    Ok(ParsedDiagram {
        lines,
        diagram_type,
        boxes,
        connections,
    })
}
```

### 5. `grid.rs` - Grid Alignment

**Ответственность**:
- Анализ сетки диаграммы
- Вычисление характерной ширины колонок
- Выравнивание по сетке
- Нормализация пробелов

**Ключевые типы**:
```rust
#[derive(Debug, Clone)]
struct GridMetrics {
    column_width: usize,      // Character width of grid cells
    row_height: usize,        // Height of rows (usually 1)
    columns: Vec<usize>,      // X positions of grid lines
    rows: Vec<usize>,         // Y positions of grid lines
}
```

**Алгоритм анализа**:
```rust
pub fn analyze_grid(lines: &[String]) -> Result<GridMetrics> {
    // Find all vertical lines (│, ┤, ├)
    let vertical_positions = find_vertical_lines(lines);

    // Calculate gaps between vertical lines
    let gaps: Vec<usize> = vertical_positions
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect();

    // Find most common gap = column width
    let column_width = most_common(&gaps).unwrap_or(2);

    // Find all horizontal lines (─, ┴, ┬)
    let horizontal_positions = find_horizontal_lines(lines);

    Ok(GridMetrics {
        column_width,
        row_height: 1,
        columns: vertical_positions,
        rows: horizontal_positions,
    })
}
```

**Нормализация пробелов**:
```rust
pub fn normalize_whitespace(
    diagram: &mut ParsedDiagram,
    metrics: &GridMetrics,
) -> Result<()> {
    for line in &mut diagram.lines {
        // Trim trailing whitespace
        let trimmed = line.trim_end().to_string();

        // Pad to align with grid
        let current_len = unicode_width::UnicodeWidthStr::width(trimmed.as_str());
        let target_len = align_to_grid(current_len, metrics.column_width);

        if current_len < target_len {
            line.push_str(&" ".repeat(target_len - current_len));
        }
    }
    Ok(())
}
```

### 6. `box_drawing.rs` - Box-Drawing Symbol Fixes

**Ответственность**:
- Распознавание некорректных box-drawing символов
- Замена на правильные Unicode символы по контексту
- Обработка corner cases (double-line vs single-line)

**Ключевые функции**:
```rust
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

fn fix_char(
    ch: char,
    x: usize,
    y: usize,
    diagram: &ParsedDiagram,
) -> Result<char> {
    // Get neighboring characters
    let neighbors = get_neighbors(x, y, diagram);

    // Determine correct character based on context
    match ch {
        '+' => fix_plus(neighbors),
        '-' | '_' => fix_horizontal(neighbors),
        '|' | '!' => fix_vertical(neighbors),
        '/' => fix_slash(neighbors),
        '\\' => fix_backslash(neighbors),
        c if is_arrow(c) => fix_arrow(c, neighbors),
        _ => Ok(ch),
    }
}

fn fix_plus(neighbors: Neighbors) -> Result<char> {
    // Check all 8 directions
    let has_left = neighbors.left.map_or(false, |c| is_horizontal(c));
    let has_right = neighbors.right.map_or(false, |c| is_horizontal(c));
    let has_up = neighbors.up.map_or(false, |c| is_vertical(c));
    let has_down = neighbors.down.map_or(false, |c| is_vertical(c));

    match (has_left, has_right, has_up, has_down) {
        (true, true, true, true) => Ok('┼'),  // Cross
        (true, true, false, false) => Ok('─'), // Horizontal
        (false, false, true, true) => Ok('│'), // Vertical
        (true, true, true, false) => Ok('┴'), // T up
        (true, true, false, true) => Ok('┬'), // T down
        (true, false, true, true) => Ok('┤'), // T left
        (false, true, true, true) => Ok('├'), // T right
        (true, false, true, false) => Ok('┘'), // Top-right corner
        (false, true, true, false) => Ok('└'), // Top-left corner
        (true, false, false, true) => Ok('┐'), // Bottom-right corner
        (false, true, false, true) => Ok('┌'), // Bottom-left corner
        _ => Ok('+'), // Keep as-is if ambiguous
    }
}
```

**Таблица замен**:

| Wrong | Context | Correct |
|-------|---------|---------|
| `+` | All 4 directions | `┼` |
| `+` | Horizontal only | `─` |
| `+` | Vertical only | `│` |
| `+` | T-junction | `├ ┤ ┬ ┴` |
| `+` | Corner | `┌ ┐ └ ┘` |
| `-` | Horizontal | `─` |
| `|` | Vertical | `│` |
| `/` | Diagonal | `╱` (U+2571) |
| `\` | Diagonal | `╲` (U+2572) |

### 7. `patterns.rs` - Pattern Matching

**Ответственность**:
- Определение типа диаграммы
- Выявление характерных паттернов
- Детектирование коробок и соединений

**Детектирование типа**:
```rust
pub fn detect_type(lines: &[String]) -> DiagramType {
    let has_arrows = lines.iter().any(|l| l.contains("->") || l.contains("->"));
    let has_tree_indent = lines.iter().any(|l| l.starts_with("  ") && l.contains("├"));
    let has_layers = lines.iter().any(|l| l.contains("├") && l.contains("┤"));
    let has_vertical_lines = lines.iter().any(|l| l.contains("│"));
    let has_borders = lines.iter().any(|l| l.contains("┌") || l.contains("│"));

    match (has_arrows, has_tree_indent, has_layers, has_borders) {
        (_, true, _, _) => DiagramType::Tree,
        (true, false, false, true) => DiagramType::Flowchart,
        (false, false, true, true) => DiagramType::Architecture,
        (false, false, false, true) if has_vertical_lines => DiagramType::Sequence,
        (false, false, false, true) => DiagramType::Table,
        _ => DiagramType::Unknown,
    }
}
```

### 8. `text_align.rs` - Text Alignment

**Ответственность**:
- Центрирование заголовков в коробках
- Выравнивание содержимого блоков
- Сохранение отступов для вложенности

**Алгоритм**:
```rust
pub fn align_text_content(
    diagram: &mut ParsedDiagram,
    metrics: &GridMetrics,
    options: &Options,
) -> Result<()> {
    for box_rect in &diagram.boxes {
        align_text_in_box(diagram, box_rect, metrics, options)?;
    }
    Ok(())
}

fn align_text_in_box(
    diagram: &mut ParsedDiagram,
    rect: &Rect,
    metrics: &GridMetrics,
    options: &Options,
) -> Result<()> {
    let box_width = rect.width - 2; // Exclude borders

    for y in (rect.y + 1)..(rect.y + rect.height - 1) {
        if y >= diagram.lines.len() {
            break;
        }

        let line = &mut diagram.lines[y];
        if x >= line.len() {
            break;
        }

        // Extract content inside box (between │ chars)
        let content_start = rect.x + 1;
        let content_end = rect.x + rect.width - 1;

        if content_end >= line.len() {
            continue;
        }

        let content: String = line
            .chars()
            .skip(content_start)
            .take(box_width)
            .collect();

        // Align based on style
        let aligned = match options.style {
            Style::Minimal => align_left(&content, box_width),
            Style::Standard => align_left(&content, box_width),
            Style::Detailed => align_center(&content, box_width),
        };

        // Replace content
        let new_line: String = line
            .chars()
            .take(content_start)
            .chain(aligned.chars())
            .chain(line.chars().skip(content_end))
            .collect();

        diagram.lines[y] = new_line;
    }

    Ok(())
}

fn align_center(text: &str, width: usize) -> String {
    let text_len = unicode_width::UnicodeWidthStr::width(text);
    if text_len >= width {
        return text[..width.min(text.len())].to_string();
    }

    let padding = width - text_len;
    let left_pad = padding / 2;
    let right_pad = padding - left_pad;

    format!("{}{}{}", " ".repeat(left_pad), text, " ".repeat(right_pad))
}

fn align_left(text: &str, width: usize) -> String {
    let text_len = unicode_width::UnicodeWidthStr::width(text);
    if text_len >= width {
        return text[..width.min(text.len())].to_string();
    }

    format!("{}{}", text, " ".repeat(width - text_len))
}
```

### 9. `utils.rs` - Helper Functions

**Ответственность**:
- Общие утилиты
- Unicode width helpers
- Строковые операции

**Полезные функции**:
```rust
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

pub fn most_common<T: Eq + Hash + Clone>(items: &[T]) -> Option<T> {
    use std::collections::HashMap;
    let mut counts: HashMap<&T, usize> = HashMap::new();

    for item in items {
        *counts.entry(item).or_insert(0) += 1;
    }

    counts.into_iter().max_by_key(|&(_, count)| count).map(|(k, _)| k.clone())
}

pub fn align_to_grid(value: usize, grid_size: usize) -> usize {
    ((value + grid_size - 1) / grid_size) * grid_size
}
```

## Алгоритм работы (Two-Pass)

### Phase 1: Analysis (Read-Only)

```rust
// 1. Parse input into AST
let diagram = parse(input)?;

// 2. Analyze grid structure
let metrics = analyze_grid(&diagram.lines)?;

// 3. Determine alignment points
let alignment = calculate_alignment(&diagram, &metrics);
```

**Важно**: В Phase 1 мы только читаем и анализируем, не модифицируем данные.

### Phase 2: Transformation (Apply Changes)

```rust
// 4. Apply fixes (in-place on cloned diagram)
let mut formatted = diagram.clone();

if options.fix_box_drawing {
    fix_box_drawing_symbols(&mut formatted)?;
}

if options.fix_whitespace {
    normalize_whitespace(&mut formatted, &metrics)?;
}

align_text_content(&mut formatted, &metrics, options)?;

// 5. Render to final string
let output = render(&formatted, &metrics);
```

**Важно**: В Phase 2 мы модифицируем клон диаграммы, оригинал остается неизменным.

## Обработка ошибок

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Invalid grid structure: {0}")]
    InvalidGrid(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
```

### Error Handling Strategy

- **Parser errors** - Возвращаются немедленно с контекстом
- **Grid errors** - Пытаемся восстановиться (fallback к дефолтным значениям)
- **IO errors** - Propagate up с подробным сообщением

## Performance Considerations

### 1. Memory Management

```rust
// BAD: Allocate for every line
for line in lines {
    let processed = process(line); // Allocation
}

// GOOD: Pre-allocate capacity
let mut result = String::with_capacity(estimated_size);
for line in lines {
    result.push_str(process(line));
}
```

### 2. Unicode Width Caching

```rust
// BAD: Recalculate width repeatedly
for i in 0..100 {
    let width = unicode_width::UnicodeWidthStr::width(text);
}

// GOOD: Cache width calculation
let width = unicode_width::UnicodeWidthStr::width(text);
for i in 0..100 {
    use_cached_width(width);
}
```

### 3. Byte-Level Operations for ASCII

```rust
// BAD: UTF-8 char iteration
for ch in text.chars() {
    if ch == b'-' {
        // ...
    }
}

// GOOD: Byte iteration (for ASCII-only)
for byte in text.as_bytes() {
    if *byte == b'-' {
        // ...
    }
}
```

## Тестирование

### Unit Tests

Каждый модуль тестируется независимо:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_box() {
        let input = "┌──┐\n│AB│\n└──┘";
        let diagram = parse(input).unwrap();
        assert_eq!(diagram.boxes.len(), 1);
        assert_eq!(diagram.boxes[0].width, 4);
    }

    #[test]
    fn test_fix_plus_to_cross() {
        assert_eq!(fix_char('+', 0, 0, &mock_diagram()).unwrap(), '┼');
    }
}
```

### Integration Tests

Полный цикл форматирования:

```rust
#[test]
fn test_format_architecture_diagram() {
    let input = include_str!("../tests/fixtures/architecture/bad.txt");
    let expected = include_str!("../tests/fixtures/architecture/good.txt");

    let output = format_ascii(input, &Options::default()).unwrap();
    assert_eq!(output, expected);
}
```

### Benchmarks

Performance testing:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_format_diagram(c: &mut Criterion) {
    let input = include_str!("../tests/fixtures/large_diagram.txt");
    let options = Options::default();

    c.bench_function("format_large_diagram", |b| {
        b.iter(|| format_ascii(black_box(input), black_box(&options)))
    });
}

criterion_group!(benches, bench_format_diagram);
criterion_main!(benches);
```

## Extensibility

### Adding New Diagram Types

1. Добавьте вариант в `DiagramType` enum
2. Реализуйте детекцию в `patterns.rs`
3. Добавьте специфичную логику форматирования (если нужна)

### Adding New Style Presets

1. Добавьте вариант в `Style` enum
2. Реализуйте специфичную логику в formatter
3. Добавьте тесты для нового стиля

## Dependencies Rationale

| Crate | Purpose | Why |
|-------|---------|-----|
| `clap` | CLI parsing | De facto standard for Rust CLIs |
| `thiserror` | Error types | Derive macros for errors |
| `unicode-display-width` | Width calc | Terminal display width (Unicode-aware) |

## Future Enhancements

### v2.0 Features

1. **Plugin System**
   - Loadable plugins for custom diagram types
   - Hook-based architecture for extensibility

2. **Color Support**
   - ANSI color codes for syntax highlighting
   - Theme configuration files

3. **Export Formats**
   - SVG renderer
   - PNG export via `cairo-rs`

4. **Interactive Mode**
   - Real-time preview
   - Undo/redo support

---

*Last updated: 2025-01-21*
