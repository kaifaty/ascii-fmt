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
    let has_tree_symbols = lines.iter().any(|l| l.contains("├") || l.contains("└"));
    let has_tree_indent = {
        let indent_levels: std::collections::HashSet<usize> = lines
            .iter()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.len() - l.trim_start().len())
            .collect();
        indent_levels.len() >= 2
    };
    let has_vertical_lines = lines.iter().any(|l| l.contains("│"));
    let has_all_corners = lines.iter().any(|l| l.contains("┌")) &&
                        lines.iter().any(|l| l.contains("┐")) &&
                        lines.iter().any(|l| l.contains("└")) &&
                        lines.iter().any(|l| l.contains("┘"));
    let has_tables = lines.iter().any(|l| l.contains("┬") || l.contains("┼"));

    if has_tables && has_all_corners {
        return DiagramType::Table;
    }

    if has_tree_symbols || has_tree_indent {
        if !has_all_corners && !has_arrows {
            return DiagramType::Tree;
        }
    }

    if has_arrows && has_all_corners {
        return DiagramType::Flowchart;
    }

    if has_arrows && !has_tree_symbols && !has_tree_indent {
        return DiagramType::Flowchart;
    }

    if has_all_corners && !has_tables {
        return DiagramType::Architecture;
    }

    if has_vertical_lines && !has_tree_symbols {
        return DiagramType::Sequence;
    }

    DiagramType::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagram_type_equality() {
        assert_eq!(DiagramType::Flowchart, DiagramType::Flowchart);
        assert_eq!(DiagramType::Tree, DiagramType::Tree);
        assert_eq!(DiagramType::Architecture, DiagramType::Architecture);
        assert_eq!(DiagramType::Sequence, DiagramType::Sequence);
        assert_eq!(DiagramType::Table, DiagramType::Table);
        assert_eq!(DiagramType::Unknown, DiagramType::Unknown);
    }

    #[test]
    fn test_diagram_type_inequality() {
        assert_ne!(DiagramType::Flowchart, DiagramType::Tree);
        assert_ne!(DiagramType::Architecture, DiagramType::Sequence);
        assert_ne!(DiagramType::Table, DiagramType::Unknown);
    }

    #[test]
    fn test_detect_type_flowchart_with_arrows() {
        let lines = vec![
            "A -> B".to_string(),
            "B => C".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Flowchart);
    }

    #[test]
    fn test_detect_type_flowchart_with_unicode_arrow() {
        let lines = vec![
            "A → B".to_string(),
            "┌───┐".to_string(),
            "│ A │".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Flowchart);
    }

    #[test]
    fn test_detect_type_tree_with_indent() {
        let lines = vec![
            "root".to_string(),
            "  child1".to_string(),
            "    grandchild".to_string(),
            "  child2".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Tree);
    }

    #[test]
    fn test_detect_type_tree_with_tree_symbols() {
        let lines = vec![
            "root".to_string(),
            "  ├─ child1".to_string(),
            "  │   └─ grandchild".to_string(),
            "  └─ child2".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Tree);
    }

    #[test]
    fn test_detect_type_tree_precedence_over_arrows() {
        let lines = vec![
            "root".to_string(),
            "  ├─ child -> test".to_string(),
            "  └─ child2".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Tree);
    }

    #[test]
    fn test_detect_type_architecture() {
        let lines = vec![
            "┌─────────┐".to_string(),
            "│  Front  │".to_string(),
            "├─────────┤".to_string(),
            "│  Back   │".to_string(),
            "└─────────┘".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Architecture);
    }

    #[test]
    fn test_detect_type_architecture_with_layers() {
        let lines = vec![
            "┌─────┐".to_string(),
            "│ App │".to_string(),
            "└─────┘".to_string(),
            "   │".to_string(),
            "┌─────┐".to_string(),
            "│ DB  │".to_string(),
            "└─────┘".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Architecture);
    }

    #[test]
    fn test_detect_type_sequence() {
        let lines = vec![
            "User   │ System".to_string(),
            "───────┼───────".to_string(),
            "request│       ".to_string(),
            "       │response".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Sequence);
    }

    #[test]
    fn test_detect_type_table() {
        let lines = vec![
            "┌─────┬─────┐".to_string(),
            "│ A   │ B   │".to_string(),
            "├─────┼─────┤".to_string(),
            "│ C   │ D   │".to_string(),
            "└─────┴─────┘".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Table);
    }

    #[test]
    fn test_detect_type_table_with_cross() {
        let lines = vec![
            "┌─────┐".to_string(),
            "│ A   │".to_string(),
            "┼─────┤".to_string(),
            "│ B   │".to_string(),
            "└─────┘".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Table);
    }

    #[test]
    fn test_detect_type_unknown() {
        let lines = vec![
            "hello".to_string(),
            "world".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Unknown);
    }

    #[test]
    fn test_detect_type_empty() {
        let lines: Vec<String> = vec![];
        assert_eq!(detect_type(&lines), DiagramType::Unknown);
    }

    #[test]
    fn test_detect_type_mixed_diagram() {
        let lines = vec![
            "┌─────┐".to_string(),
            "│ A→B │".to_string(),
            "└─────┘".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Flowchart);
    }

    #[test]
    fn test_detect_type_flowchart_with_only_arrows() {
        let lines = vec![
            "A -> B".to_string(),
            "B -> C".to_string(),
            "C -> D".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Unknown);
    }

    #[test]
    fn test_detect_type_tree_with_vertical_lines() {
        let lines = vec![
            "root".to_string(),
            "  │".to_string(),
            "  ├─ child1".to_string(),
            "  │".to_string(),
            "  └─ child2".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Tree);
    }

    #[test]
    fn test_detect_type_architecture_detection() {
        let lines = vec![
            "┌─────┐".to_string(),
            "│ App │".to_string(),
            "├─────┤".to_string(),
            "│ DB  │".to_string(),
            "└─────┘".to_string(),
        ];
        assert!(matches!(detect_type(&lines), DiagramType::Architecture));
    }

    #[test]
    fn test_detect_type_tree_without_arrows() {
        let lines = vec![
            "root".to_string(),
            "  ├─ child1".to_string(),
            "  └─ child2".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Tree);
    }

    #[test]
    fn test_detect_type_flowchart_priority() {
        let lines = vec![
            "A -> B".to_string(),
            "┌─────┐".to_string(),
            "│  A  │".to_string(),
            "└─────┘".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Flowchart);
    }

    #[test]
    fn test_detect_type_sequence_with_borders() {
        let lines = vec![
            "┌───┬───┐".to_string(),
            "│ A │ B │".to_string(),
            "├───┼───┤".to_string(),
            "│   │   │".to_string(),
            "└───┴───┘".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Table);
    }

    #[test]
    fn test_detect_type_flowchart_with_borders() {
        let lines = vec![
            "┌───┐".to_string(),
            "│ A │ → ".to_string(),
            "└───┘".to_string(),
            "   ↓".to_string(),
            "┌───┐".to_string(),
            "│ B │".to_string(),
            "└───┘".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Flowchart);
    }

    #[test]
    fn test_detect_type_tree_detection_order() {
        let lines = vec![
            "root".to_string(),
            "  ├─ child".to_string(),
            "  │   └─ grandchild".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Tree);
    }

    #[test]
    fn test_detect_type_architecture_with_corners() {
        let lines = vec![
            "┌─────┐".to_string(),
            "│ A   │".to_string(),
            "└─────┘".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Unknown);
    }

    #[test]
    fn test_detect_type_sequence_without_layers() {
        let lines = vec![
            "│ A │ B │".to_string(),
            "│   │   │".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Sequence);
    }

    #[test]
    fn test_detect_type_mixed_indents() {
        let lines = vec![
            "root".to_string(),
            "  child1".to_string(),
            "    grandchild".to_string(),
            "  child2".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Tree);
    }

    #[test]
    fn test_detect_type_copy_and_clone() {
        let dt = DiagramType::Flowchart;
        let dt_copy = dt;
        assert_eq!(dt, dt_copy);
        let dt_clone = dt.clone();
        assert_eq!(dt, dt_clone);
    }

    #[test]
    fn test_detect_type_with_equals_arrow() {
        let lines = vec![
            "A => B".to_string(),
            "┌─────┐".to_string(),
            "│  A  │".to_string(),
            "└─────┘".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Flowchart);
    }

    #[test]
    fn test_detect_type_tree_without_arrows_but_with_indents() {
        let lines = vec![
            "root".to_string(),
            "  child1".to_string(),
            "    grandchild".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Tree);
    }

    #[test]
    fn test_detect_type_with_multiple_indicators() {
        let lines = vec![
            "┌─────┐".to_string(),
            "│ A→B │".to_string(),
            "├─────┤".to_string(),
            "│ C←D │".to_string(),
            "└─────┘".to_string(),
        ];
        assert_eq!(detect_type(&lines), DiagramType::Flowchart);
    }
}
