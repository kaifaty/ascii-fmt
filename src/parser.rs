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
