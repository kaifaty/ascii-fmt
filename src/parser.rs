use crate::error::{Error, Result};
use crate::patterns::{detect_type, DiagramType};

#[derive(Debug, Clone)]
pub struct ParsedDiagram {
    pub lines: Vec<String>,
    pub diagram_type: DiagramType,
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub from: Point,
    pub to: Point,
}

pub fn parse(input: &str) -> Result<ParsedDiagram> {
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    if lines.is_empty() {
        return Err(Error::InvalidInput("Empty input".to_string()));
    }

    let diagram_type = detect_type(&lines);

    Ok(ParsedDiagram {
        lines,
        diagram_type,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_default() {
        let rect = Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        assert_eq!(rect.x, 0);
        assert_eq!(rect.y, 0);
        assert_eq!(rect.width, 0);
        assert_eq!(rect.height, 0);
    }

    #[test]
    fn test_rect_copy_and_clone() {
        let rect = Rect {
            x: 10,
            y: 20,
            width: 30,
            height: 40,
        };

        let rect_copy = rect;
        assert_eq!(rect_copy.x, 10);
        assert_eq!(rect_copy.y, 20);
        assert_eq!(rect_copy.width, 30);
        assert_eq!(rect_copy.height, 40);

        let rect_clone = rect.clone();
        assert_eq!(rect_clone.x, 10);
        assert_eq!(rect_clone.y, 20);
    }

    #[test]
    fn test_point_default() {
        let point = Point { x: 0, y: 0 };
        assert_eq!(point.x, 0);
        assert_eq!(point.y, 0);
    }

    #[test]
    fn test_point_copy_and_clone() {
        let point = Point { x: 5, y: 10 };

        let point_copy = point;
        assert_eq!(point_copy.x, 5);
        assert_eq!(point_copy.y, 10);

        let point_clone = point.clone();
        assert_eq!(point_clone.x, 5);
        assert_eq!(point_clone.y, 10);
    }

    #[test]
    fn test_connection_default() {
        let conn = Connection {
            from: Point { x: 0, y: 0 },
            to: Point { x: 0, y: 0 },
        };
        assert_eq!(conn.from.x, 0);
        assert_eq!(conn.from.y, 0);
        assert_eq!(conn.to.x, 0);
        assert_eq!(conn.to.y, 0);
    }

    #[test]
    fn test_connection_clone() {
        let conn = Connection {
            from: Point { x: 5, y: 10 },
            to: Point { x: 15, y: 20 },
        };

        let conn_clone = conn.clone();
        assert_eq!(conn_clone.from.x, 5);
        assert_eq!(conn_clone.from.y, 10);
        assert_eq!(conn_clone.to.x, 15);
        assert_eq!(conn_clone.to.y, 20);
    }

    #[test]
    fn test_parse_simple_diagram() {
        let input = "┌─────┐
│ hello │
└─────┘";

        let result = parse(input).unwrap();
        assert_eq!(result.lines.len(), 3);
        assert_eq!(result.lines[0], "┌─────┐");
        assert_eq!(result.lines[1], "│ hello │");
        assert_eq!(result.lines[2], "└─────┘");
    }

    #[test]
    fn test_parse_empty_input() {
        let input = "";
        let result = parse(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_whitespace_only() {
        let input = "   \n   \n   ";
        let result = parse(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().lines.len(), 3);
    }

    #[test]
    fn test_parse_single_line() {
        let input = "hello world";
        let result = parse(input).unwrap();
        assert_eq!(result.lines.len(), 1);
        assert_eq!(result.lines[0], "hello world");
    }

    #[test]
    fn test_parse_multiple_lines() {
        let input = "line1\nline2\nline3\nline4";
        let result = parse(input).unwrap();
        assert_eq!(result.lines.len(), 4);
        assert_eq!(result.lines[0], "line1");
        assert_eq!(result.lines[1], "line2");
        assert_eq!(result.lines[2], "line3");
        assert_eq!(result.lines[3], "line4");
    }

    #[test]
    fn test_parse_with_empty_lines() {
        let input = "line1\n\nline3\n\nline5";
        let result = parse(input).unwrap();
        assert_eq!(result.lines.len(), 5);
        assert_eq!(result.lines[0], "line1");
        assert_eq!(result.lines[1], "");
        assert_eq!(result.lines[2], "line3");
        assert_eq!(result.lines[3], "");
        assert_eq!(result.lines[4], "line5");
    }

    #[test]
    fn test_parse_preserves_whitespace() {
        let input = "  line  \n  line  ";
        let result = parse(input).unwrap();
        assert_eq!(result.lines[0], "  line  ");
        assert_eq!(result.lines[1], "  line  ");
    }

    #[test]
    fn test_parse_unicode_content() {
        let input = "你好\n世界\n测试";
        let result = parse(input).unwrap();
        assert_eq!(result.lines.len(), 3);
        assert_eq!(result.lines[0], "你好");
        assert_eq!(result.lines[1], "世界");
        assert_eq!(result.lines[2], "测试");
    }

    #[test]
    fn test_parse_box_drawing() {
        let input = "┌─┐\n│ │\n└─┘";
        let result = parse(input).unwrap();
        assert_eq!(result.lines.len(), 3);
        assert_eq!(result.lines[0], "┌─┐");
        assert_eq!(result.lines[1], "│ │");
        assert_eq!(result.lines[2], "└─┘");
    }

    #[test]
    fn test_parsed_diagram_clone() {
        let input = "┌─────┐\n│ hello │\n└─────┘";
        let diagram = parse(input).unwrap();
        let diagram_clone = diagram.clone();

        assert_eq!(diagram_clone.lines.len(), 3);
        assert_eq!(diagram_clone.lines[0], "┌─────┐");
    }

    #[test]
    fn test_parse_diagram_type() {
        let input = "┌─────┐\n│ hello │\n└─────┘";
        let result = parse(input).unwrap();
        assert_eq!(result.diagram_type, crate::patterns::DiagramType::Table);
    }

    #[test]
    fn test_parse_mixed_newlines() {
        let input = "line1\r\nline2\nline3\r\nline4";
        let result = parse(input).unwrap();
        assert_eq!(result.lines.len(), 4);
    }

    #[test]
    fn test_parse_trailing_newline() {
        let input = "line1\nline2\n";
        let result = parse(input).unwrap();
        assert_eq!(result.lines.len(), 2);
    }

    #[test]
    fn test_parse_special_characters() {
        let input = "hello\tworld\ntest\tdata";
        let result = parse(input).unwrap();
        assert_eq!(result.lines.len(), 2);
        assert_eq!(result.lines[0], "hello\tworld");
        assert_eq!(result.lines[1], "test\tdata");
    }

    #[test]
    fn test_parse_empty_string_returns_error() {
        let input = "";
        let result = parse(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn test_parse_invalid_input_error_message() {
        let input = "";
        let result = parse(input);
        match result {
            Err(Error::InvalidInput(msg)) => {
                assert_eq!(msg, "Empty input");
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_rect_with_large_values() {
        let rect = Rect {
            x: 500,
            y: 1000,
            width: 0,
            height: 0,
        };
        assert_eq!(rect.x, 500);
        assert_eq!(rect.y, 1000);
    }

    #[test]
    fn test_point_with_large_values() {
        let point = Point { x: 100, y: 200 };
        assert_eq!(point.x, 100);
        assert_eq!(point.y, 200);
    }

    #[test]
    fn test_connection_with_large_points() {
        let conn = Connection {
            from: Point { x: 50, y: 100 },
            to: Point { x: 150, y: 200 },
        };
        assert_eq!(conn.from.x, 50);
        assert_eq!(conn.to.y, 200);
    }
}
