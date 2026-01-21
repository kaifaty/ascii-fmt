use crate::box_drawing::fix_box_drawing_symbols;
use crate::cli::Options;
use crate::error::Result;
use crate::grid::{analyze_grid, normalize_whitespace};
use crate::parser::parse;
use crate::text_align::align_text_content;

pub fn format_ascii(input: &str, options: &Options) -> Result<String> {
    let diagram = parse(input)?;

    let grid_metrics = analyze_grid(&diagram.lines)?;

    let mut formatted = diagram.clone();

    if options.fix_box_drawing {
        fix_box_drawing_symbols(&mut formatted)?;
    }

    if options.fix_whitespace {
        normalize_whitespace(&mut formatted, &grid_metrics)?;
    }

    align_text_content(&mut formatted, &grid_metrics, options.style)?;

    render(&formatted)
}

fn render(diagram: &crate::parser::ParsedDiagram) -> Result<String> {
    Ok(diagram.lines.join("\n"))
}
