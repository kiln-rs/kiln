use super::{Document, Element, Node};

#[path = "browser_html_table/cell_grouping.rs"]
mod cell_grouping;

pub(super) fn insert_implied_sections(mut document: Document) -> Document {
    normalize_nodes(&mut document.children);
    document
}

fn normalize_nodes(nodes: &mut Vec<Node>) {
    for node in nodes {
        let Node::Element(element) = node else {
            continue;
        };
        normalize_nodes(&mut element.children);
        if element.tag == "table" {
            group_rows(element);
        }
    }
}

fn group_rows(table: &mut Element) {
    cell_grouping::wrap_stray_cells(&mut table.children);
    let mut normalized = Vec::with_capacity(table.children.len());
    let mut rows = Vec::new();
    for child in table.children.drain(..) {
        let is_row = matches!(&child, Node::Element(element) if element.tag == "tr");
        let is_space = matches!(&child, Node::Text(text) if text.trim().is_empty());
        if is_row || (!rows.is_empty() && is_space) {
            rows.push(child);
        } else {
            flush_rows(&mut normalized, &mut rows);
            normalized.push(child);
        }
    }
    flush_rows(&mut normalized, &mut rows);
    table.children = normalized;
}

fn flush_rows(output: &mut Vec<Node>, rows: &mut Vec<Node>) {
    if rows.is_empty() {
        return;
    }
    output.push(Node::Element(Element {
        tag: "tbody".into(),
        attrs: Default::default(),
        children: std::mem::take(rows),
    }));
}
