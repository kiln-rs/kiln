use super::{Element, Node};

/// Wrap runs of stray `<td>`/`<th>` children into implicit `<tr>` rows,
/// matching the foster-parenting browsers perform for cells authored
/// directly inside `<table>`.
pub(super) fn wrap_stray_cells(children: &mut Vec<Node>) {
    let mut normalized = Vec::with_capacity(children.len());
    let mut cells = Vec::new();
    for child in children.drain(..) {
        if is_cell(&child) || keeps_cell_run_alive(&child, &cells) {
            cells.push(child);
        } else {
            flush_cells(&mut normalized, &mut cells);
            normalized.push(child);
        }
    }
    flush_cells(&mut normalized, &mut cells);
    *children = normalized;
}

fn is_cell(node: &Node) -> bool {
    matches!(node, Node::Element(element) if matches!(element.tag.as_str(), "td" | "th"))
}

fn keeps_cell_run_alive(node: &Node, cells: &[Node]) -> bool {
    !cells.is_empty() && matches!(node, Node::Text(text) if text.trim().is_empty())
}

fn flush_cells(output: &mut Vec<Node>, cells: &mut Vec<Node>) {
    if cells.is_empty() {
        return;
    }
    output.push(Node::Element(Element {
        tag: "tr".into(),
        attrs: Default::default(),
        children: std::mem::take(cells),
    }));
}
