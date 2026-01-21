#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagramType {
    Flowchart,
    Tree,
    Architecture,
    Sequence,
    Table,
    Unknown,
}

pub fn detect_type(lines: &[String]) -> DiagramType {
    let has_arrows = lines.iter().any(|l| l.contains("->") || l.contains("=>") || l.contains('→'));
    let has_tree_indent = lines.iter().any(|l| l.starts_with("  ") && (l.contains("├") || l.contains("│")));
    let has_layers = lines.iter().any(|l| (l.contains("├") && l.contains("┤")) || (l.contains("┌") && l.contains("└")));
    let has_vertical_lines = lines.iter().any(|l| l.contains("│"));
    let has_borders = lines.iter().any(|l| l.contains("┌") || l.contains("│"));
    let has_tables = lines.iter().any(|l| l.contains("┬") || l.contains("┼"));

    if has_tree_indent && !has_arrows {
        return DiagramType::Tree;
    }

    if has_arrows {
        if has_borders || has_vertical_lines {
            return DiagramType::Flowchart;
        }
    }

    if has_layers && has_borders {
        return DiagramType::Architecture;
    }

    if has_vertical_lines && has_borders && !has_layers {
        return DiagramType::Sequence;
    }

    if has_borders && has_tables {
        return DiagramType::Table;
    }

    DiagramType::Unknown
}
